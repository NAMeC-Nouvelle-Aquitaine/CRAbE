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

#[derive(Deserialize, Debug, PartialEq)]
pub enum PlanktonCommand {
    Command(Command),
}

impl Plankton {
    pub fn step(
        &mut self,
        commands_wrapper: &mut CommandsWrapper,
        store: &DataStore,
    ) -> Result<(), String> {
        let packet = serde_json::to_string(&store).map_err(|e| e.to_string())?;
        self.socket
            .send(packet.as_str(), 0)
            .map_err(|e| e.to_string())?;
        let received_message = self.socket.recv_msg(0).map_err(|e| e.to_string())?;
        let rep = serde_json::from_str::<Vec<PlanktonCommand>>(received_message.as_str().unwrap())
            .map_err(|e| e.to_string())?;

        for command in rep {
            match command {
                PlanktonCommand::Command(command) => {
                    commands_wrapper.add_command(command.id as usize, command)
                }
            }
        }

        Ok(())
    }

    pub fn with_cli(cli: &Cli) -> Self {
        let port = if cli.yellow { 11301 } else { 11300 };
        let socket = Context::new().socket(zmq::REQ).unwrap();
        socket
            .bind(format!("tcp://127.0.0.1:{port}").as_str())
            .unwrap();
        socket.set_sndtimeo(10).expect("Failed to set snd timeout");
        socket.set_rcvtimeo(10).expect("Failed to set rcv timeout");
        Self {
            socket,
            buf: [0u8; BUFFER_SIZE],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod deserialize {
        use super::*;

        #[test]
        fn test_plankton_packet() {
            struct TestCase {
                input: &'static str,
                expected: Result<Vec<PlanktonCommand>, ()>,
            }

            let tests = vec![
                TestCase {
                    // TC0 full packet deserialize
                    input: r#"
                    [
                        {"Command": {"id": 1, "forward_velocity": 4.0, "left_velocity": 3.0, "angular_velocity": 1.0, "charge": true, "kick": {"StraightKick": {"power": 0.5}}, "dribbler": 1.0}}, 
                        {"Command": {"id": 2, "forward_velocity": 1.0, "left_velocity": 0.0, "angular_velocity": 0.0, "charge": false, "dribbler": 0.0}}
                    ]
                    "#,
                    expected: Ok(vec![
                        PlanktonCommand::Command(Command {
                            id: 1,
                            forward_velocity: 4.0,
                            left_velocity: 3.0,
                            angular_velocity: 1.0,
                            charge: true,
                            kick: Some(Kick::StraightKick { power: 0.5 }),
                            dribbler: 1.0,
                        }),
                        PlanktonCommand::Command(Command {
                            id: 2,
                            forward_velocity: 1.0,
                            left_velocity: 0.0,
                            angular_velocity: 0.0,
                            charge: false,
                            kick: None,
                            dribbler: 0.0,
                        }),
                    ]),
                },
                TestCase {
                    // TC1 empty packet deserialize
                    input: r#"
                    [
                    ]
                    "#,
                    expected: Ok(vec![]),
                },
            ];

            for (index, test) in tests.into_iter().enumerate() {
                let actual = serde_json::from_str::<Vec<PlanktonCommand>>(test.input);
                match (actual, test.expected) {
                    (Ok(actual), Ok(expected)) => {
                        assert_eq!(actual, expected, "TC{} failed", index)
                    }
                    (Err(_), Err(_)) => {
                        // Test passed
                    }
                    (actual, expected) => {
                        // Test failed
                        panic!("TC{index} failed because actual != expected. \nActual: {actual:?}\nExpected: {expected:?}\n");
                    }
                }
            }
        }
    }
}
