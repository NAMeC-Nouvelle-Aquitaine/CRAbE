use crate::decision_making::commands_wrapper::CommandsWrapper;
use crate::libs::cli::Cli;
use crate::libs::data::{DataStore, Kick};
use log::debug;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, UdpSocket};
use crate::libs::data::Command;

const BUFFER_SIZE: usize = 4096;

pub struct Plankton {
    socket: UdpSocket,
    buf: [u8; BUFFER_SIZE],
    addr: String,
}

#[derive(Deserialize, Debug)]
pub enum PlanktonCommand {
    Command { id: u8, forward_velocity: f32, left_velocity: f32, angular_velocity: f32, charge: bool, kick: u8, dribbler: f32 }
}

#[derive(Deserialize, Debug)]
pub struct PlanktonResponse {
    commands: Vec<PlanktonCommand>,
}

impl Plankton {
    pub fn with_cli(cli: &Cli) -> Self {
        let ip = "0.0.0.0".to_string();
        let port = 11301;
        let addr = format!("{}:{}", ip, port);
        let socket = UdpSocket::bind("0.0.0.0:11300").expect("Failed to bind the UDP Socket");

        socket
            .set_nonblocking(true)
            .expect("Failed to set non blocking");

        Self {
            socket,
            buf: [0u8; BUFFER_SIZE],
            addr,
        }
    }

    pub fn step(&mut self, commands_wrapper: &mut CommandsWrapper, store: &DataStore) {
        debug!("{:#?}", store);
        let json = serde_json::to_vec(&store).unwrap();
        self.socket.send_to(&json[..], self.addr.as_str()).unwrap();

        if let Ok(p_size) = self.socket.recv(&mut self.buf) {
            let rep: PlanktonResponse = serde_json::from_slice(&self.buf[0..p_size]).unwrap();
            for command in rep.commands {
                match command {
                    PlanktonCommand::Command {id,
                        forward_velocity, left_velocity, angular_velocity,
                        charge, dribbler, kick } => {
                        commands_wrapper.add_command(id as usize, Command {
                            id,
                            forward_velocity,
                            left_velocity,
                            angular_velocity,
                            charge,
                            kick: None,
                            dribbler,
                        })
                    }
                }
            }
        }
    }
}

