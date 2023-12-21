use std::{collections::HashMap, path::PathBuf, sync::Arc};

use rocket::{futures::lock::Mutex, tokio::sync::broadcast::Sender};

use crate::{
    data_serving::Info,
    fixture::{FixtureType, FixtureUniverse, UniverseId},
    send,
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ProjectI {
    name: String,
    fixtures: Vec<FixtureType>,
    universes: HashMap<UniverseId, FixtureUniverse>,
}

pub struct Project {
    project: Arc<Mutex<ProjectI>>,
}

impl Project {
    #[allow(unused)]
    pub async fn save(&self, info: &Sender<Info>) -> Result<(), &'static str> {
        self.save_(None, info).await
    }

    pub async fn save_as(&self, name: &str, info: &Sender<Info>) -> Result<(), &'static str> {
        self.save_(Some(name), info).await
    }

    async fn save_(&self, name: Option<&str>, info: &Sender<Info>) -> Result<(), &'static str> {
        let data: &mut ProjectI = &mut *self.project.lock().await;
        if let Some(new_name) = name {
            data.name = new_name.to_string();
        }

        // let toml_data = toml::to_string(data).map_err(|_| "Failed serializing data")?;
        let json_data =
            serde_json::to_string_pretty(data).map_err(|_| "Failed serializing data")?;
        if let Some(path) = make_path(name.unwrap_or(&data.name)) {
            std::fs::write(path, json_data).map_err(|_| "Failed writing to file")?;
        } else {
            Err("Failed creating path")?;
        }

        send!(info, Info::ProjectSaved);

        Ok(())
    }
    pub async fn load(&self, name: &str, info: &Sender<Info>) -> Result<(), &str> {
        if let Some(path) = make_path(name) {
            if let Ok(json_data) = std::fs::read_to_string(path) {
                let new_data: ProjectI =
                    // toml::from_str(&toml_data).map_err(|_| "Failed deserializing data")?;
                    serde_json::from_str(&json_data).map_err(|_| "Failed deserializing data")?;
                let mut data = self.project.lock().await;
                *data = new_data
            } else {
                Err("Failed reading file")?;
            }
        } else {
            Err("Failed creating path")?;
        }

        send!(info, Info::ProjectLoaded);

        Ok(())
    }

    pub async fn insert_fixture(&self, fixture: FixtureType, info: &Sender<Info>) {
        let mut data = self.project.lock().await;
        if !data.fixtures.contains(&fixture) {
            data.fixtures.push(fixture);
            send!(info, Info::FixtureTypesUpdated);
        }
    }

    pub async fn get_fixtures(&self) -> Vec<FixtureType> {
        let data = self.project.lock().await;
        data.fixtures.clone()
    }

    pub async fn get_universes(&self) -> Vec<UniverseId> {
        let data = self.project.lock().await;
        data.universes.keys().copied().collect()
    }

    pub async fn try_patch(
        &self,
        fixture: &FixtureType,
        mode_index: usize,
        create_new_universe: bool,
        info: &Sender<Info>,
    ) -> Option<()> {
        let data: Vec<_> = {
            let d = self
                .project
                .lock()
                .await
                .universes
                .keys()
                .copied()
                .collect();
            d
        };
        for id in data {
            if self
                .try_patch_to_universe(fixture.clone(), mode_index, id, info)
                .await
                .is_some()
            {
                return Some(());
            }
        }

        if create_new_universe {
            let new_id = {
                let mut data = self.project.lock().await;
                let id = data
                    .universes
                    .keys()
                    .max_by(|a, b| a.0.cmp(&b.0))
                    .map(|f| UniverseId(f.0 + 1))
                    .unwrap_or(UniverseId(1));
                data.universes.insert(id, FixtureUniverse::empty(id));
                id
            };

            send!(info, Info::UniversesUpdated);

            return self
                .try_patch_to_universe(fixture.clone(), mode_index, new_id, info)
                .await;
        }

        None
    }

    pub async fn try_patch_to_universe(
        &self,
        fixture: FixtureType,
        mode_index: usize,
        universe_id: UniverseId,
        info: &Sender<Info>,
    ) -> Option<()> {
        let mut data = self.project.lock().await;
        let universe = data.universes.get_mut(&universe_id)?;
        if universe.can_patch(&fixture, mode_index) {
            universe.patch(&fixture, mode_index).unwrap();
            send!(info, Info::UniversePatchChanged(universe_id));
            Some(())
        } else {
            None
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            project: Arc::new(Mutex::new(ProjectI {
                name: "unnamed".to_string(),
                fixtures: Vec::new(),
                universes: HashMap::new(),
            })),
        }
    }
}

fn get_project_dirs() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("de", "pixelboystm", "mlc_engine")
}

fn make_path(name: &str) -> Option<PathBuf> {
    get_project_dirs().map(|d| {
        let dir = d.data_dir();
        std::fs::create_dir_all(dir).unwrap();
        dir.join(format!("{}.mlc", name))
    })
}
