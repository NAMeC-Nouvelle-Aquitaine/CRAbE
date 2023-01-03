use crate::libs::cli::Cli;
use crate::libs::data::{ControllableRobot, ControllableRobotFeedback, Robot};
use crate::libs::protobuf::simulation_packet::robot_move_command::Command;
use crate::libs::protobuf::simulation_packet::{
    MoveWheelVelocity, RobotCommand, RobotControl, RobotControlResponse, RobotMoveCommand,
};
use crate::libs::{data, tasks};
use clap::Args;
use data::DataStore;
use log::{debug, error};
use prost::Message;
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
    fn send(&mut self, robots: &mut [ControllableRobot; 6]) -> bool {
        let mut packet = RobotControl::default();
        for (id, robot) in robots.iter_mut().enumerate() {
            if let Some(mut cmd) = robot.command.take() {
                cmd.id = id as u32;
                packet.robot_commands.push(cmd);
            }
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
    /// blue team output port
    #[arg(long, default_value_t = 10301)]
    sim_commands_blue_port: u32,

    /// blue team output port
    #[arg(long, default_value_t = 10302)]
    sim_commands_yellow_port: u32,
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
                cli.sim_commands.sim_commands_yellow_port
            } else {
                cli.sim_commands.sim_commands_blue_port
            },
            buf: [0u8; BUFFER_SIZE],
        }
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        
        let sending = self.send(&mut data_store.allies);
        if sending {
            match self.socket.recv(&mut self.buf) {
                Ok(p_size) => {
                    let packet = RobotControlResponse::decode(Cursor::new(&self.buf[0..p_size]))
                        .expect("Error - Decoding the packet");

                    for robot_feedback in packet.feedback {
                        match data_store.allies.get_mut(robot_feedback.id as usize) {
                            None => {
                                error!(
                                    "Cannot assign robot feedback to our robot #{}",
                                    robot_feedback.id
                                );
                            }
                            Some(robot) => {
                                debug!(
                                    "assigned feedback {:?} to robot #{}",
                                    robot_feedback, robot_feedback.id
                                );
                                robot.feedback = Some(ControllableRobotFeedback {
                                    infrared: robot_feedback.dribbler_ball_contact.unwrap_or_default(),
                                });
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
       }
    
        Ok(())
    }
}

impl Drop for SimCommandsOutputTask {
    fn drop(&mut self) {
        let mut robots: [ControllableRobot; 6] = [Robot::default(); 6]
            .map(|r| ControllableRobot {
                robot: r,
                command: Some(RobotCommand {
                    id: 0,
                    move_command: Some(RobotMoveCommand {
                        command: Some(Command::WheelVelocity(MoveWheelVelocity {
                            front_right: 0.0,
                            back_right: 0.0,
                            back_left: 0.0,
                            front_left: 0.0,
                        })),
                    }),
                    kick_speed: Some(0.0),
                    kick_angle: Some(0.0),
                    dribbler_speed: Some(0.0),
                }),
                feedback: None,
            });
        self.send(&mut robots);
    }
}
