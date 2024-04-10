use crate::runtime::endpoints::{Endpoint, EndpointData};
use mlc_common::endpoints::Speed;
use open_dmx::DMXSerial;

pub struct UsbEndpoint {
    pub(crate) port: String,
    pub(crate) speed: Speed,
}

impl Default for UsbEndpoint {
    fn default() -> Self {
        Self {
            port: String::new(),
            speed: Speed::Medium,
        }
    }
}

impl Endpoint for UsbEndpoint {
    fn register(self, mut rx: rocket::tokio::sync::broadcast::Receiver<EndpointData>) {
        rocket::tokio::spawn(async move {
            let mut dmx = DMXSerial::open(&self.port).unwrap();
            dmx.set_packet_time(self.speed.get_duration());
            while let Ok(data) = rx.recv().await {
                match data {
                    EndpointData::Single { channel, value } => {
                        dmx.set_channel(channel.i() + 1, value)
                            .expect("Sending usb data failed");
                    }
                    EndpointData::Multiple { channels, values } => {
                        for (i, channel) in channels.iter().enumerate() {
                            dmx.set_channel(channel.i() + 1, values[i])
                                .expect("Sending usb data failed");
                        }
                    }
                    EndpointData::Entire { values } => {
                        dmx.set_channels(values);
                    }
                    EndpointData::Exit => {}
                }
            }
        });
    }
}
