use std::net::SocketAddr;

use artnet_protocol::{ArtCommand, Output, Poll};
use rocket::tokio::{net::UdpSocket, select};

use crate::fixture::UNIVERSE_SIZE;

use super::Endpoint;

pub struct ArtNetEndpoint {
    data: [u8; UNIVERSE_SIZE],
    connections: Vec<SocketAddr>,
}

impl Default for ArtNetEndpoint {
    fn default() -> Self {
        Self {
            data: [0; UNIVERSE_SIZE],
            connections: vec![],
        }
    }
}

impl Endpoint for ArtNetEndpoint {
    fn register(mut self, mut rx: rocket::tokio::sync::broadcast::Receiver<super::EndpointData>) {
        rocket::tokio::spawn(async move {
            let socket = UdpSocket::bind(("0.0.0.0", 6454)).await.unwrap();
            let broadcast_adds = ("255.255.255.255", 6454);
            socket.set_broadcast(true).unwrap();
            let buf = ArtCommand::Poll(Poll::default()).write_to_buffer().unwrap();
            socket.send_to(&buf, &broadcast_adds).await.unwrap();

            let mut buffer = [0u8; 1024];
            loop {
                select! {
                    Ok(msg) = rx.recv() => {
                        match msg {
                            super::EndpointData::Exit => {
                                println!("[ARTNET] Exiting");
                                break;
                            }
                            super::EndpointData::Single{channel, value} => {
                                self.data[channel.i()] = value;
                                send(&self.data, &self.connections, &socket).await;
                            }
                            super::EndpointData::Multiple{channels, values} => {
                                for (index, c) in channels.into_iter().enumerate() {
                                    self.data[c.i()] = values[index];
                                }
                                send(&self.data, &self.connections, &socket).await;
                            }
                            super::EndpointData::Entire { values } => {
                                self.data = values;
                                send(&self.data, &self.connections, &socket).await;
                            }
                        }
                    },
                    Ok((length, adds)) = socket.recv_from(&mut buffer) => {
                        let command = ArtCommand::from_buffer(&buffer[..length]).expect("?????? WHY");
                        match command {
                            ArtCommand::Poll(_) => {},
                            ArtCommand::PollReply(_) => {
                                println!("Connecting");
                                self.connections.push(adds);
                                let command = make_output(&self.data);
                                let bytes = command.write_to_buffer().unwrap();
                                socket.send_to(&bytes, &adds).await.unwrap();
                            },
                            _ => {}
                        }
                    }
                }
            }

            drop(socket);
        });
    }
}

fn make_output(data: &[u8; UNIVERSE_SIZE]) -> ArtCommand {
    ArtCommand::Output(Output {
        data: data.to_vec().into(),
        ..Default::default()
    })
}
async fn send(data: &[u8; UNIVERSE_SIZE], adds: &[SocketAddr], soket: &UdpSocket) {
    let cmd = make_output(data);
    let buf = cmd.write_to_buffer().unwrap();
    for a in adds {
        soket.send_to(&buf, a).await.unwrap();
    }
}
