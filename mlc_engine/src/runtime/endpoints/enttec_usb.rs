use enttecopendmx::EnttecOpenDMX;
use crate::runtime::endpoints::{Endpoint, EndpointData};

pub struct EnttecUsb {
    dummy: bool,
}

impl Default for EnttecUsb {
    fn default() -> Self {
        EnttecUsb {
            dummy: true
        }
    }
}

impl Endpoint for EnttecUsb {
    fn register(mut self, mut rx: rocket::tokio::sync::broadcast::Receiver<EndpointData>) {
        rocket::tokio::spawn(async move {
            rocket::tokio::task::spawn_local(async move {
                let s = self;
                let mut dmx_source = EnttecOpenDMX::new().unwrap();
                dmx_source.open().expect("Hi");

                while let Ok(msg) = rx.recv().await {
                    match msg {
                        EndpointData::Single { channel, value } => {
                            dmx_source.set_channel(channel.i(), value);
                        }
                        EndpointData::Multiple { values, channels } => {
                            for (i, channel) in channels.iter().enumerate() {
                                dmx_source.set_channel(channel.i(), values[i])
                            }
                        }
                        EndpointData::Entire { values } => {
                            for i in 0..513 {
                                dmx_source.set_channel(i, values[i]);
                            }
                        }
                        EndpointData::Exit => {
                            break;
                        }
                    }
                    dmx_source.render().unwrap();
                }

                dmx_source.close().unwrap();
            });
        });
    }
}