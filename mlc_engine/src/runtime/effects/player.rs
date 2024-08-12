use chrono::Duration;
use mlc_common::effect::EffectId;
use rocket::{
    futures::{
        channel::mpsc::{self, Receiver, Sender},
        lock::MutexGuard,
        SinkExt, StreamExt,
    },
    tokio::{
        select,
        sync::broadcast::{self, Receiver as BReceiver, Sender as BSender},
        time::{interval, Interval},
    },
};
use std::{collections::HashMap, future::IntoFuture};

use crate::{
    project::{ProjectHandle, ProjectI},
    runtime::RuntimeData,
};

use super::baking::{self, BakedEffect, BakedFixtureData, EffectBaker};

pub struct EffectPlayerHandle {
    pub cmd_sender: Sender<EffectPlayerCmd>,
    pub update_receiver: BReceiver<EffectPlayerUpdate>,
}

impl Clone for EffectPlayerHandle {
    fn clone(&self) -> Self {
        Self {
            cmd_sender: self.cmd_sender.clone(),
            update_receiver: self.update_receiver.resubscribe(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectPlayerCmd {
    Play { id: EffectId },
    Stop { id: EffectId },
    EffectChanged { id: EffectId },
    EffectsChanged,
    StopPlayer,
    GetPlayingEffects,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EffectPlayerUpdate {
    PlayingEffects(Vec<EffectId>),
}

struct EffectPlayer {
    project: ProjectHandle,
    baking_map: HashMap<EffectId, BakingStatus>,
    baked_effects: HashMap<EffectId, BakedEffect>,
    playing_effects: HashMap<EffectId, Duration>,
    update_freq: Interval,
    cmd_receiver: Receiver<EffectPlayerCmd>,
    update_sender: BSender<EffectPlayerUpdate>,
    effect_baker: EffectBaker,
    time: chrono::NaiveTime,
    runtime: RuntimeData,
}

pub async fn startup_effect_player(
    project: ProjectHandle,
    runtime: RuntimeData,
) -> EffectPlayerHandle {
    let (cmd_sender, cmd_receiver) = mpsc::channel::<EffectPlayerCmd>(1024);
    let (update_sender, update_receiver) = broadcast::channel::<EffectPlayerUpdate>(1024);

    let baker = async {
        let p = project.lock().await;
        baking::startup_effect_baker(get_patched_fixtures_clone(&p))
    }
    .await;

    let player = EffectPlayer {
        project,
        baked_effects: HashMap::new(),
        baking_map: HashMap::new(),
        playing_effects: HashMap::new(),
        cmd_receiver,
        update_sender,
        effect_baker: baker,
        runtime,
        time: chrono::Utc::now().naive_utc().time(),
        update_freq: interval(std::time::Duration::from_millis(20)), //TODO: Make available in settings
    };
    rocket::tokio::task::spawn(player.run());
    EffectPlayerHandle {
        cmd_sender,
        update_receiver,
    }
}

impl EffectPlayer {
    async fn run(mut self) {
        self.sync_baking_map().await;

        let mut should_exit = false;
        while !should_exit {
            select! {
                _ = self.update_freq.tick() => {
                    self.tick().await;
                }
                cmd = self.cmd_receiver.next() => {
                    if let Some(cmd) = cmd {
                        self.handle_cmd(cmd, &mut should_exit).await;
                    } else {
                        should_exit = true;
                        println!("Exiting effect player");
                    }
                }
                baked_effect = self.effect_baker.effect_recv.next() => {
                    if let Some(baked_effect) = baked_effect {
                        self.handle_baked_effect(baked_effect).await;
                    } else {
                        should_exit = true;
                        println!("Exiting effect player");
                    }
                }
            }
        }

        let _ = self
            .effect_baker
            .task_sender
            .send(baking::BakingRequest::Shutdown)
            .await;
        let _ = self.effect_baker.join_handle.into_future().await;
    }

    async fn tick(&mut self) {
        let now = chrono::Utc::now().naive_utc().time();
        let elapsed = now - self.time;
        self.time = now;

        let mut value_map = HashMap::new();

        let mut marked_for_stopping = vec![];

        for (id, time) in &mut self.playing_effects {
            let status = self.baking_map.get(id);
            match status {
                Some(BakingStatus::Baked) => {}
                Some(BakingStatus::InProgress) => {
                    if !self.baked_effects.contains_key(id) {
                        println!("Waiting for effect to finish Baking");
                        continue;
                    }
                }
                Some(BakingStatus::Changed) | Some(BakingStatus::Unbaked) => {
                    println!("Requesting baking...............");
                    let effect = self
                        .project
                        .lock()
                        .await
                        .effects
                        .iter()
                        .find(|e| e.id == *id)
                        .cloned()
                        .unwrap();
                    let _ = self
                        .effect_baker
                        .task_sender
                        .send(baking::BakingRequest::Bake(effect))
                        .await;
                    self.baking_map.insert(*id, BakingStatus::InProgress);
                    continue;
                }
                None => {
                    eprintln!("Why has effect no status?");
                    continue;
                }
            }

            let effect = self.baked_effects.get(id).unwrap();

            *time += elapsed;

            if *time > effect.max_time {
                if effect.looping {
                    while *time > effect.max_time {
                        *time -= effect.max_time;
                    }
                } else {
                    marked_for_stopping.push(*id);
                    continue;
                }
            }

            for f in &effect.faders {
                let mut value = 0;
                for (d, v) in f.1.iter() {
                    if &*time > d {
                        value = *v;
                    }
                }
                value_map.insert(*f.0, value);
            }
        }

        if !marked_for_stopping.is_empty() {
            for to_stop in marked_for_stopping {
                self.playing_effects.remove(&to_stop);
            }
            let _ = self.update_sender.send(EffectPlayerUpdate::PlayingEffects(
                self.playing_effects.keys().cloned().collect::<Vec<_>>(),
            ));
        }

        let mut universes = vec![];
        let mut channels = vec![];
        let mut values = vec![];

        for (k, v) in value_map {
            universes.push(k.universe);
            channels.push(k.address);
            values.push(v);
        }

        if !universes.is_empty() {
            self.runtime.set_values(universes, channels, values).await;
        }
    }

    async fn handle_baked_effect(&mut self, (id, baked): (EffectId, BakedEffect)) {
        self.baked_effects.insert(id, baked);
        if let Some(status) = self.baking_map.get_mut(&id) {
            if *status != BakingStatus::Changed {
                *status = BakingStatus::Baked;
            }
        }
    }

    async fn handle_cmd(&mut self, cmd: EffectPlayerCmd, should_exit: &mut bool) {
        match cmd {
            EffectPlayerCmd::Play { id } => {
                self.playing_effects
                    .entry(id)
                    .or_insert_with(Duration::zero);
                let _ = self.update_sender.send(EffectPlayerUpdate::PlayingEffects(
                    self.playing_effects.keys().cloned().collect::<Vec<_>>(),
                ));
            }
            EffectPlayerCmd::Stop { id } => {
                self.playing_effects.remove(&id);
                let _ = self.update_sender.send(EffectPlayerUpdate::PlayingEffects(
                    self.playing_effects.keys().cloned().collect::<Vec<_>>(),
                ));
            }
            EffectPlayerCmd::EffectChanged { id } => {
                if let Some(status) = self.baking_map.get_mut(&id) {
                    if *status == BakingStatus::InProgress {
                        let effect = self
                            .project
                            .lock()
                            .await
                            .effects
                            .iter()
                            .find(|e| e.id == id)
                            .cloned();
                        if let Some(effect) = effect {
                            let _ = self
                                .effect_baker
                                .task_sender
                                .send(baking::BakingRequest::Bake(effect))
                                .await;
                        } else {
                            eprintln!("Why is the effect not present?");
                        }
                    } else {
                        *status = BakingStatus::Changed;
                    }
                } else {
                    self.baking_map.insert(id, BakingStatus::Changed);
                }
            }
            EffectPlayerCmd::EffectsChanged => {
                self.sync_baking_map().await;
                self.playing_effects.clear();
            }
            EffectPlayerCmd::StopPlayer => *should_exit = true,
            EffectPlayerCmd::GetPlayingEffects => {
                let _ = self.update_sender.send(EffectPlayerUpdate::PlayingEffects(
                    self.playing_effects.keys().cloned().collect::<Vec<_>>(),
                ));
            }
        }
    }

    async fn sync_baking_map(&mut self) {
        let p = self.project.lock().await;
        let mut new_map = HashMap::new();
        for effect in &p.effects {
            if let Some(status) = self.baking_map.get(&effect.id) {
                new_map.insert(effect.id, *status);
            } else {
                new_map.insert(effect.id, BakingStatus::Unbaked);
            }
        }

        let _ = self
            .effect_baker
            .task_sender
            .send(baking::BakingRequest::Fixtures(
                p.universes
                    .iter()
                    .flat_map(|u| u.1.fixtures.clone())
                    .collect::<Vec<_>>(),
            ))
            .await;

        self.baking_map = new_map;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BakingStatus {
    Unbaked,
    Changed,
    InProgress,
    Baked,
}

fn get_patched_fixtures_clone(p: &MutexGuard<ProjectI>) -> BakedFixtureData {
    use get_size::GetSize;
    let patched_fixtures: Vec<_> = p
        .universes
        .values()
        .flat_map(|u| u.fixtures.clone())
        .collect();
    println!(
        "Debug: Patched fixture clone size for baking {} bytes",
        patched_fixtures.get_heap_size()
    );
    patched_fixtures
}
