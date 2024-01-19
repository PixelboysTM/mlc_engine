use std::time::Duration;

use rocket::tokio::select;
use sacn::DmxSource;

use crate::fixture::UNIVERSE_SIZE;

use super::Endpoint;

pub struct SacnEndpoint {
    pub data: [u8; UNIVERSE_SIZE],
    pub universe: u16,
    pub speed: Speed,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Speed {
    Slow,   // 200ms
    Medium, // 100ms
    Fast,   // 30ms
}

impl Speed {
    fn get_duration(&self) -> Duration {
        match self {
            Speed::Slow => Duration::from_millis(200),
            Speed::Medium => Duration::from_millis(100),
            Speed::Fast => Duration::from_millis(30),
        }
    }
}

impl Default for SacnEndpoint {
    fn default() -> Self {
        Self {
            data: [0; UNIVERSE_SIZE],
            universe: 1,
            speed: Speed::Medium,
        }
    }
}

impl Endpoint for SacnEndpoint {
    fn register(mut self, mut rx: rocket::tokio::sync::broadcast::Receiver<super::EndpointData>) {
        rocket::tokio::spawn(async move {
            let dmx_source = DmxSource::new("MLC Controller").unwrap();

            let mut sleep = rocket::tokio::time::interval(self.speed.get_duration());

            loop {
                select! {
                    Ok(msg) = rx.recv() => {
                        match msg {
                            super::EndpointData::Exit => break,
                            super::EndpointData::Single { channel, value } => {
                                    self.data[channel.i()] = value;
                                    // dmx_source.send(self.universe, &self.data).unwrap();
                                }
                            super::EndpointData::Multiple { channels, values } => {
                                let mut index = 0;
                                for c in channels {
                                    self.data[c.i()] = values[index];
                                    index += 1;
                                }
                                // dmx_source.send(self.universe, &self.data).unwrap();
                            }
                            super::EndpointData::Entire { values } => {
                                self.data = values;
                                // dmx_source.send(self.universe, &self.data).unwrap();
                            }
                        }
                    },
                    _ = sleep.tick() => {
                        dmx_source.send(self.universe, &self.data).unwrap();
                    }
                }
            }

            dmx_source.terminate_stream(1).unwrap();
        });
    }
}
