use std::{collections::HashMap, path::PathBuf, sync::Arc};

use chrono::{DateTime, Local};
use rocket::{
    futures::lock::{Mutex, MutexGuard},
    tokio::sync::broadcast::Sender,
};

use mlc_common::{Info, ProjectDefinition, ProjectSettings};
use mlc_common::config::FixtureType;
use mlc_common::endpoints::EndPointConfig;
use mlc_common::patched::UniverseId;
use mlc_common::universe::FixtureUniverse;

use crate::{
    runtime::{
        effects::{Effect, EffectPlayerAction},
        RuntimeData,
    },
    send,
};
use crate::fixture::universe as u;
pub use crate::project::byte_provider::{Provider};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct ProjectI {
    // Common with general Information
    pub(crate) name: String,
    #[serde(skip)]
    pub(crate) file_name: String,
    pub(crate) last_edited: DateTime<Local>,

    // Other stuff
    pub(crate) fixtures: Vec<FixtureType>,
    pub(crate) universes: HashMap<UniverseId, FixtureUniverse>,

    //Effects
    pub(crate) effects: Vec<Effect>,

    pub(crate) settings: ProjectSettings,

    #[serde(default)]
    pub(crate) endpoints: EndPointConfig,
}

#[derive(Debug, Clone)]
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

        let provider: Provider = if cfg!(debug_assertions) { Provider::Json } else { Provider::Ciborium };
        //
        // let json_data =
        //     serde_json::to_string_pretty(data).map_err(|_| "Failed serializing data")?;
        let raw_data = provider.to(data);
        if let Some(path) = make_path(name.unwrap_or(&data.file_name), provider.extension()) {
            std::fs::write(path, raw_data).map_err(|_| "Failed writing to file")?;
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
        effect_handler: &Sender<EffectPlayerAction>,
    ) -> Result<(), &str> {
        let possible_loaders: Vec<(&str, Provider)> = Provider::valid_extensions();

        let mut success = false;
        for (ext, possible_loader) in possible_loaders {
            let path = make_path(name, ext).ok_or("Path creation failed")?;
            if let Ok(raw_data) = std::fs::read(path) {
                let new_data: ProjectI = possible_loader.from(&raw_data).map_err(|e| {
                    eprintln!("{e}");
                    "Failed deserializing data"
                })?;
                let mut data = self.project.lock().await;
                *data = new_data;
                data.file_name = name.to_string();

                success = true;
                break;
            }
        }

        if !success {
            Err("Failed loading data")?;
        }

        // if let Some(path) = make_path(name) {
        //     if let Ok(json_data) = std::fs::read_to_string(path) {
        //         let new_data: ProjectI =
        //             // toml::from_str(&toml_data).map_err(|_| "Failed deserializing data")?;
        //             serde_json::from_str(&json_data).map_err(|e| {
        //                 eprintln!("{:#?}", e);
        //                 "Failed deserializing data"
        //             })?;
        //         let mut data = self.project.lock().await;
        //         *data = new_data;
        //         data.file_name = name.to_string();
        //     } else {
        //         Err("Failed reading file")?;
        //     }
        // } else {
        //     Err("Failed creating path")?;
        // }

        runtime.adapt(self, true).await;
        send!(effect_handler, EffectPlayerAction::Rebake);
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
        if u::can_patch(universe, &fixture, mode_index) {
            u::patch(universe, &fixture, mode_index).unwrap();
            send!(info, Info::UniversePatchChanged(universe_id));
            Some(())
        } else {
            None
        }
    }

    pub async fn get_settings(&self) -> ProjectSettings {
        let data = self.project.lock().await;
        data.settings.clone()
    }

    pub async fn update_settings(&self, settings: ProjectSettings) -> Result<(), &'static str> {
        let mut data = self.project.lock().await;
        data.settings = settings;

        Ok(())
    }

    pub async fn get_definition(&self) -> ProjectDefinition {
        let data = self.project.lock().await;
        ProjectDefinition {
            file_name: data.file_name.clone(),
            last_edited: data.last_edited,
            name: data.name.clone(),
        }
    }

    pub async fn get_endpoint_config(&self) -> EndPointConfig {
        let data = self.project.lock().await;
        data.endpoints.clone()
    }
    pub async fn set_endpoint_config(&self, config: EndPointConfig) {
        let mut data = self.project.lock().await;
        data.endpoints = config;
    }

    pub async fn lock(&self) -> MutexGuard<'_, ProjectI> {
        self.project.lock().await
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
                settings: ProjectSettings {
                    save_on_quit: false,
                },
                endpoints: EndPointConfig::default(),
                effects: Vec::new(),
            })),
        }
    }
}

fn get_project_dirs() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("de", "pixelboystm", "mlc_engine")
}

pub fn make_path(name: &str, extension: &str) -> Option<PathBuf> {
    get_project_dirs().map(|d| {
        let dir = d.data_dir();
        std::fs::create_dir_all(dir).unwrap();
        dir.join(format!("{}.{}", name, extension))
    })
}

mod byte_provider {
    use rocket::http::hyper::body::Buf;
    use mlc_common::ProjectDefinition;
    use crate::project::ProjectI;

    pub enum Provider {
        Json,
        Ciborium,
    }

    impl Provider {
        pub fn valid_extensions() -> Vec<(&'static str, Provider)> {
            vec![("mlc", Provider::Json), ("mlcb", Provider::Ciborium)]
        }

        pub fn extension(&self) -> &'static str {
            match self {
                Provider::Json => "mlc",
                Provider::Ciborium => "mlcb",
            }
        }

        pub fn from(&self, b: &Vec<u8>) -> Result<ProjectI, String> {
            match self {
                Provider::Json => {
                    serde_json::from_slice(b).map_err(|e| format!("{e:?}"))
                }
                Provider::Ciborium => {
                    ciborium::from_reader(b.reader()).map_err(|e| format!("{e:?}"))
                }
            }
        }

        pub fn definition(&self, b: &Vec<u8>) -> Result<ProjectDefinition, String> {
            match self {
                Provider::Json => {
                    serde_json::from_slice(b).map_err(|e| format!("{e:?}"))
                }
                Provider::Ciborium => {
                    ciborium::from_reader(b.reader()).map_err(|e| format!("{e:?}"))
                }
            }
        }

        pub fn to(&self, p: &ProjectI) -> Vec<u8> {
            match self {
                Provider::Json => {
                    serde_json::to_vec_pretty(p).expect("Why?")
                }
                Provider::Ciborium => {
                    let mut b = Vec::<u8>::new();
                    ciborium::into_writer(p, &mut b).expect("Why");
                    b
                }
            }
        }
    }
}
