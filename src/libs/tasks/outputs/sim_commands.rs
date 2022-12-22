use crate::libs::cli::Cli;
use crate::libs::data::{ControllableRobot, Robot};
use crate::libs::protobuf::simulation_packet::robot_move_command::Command;
use crate::libs::protobuf::simulation_packet::{
    MoveWheelVelocity, RobotCommand, RobotControl, RobotMoveCommand,
};
use crate::libs::{data, tasks};
use clap::Args;
use data::DataStore;
use prost::Message;
use std::net::UdpSocket;
use tasks::task::Task;

pub struct SimCommandsOutputTask {
    socket: UdpSocket,
    port: u32,
}

impl SimCommandsOutputTask {
    fn send(&mut self, robots: &mut [ControllableRobot; 6]) {
        let mut buf = Vec::new();

        let mut packet = RobotControl::default();
        for (id, robot) in robots.iter_mut().enumerate() {
            if let Some(mut cmd) = robot.command.take() {
                cmd.id = id as u32;
                packet.robot_commands.push(cmd);
            }
        }

        buf.reserve(packet.encoded_len());
        packet.encode(&mut buf).unwrap();

        self.socket
            .send_to(
                &buf[0..packet.encoded_len()],
                format!("127.0.0.1:{}", self.port),
            )
            .expect("couldn't send data");
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

        Self {
            socket,
            port: if cli.y {
                cli.sim_commands.sim_commands_yellow_port
            } else {
                cli.sim_commands.sim_commands_blue_port
            },
        }
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        self.send(&mut data_store.allies);

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
                    kick_speed: None,
                    kick_angle: None,
                    dribbler_speed: None,
                }),
            })
            .map(|mut cr| {
                // cr.control(0.0, 0.0, 0.0);
                cr.dribble(false);
                cr
            });
        self.send(&mut robots);
    }
}
