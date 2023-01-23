use crate::decision_making::commands_wrapper::CommandsWrapper;
use crate::libs::cli::Cli;
use crate::libs::data::Command;
use crate::libs::data::{DataStore, Kick};
use log::debug;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, UdpSocket};
use zmq::{Context, Socket};

const BUFFER_SIZE: usize = 4096;

pub struct Plankton {
    socket: Socket,
    buf: [u8; BUFFER_SIZE],
}

#[derive(Deserialize, Debug)]
pub enum PlanktonCommand {
    Command {
        id: u8,
        forward_velocity: f32,
        left_velocity: f32,
        angular_velocity: f32,
        charge: bool,
        kick: u8,
        dribbler: f32,
    },
}

#[derive(Deserialize, Debug)]
pub struct PlanktonResponse {
    commands: Vec<PlanktonCommand>,
}

impl Plankton {
    pub fn step(&mut self, commands_wrapper: &mut CommandsWrapper, store: &DataStore) {
        let packet = serde_json::to_string(&store).unwrap();
        if let Ok(_) = self.socket.send(packet.as_str(), 0) {
            //self.socket.send_to(&json[..], self.addr.as_str()).unwrap();

            let received_message = self.socket.recv_msg(0).unwrap();
            let rep: PlanktonResponse =
                serde_json::from_str(received_message.as_str().unwrap()).unwrap();
            for command in rep.commands {
                match command {
                    PlanktonCommand::Command {
                        id,
                        forward_velocity,
                        left_velocity,
                        angular_velocity,
                        charge,
                        dribbler,
                        kick,
                    } => commands_wrapper.add_command(
                        id as usize,
                        Command {
                            id,
                            forward_velocity,
                            left_velocity,
                            angular_velocity,
                            charge,
                            kick: None,
                            dribbler,
                        },
                    ),
                }
            }
        }

        /*if let Ok(p_size) = self.socket.recv(&mut self.buf) {
            let rep: PlanktonResponse = serde_json::from_slice(&self.buf[0..p_size]).unwrap();
            for command in rep.commands {
                match command {
                    PlanktonCommand::Command {
                        id,
                        forward_velocity,
                        left_velocity,
                        angular_velocity,
                        charge,
                        dribbler,
                        kick,
                    } => commands_wrapper.add_command(
                        id as usize,
                        Command {
                            id,
                            forward_velocity,
                            left_velocity,
                            angular_velocity,
                            charge,
                            kick: None,
                            dribbler,
                        },
                    ),
                }
            }
        }*/
    }

    pub fn with_cli(cli: &Cli) -> Self {
        let port = if cli.yellow { 11301 } else { 11300 };
        let socket = Context::new().socket(zmq::REQ).unwrap();
        socket
            .bind(format!("tcp://127.0.0.1:{port}").as_str())
            .unwrap();
        socket.set_sndtimeo(1).expect("Failed to set snd timeout");
        Self {
            socket,
            buf: [0u8; BUFFER_SIZE],
        }
    }
}
