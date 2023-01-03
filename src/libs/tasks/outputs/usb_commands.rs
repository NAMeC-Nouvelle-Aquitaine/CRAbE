use crate::libs::cli::Cli;
use crate::libs::data::DataStore;
use crate::libs::data::{ControllableRobot, ControllableRobotFeedback, Robot};
use crate::libs::protobuf::robot_packet::IaToMainBoard;
use crate::libs::protobuf::simulation_packet::robot_move_command::Command;
use crate::libs::protobuf::simulation_packet::{
    MoveWheelVelocity, RobotCommand, RobotControl, RobotControlResponse, RobotMoveCommand,
};
use crate::libs::tasks::task::Task;
use log::debug;
use prost::Message;
use serialport::SerialPort;
use std::io::Write;
use std::time::Duration;

pub struct UsbCommandsOutputTask {
    port: Box<dyn SerialPort>,
}

impl UsbCommandsOutputTask {
    // TODO : Seperate the packet preparation to the send
    fn send(&mut self, robots: &mut [ControllableRobot; 6]) {
        let mut packet: Option<IaToMainBoard> = None;
        for (id, robot) in robots.iter_mut().enumerate() {
            if let Some(mut cmd) = robot.command.take() {
                cmd.id = id as u32;

                packet = Some(IaToMainBoard {
                    robot_id: cmd.id,
                    normal_speed: 0.0,
                    tangential_speed: 0.0,
                    angular_speed: 0.0,
                    motor_break: false,
                    kicker_cmd: match cmd.kick_angle {
                        None => 0,
                        Some(0.0) => 1,
                        Some(_) => 2,
                    },
                    kick_power: match cmd.kick_speed {
                        None => 0.0,
                        Some(p) => p,
                    },
                    charge: false,
                    dribbler: match cmd.dribbler_speed {
                        None => false,
                        Some(_) => true,
                    },
                });
            }
        }

        match packet {
            None => {
                return;
            }
            Some(packet) => {
                let mut buf = Vec::new();
                buf.reserve(packet.encoded_len());
                packet.encode(&mut buf).unwrap();

                self.port
                    .write(&buf[0..packet.encoded_len()])
                    .expect("couldn't send data");

                debug!("sent order: {:?}", packet);
            }
        }
    }
}

impl Task for UsbCommandsOutputTask {
    fn with_cli(cli: &mut Cli) -> Self {
        Self {
            port: serialport::new("/dev/ttyUSB0", 115_200)
                .timeout(Duration::from_millis(10))
                .open()
                .expect("Failed to open port"),
        }
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        self.send(&mut data_store.allies);

        Ok(())
    }
}

impl Drop for UsbCommandsOutputTask {
    fn drop(&mut self) {
        let mut robots: [ControllableRobot; 6] = [Robot::default(); 6].map(|r| ControllableRobot {
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
