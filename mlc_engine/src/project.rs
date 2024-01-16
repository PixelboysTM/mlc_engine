use std::{collections::HashMap, path::PathBuf, sync::Arc};

use chrono::{DateTime, Local};
use rocket::{futures::lock::Mutex, tokio::sync::broadcast::Sender, State};

use crate::{
    data_serving::Info,
    fixture::{FixtureType, FixtureUniverse, UniverseId},
    runtime::RuntimeData,
    send,
    settings::ProjectDefinition,
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ProjectI {
    // Common with general Information
    name: String,
    #[serde(skip)]
    file_name: String,
    last_edited: DateTime<Local>,

    // Other stuff
    fixtures: Vec<FixtureType>,
    universes: HashMap<UniverseId, FixtureUniverse>,
    settings: Settings,
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

        data.last_edited = Local::now();

        // let toml_data = toml::to_string(data).map_err(|_| "Failed serializing data")?;
        let json_data =
            serde_json::to_string_pretty(data).map_err(|_| "Failed serializing data")?;
        if let Some(path) = make_path(name.unwrap_or(&data.file_name)) {
            std::fs::write(path, json_data).map_err(|_| "Failed writing to file")?;
        } else {
            Err("Failed creating path")?;
        }

        send!(info, Info::ProjectSaved);

        Ok(())
    }
    pub async fn load(
        &self,
        name: &str,
        info: &Sender<Info>,
        runtime: &RuntimeData,
    ) -> Result<(), &str> {
        if let Some(path) = make_path(name) {
            if let Ok(json_data) = std::fs::read_to_string(path) {
                let new_data: ProjectI =
                    // toml::from_str(&toml_data).map_err(|_| "Failed deserializing data")?;
                    serde_json::from_str(&json_data).map_err(|e| { eprintln!("{:#?}", e); "Failed deserializing data"})?;
                let mut data = self.project.lock().await;
                *data = new_data;
                data.file_name = name.to_string();
            } else {
                Err("Failed reading file")?;
            }
        } else {
            Err("Failed creating path")?;
        }

        runtime.adapt(self, true).await;
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

    pub async fn get_universe(&self, id: &UniverseId) -> Result<FixtureUniverse, &str> {
        let data = self.project.lock().await;
        data.universes
            .get(id)
            .ok_or("Universe Id not found")
            .map(|s| s.clone())
    }

    pub async fn try_patch(
        &self,
        fixture: &FixtureType,
        mode_index: usize,
        create_new_universe: bool,
        info: &Sender<Info>,
        runtime: &RuntimeData,
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
            runtime.adapt(self, false).await;

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

    pub async fn get_settings(&self) -> Settings {
        let data = self.project.lock().await;
        data.settings.clone()
    }

    pub async fn update_settings(&self, settings: Settings) -> Result<(), &'static str> {
        let mut data = self.project.lock().await;
        data.settings = settings;

        Ok(())
    }

    pub async fn get_definition(&self) -> ProjectDefinition {
        let data = self.project.lock().await;
        ProjectDefinition {
            file_name: data.file_name.clone(),
            last_edited: data.last_edited.clone(),
            name: data.name.clone(),
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        let mut s = HashMap::new();
        s.insert(UniverseId(1), FixtureUniverse::empty(UniverseId(1)));

        Self {
            project: Arc::new(Mutex::new(ProjectI {
                name: "unnamed".to_string(),
                file_name: "unnamed".to_string(),
                last_edited: DateTime::default(),
                fixtures: Vec::new(),
                universes: s,
                settings: Settings {
                    save_on_quit: false,
                },
            })),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Settings {
    save_on_quit: bool,
}

impl Settings {
    pub fn save_on_quit(&self) -> bool {
        self.save_on_quit
    }
}

fn get_project_dirs() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("de", "pixelboystm", "mlc_engine")
}

pub fn make_path(name: &str) -> Option<PathBuf> {
    get_project_dirs().map(|d| {
        let dir = d.data_dir();
        std::fs::create_dir_all(dir).unwrap();
        dir.join(format!("{}.mlc", name))
    })
}
