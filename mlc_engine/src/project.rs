use std::{path::PathBuf, sync::Arc};

use rocket::futures::lock::Mutex;

use crate::{
    data_serving::{Info, InfoTx},
    data_spreader::DataSender,
    fixture::FixtureType,
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ProjectI {
    name: String,
    fixtures: Vec<FixtureType>,
}

pub struct Project {
    project: Arc<Mutex<ProjectI>>,
}

impl Project {
    #[allow(unused)]
    pub async fn save(&self, info: &DataSender<Info>) -> Result<(), &str> {
        self.save_(None, info).await
    }

    pub async fn save_as(&self, name: &str, info: &DataSender<Info>) -> Result<(), &str> {
        self.save_(Some(name), info).await
    }

    async fn save_(&self, name: Option<&str>, info: &DataSender<Info>) -> Result<(), &str> {
        let data: &mut ProjectI = &mut *self.project.lock().await;
        if let Some(new_name) = name {
            data.name = new_name.to_string();
        }

        let toml_data = toml::to_string_pretty(data).map_err(|_| "Failed serializing data")?;
        if let Some(path) = make_path(name.unwrap_or(&data.name)) {
            std::fs::write(path, toml_data).map_err(|_| "Failed writing to file")?;
        } else {
            Err("Failed creating path")?;
        }

        info.send(Info::ProjectSaved);

        Ok(())
    }
    pub async fn load(&self, name: &str, info: &DataSender<Info>) -> Result<(), &str> {
        if let Some(path) = make_path(name) {
            if let Ok(toml_data) = std::fs::read_to_string(path) {
                let new_data: ProjectI =
                    toml::from_str(&toml_data).map_err(|_| "Failed deserializing data")?;
                let mut data = self.project.lock().await;
                *data = new_data
            } else {
                Err("Failed reading file")?;
            }
        } else {
            Err("Failed creating path")?;
        }

        info.send(Info::ProjectLoaded);

        Ok(())
    }

    pub async fn insert_fixture(&self, fixture: FixtureType, info: &DataSender<Info>) {
        let mut data = self.project.lock().await;
        if !data.fixtures.contains(&fixture) {
            data.fixtures.push(fixture);
            info.send(Info::FixtureTypesUpdated);
        }
    }

    pub async fn get_fixtures(&self) -> Vec<FixtureType> {
        let data = self.project.lock().await;
        data.fixtures.clone()
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            project: Arc::new(Mutex::new(ProjectI {
                name: "unnamed".to_string(),
                fixtures: Vec::new(),
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
