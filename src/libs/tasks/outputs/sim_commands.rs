use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{Command, KICK};
use crate::libs::protobuf::simulation_packet::{
    robot_move_command, MoveLocalVelocity, RobotCommand, RobotControl, RobotControlResponse,
    RobotMoveCommand,
};
use crate::libs::robot::{AllyRobot, AllyRobotInfo};
use crate::libs::{data, tasks};
use clap::Args;
use data::DataStore;
use log::{debug, error};
use prost::Message;
use serialport::ClearBuffer::All;
use std::io::Cursor;
use std::net::UdpSocket;
use tasks::task::Task;

const BUFFER_SIZE: usize = 4096;

pub struct SimCommandsOutputTask {
    socket: UdpSocket,
    buf: [u8; BUFFER_SIZE],
    port: u32,
}

impl SimCommandsOutputTask {
    // TODO : Seperate the packet preparation to the send
    // TODO : Refactor lines 37, 52 and 88 to 89 | Don't send feedback if no command has been sent
    fn send(&mut self, commands: &mut Vec<Command>) -> bool {
        let mut packet = RobotControl::default();

        while let Some(command) = commands.last().take() {
            let (kick_speed, kick_angle) = match &command.kick {
                None => (0.0, 0.0),
                Some(KICK::StraightKick { power }) => (*power, 0.0),
                Some(KICK::ChipKick { power }) => (*power, 45.0),
            };

            let robot_command = RobotCommand {
                id: command.id,
                move_command: Some(RobotMoveCommand {
                    command: Some(robot_move_command::Command::LocalVelocity {
                        0: MoveLocalVelocity {
                            forward: command.forward_velocity,
                            left: command.left_velocity,
                            angular: command.angular_velocity,
                        },
                    }),
                }),
                kick_speed: Some(kick_speed),
                kick_angle: Some(kick_angle),
                dribbler_speed: Some(command.dribbler),
            };
            packet.robot_commands.push(robot_command);
        }

        if packet.robot_commands.len() == 0 {
            return false;
        }

        let mut buf = Vec::new();
        buf.reserve(packet.encoded_len());
        packet.encode(&mut buf).unwrap();

        self.socket
            .send_to(
                &buf[0..packet.encoded_len()],
                format!("127.0.0.1:{}", self.port),
            )
            .expect("couldn't send data");

        debug!("sent order: {:?}", packet);
        true
    }
}

#[derive(Args)]
pub struct SimCommandsOutputTaskCli {
    /// blue team simulation output port
    #[arg(long, default_value_t = 10301)]
    sim_blue_port: u32,

    /// blue team simulation output port
    #[arg(long, default_value_t = 10302)]
    sim_yellow_port: u32,
}

impl Task for SimCommandsOutputTask {
    fn with_cli(cli: &mut Cli) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind the UDP Socket");

        socket
            .set_nonblocking(true)
            .expect("Failed to set socket to non-blocking mode");

        Self {
            socket,
            port: if cli.y {
                cli.sim_commands.sim_yellow_port
            } else {
                cli.sim_commands.sim_blue_port
            },
            buf: [0u8; BUFFER_SIZE],
        }
    }

    fn run(&mut self, data_store: &mut DataStore) {
        let sending = self.send(&mut data_store.commands);
        if sending {
            match self.socket.recv(&mut self.buf) {
                Ok(p_size) => {
                    let packet = RobotControlResponse::decode(Cursor::new(&self.buf[0..p_size]))
                        .expect("Error - Decoding the packet");

                    for robot_feedback in packet.feedback {
                        debug!(
                            "assigned feedback {:?} to robot #{}",
                            robot_feedback, robot_feedback.id
                        );

                        match data_store
                            .allies
                            .get_mut(robot_feedback.id as usize)
                            .unwrap()
                            .info
                        {
                            None => {
                                data_store
                                    .allies
                                    .get_mut(robot_feedback.id as usize)
                                    .unwrap()
                                    .info = Some(AllyRobotInfo {
                                    has_ball: robot_feedback
                                        .dribbler_ball_contact
                                        .unwrap_or_default(),
                                });
                            }
                            Some(AllyRobotInfo { mut has_ball }) => {
                                has_ball = robot_feedback.dribbler_ball_contact.unwrap_or_default();
                            }
                        };
                    }
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
        data_store.commands.clear();
    }
}

impl Drop for SimCommandsOutputTask {
    fn drop(&mut self) {
        let mut commands: Vec<Command> = vec![];

        for i in 0..NUMBER_OF_ROBOTS {
            commands.push(Command {
                id: i as u32,
                forward_velocity: 0.0,
                left_velocity: 0.0,
                angular_velocity: 0.0,
                charge: false,
                kick: None,
                dribbler: 0.0,
            })
        }
        self.send(&mut commands);
    }
}
