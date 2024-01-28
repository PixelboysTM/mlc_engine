use chrono::Duration;
use rocket::futures::{SinkExt, StreamExt};
use rocket::tokio::select;
use rocket::{get, routes, Shutdown, State};
use rocket_ws::stream::DuplexStream;
use rocket_ws::WebSocket;
use serde_with::serde_as;
use serde_with::{formats::Flexible, DurationSecondsWithFrac};

use crate::project::Project;
use crate::{fixture::FaderAddress, module::Module};

use super::RuntimeData;

pub struct EffectModule;

impl Module for EffectModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        app.mount("/effects", routes![get_effect_handler])
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Effect {
    id: uuid::Uuid,
    name: String,
    tracks: Vec<CueTrack>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum CueTrack {
    FaderCue(FaderCue),
}

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FaderCue {
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    duration: Duration,
    address: FaderAddress,
    values: Vec<FaderKey>,
}

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FaderKey {
    value: u8,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    start_time: Duration,
}

#[derive(Debug, serde::Serialize)]
pub enum EffectHandlerResponse {
    EffectCreated { name: String, id: uuid::Uuid },
    EffectUpdated { id: uuid::Uuid },
}

#[derive(Debug, serde::Deserialize)]
pub enum EffectHandlerRequest {
    Create {
        name: String,
    },
    Update {
        id: uuid::Uuid,
        tracks: Vec<CueTrack>,
    },
}

#[get("/effectHandler")]
fn get_effect_handler<'a>(
    ws: WebSocket,
    mut shutdown: Shutdown,
    runtime: &'a State<RuntimeData>,
    project: &'a State<Project>,
) -> rocket_ws::Channel<'a> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    Some(msg) = stream.next() => {
                        if let Ok(msg) = msg {
                            let req: EffectHandlerRequest = serde_json::from_str(msg.to_text().unwrap()).expect("Must be");
                            handle_msg(&mut stream, req, runtime, project).await;
                        }
                    }

                    _ = &mut shutdown => {
                        break;
                    }
                }
            }
            Ok(())
        })
    })
}

async fn handle_msg(
    stream: &mut DuplexStream,
    req: EffectHandlerRequest,
    runtime: &State<RuntimeData>,
    project: &State<Project>,
) {
    match req {
        EffectHandlerRequest::Create { name } => {
            let mut p = project.lock().await;
            let id = uuid::Uuid::new_v4();
            p.effects.push(Effect {
                id,
                name: name.clone(),
                tracks: vec![],
            });
            let _ = stream
                .send(make_msg(&EffectHandlerResponse::EffectCreated { name, id }))
                .await;
        }
        EffectHandlerRequest::Update { id, tracks } => {
            let mut p = project.lock().await;
            let effect = p.effects.iter_mut().find(|f| f.id == id);
            if let Some(effect) = effect {
                effect.tracks = tracks;
            }
            let _ = stream
                .send(make_msg(&EffectHandlerResponse::EffectUpdated { id }))
                .await;
        }
    }
}

fn make_msg<T: serde::Serialize>(t: &T) -> rocket_ws::Message {
    rocket_ws::Message::Text(serde_json::to_string(t).unwrap())
}
