use crate::libs::cli::Cli;
use crate::libs::data::{Command, DataStore, KICK};
use crate::libs::protobuf::robot_packet::{IaToMainBoard};
use crate::libs::tasks::task::Task;
use clap::Args;
use log::{debug, error, info};
use prost::Message;
use serialport::SerialPort;
use std::io::Write;
use std::time::Duration;
use crate::libs::constants::NUMBER_OF_ROBOTS;

pub struct UsbCommandsOutputTask {
    port: Box<dyn SerialPort>,
}

#[derive(Args, Clone)]
pub struct UsbCommandsOutputTaskCli {
    /// USB port of the mainboard
    #[arg(long, default_value = "/dev/USB0")]
    usb_port: String,
}

impl UsbCommandsOutputTask {
    // TODO : Seperate the packet preparation to the send
    fn send(&mut self, commands: &mut Vec<Command>) {
        for command in commands.iter_mut() {

            let (kicker_cmd, kick_power) = match &command.kick {
                None => {
                    (0, 0.0 as f32) // TODO : Remove this 0 and use the kicker enum
                }
                Some(c) => {
                    match c {
                        KICK::STRAIGHT_KICK { power } => {
                            (1, power.clone())
                        }
                        KICK::CHIP_KICK { power } => {
                            (2, power.clone())
                        }
                    }
                }
            };



            let packet = IaToMainBoard {
                robot_id: command.id,
                normal_speed: command.forward_velocity,
                tangential_speed: command.left_velocity,
                angular_speed: command.angular_velocity,
                motor_break: false,
                kicker_cmd,
                kick_power,
                charge: command.charge,
                dribbler: command.dribbler > 0.0,
            };

            let mut buf = Vec::new();
            buf.reserve(packet.encoded_len() + 1);
            buf.push(packet.encoded_len() as u8);
            packet.encode(&mut buf).unwrap();

            info!("{}", packet.encoded_len());

            match self.port.write(&buf[0..packet.encoded_len() + 1]) {
                Ok(_v) => {
                    debug!("sent order: {:?}", packet);
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }
}

impl Task for UsbCommandsOutputTask {
    fn with_cli(cli: &mut Cli) -> Self {
        Self {
            port: serialport::new(cli.usb_commands.usb_port.clone(), 115_200)
                .timeout(Duration::from_millis(1))
                .open()
                .expect("Failed to open port"),
        }
    }

    fn run(&mut self, data_store: &mut DataStore) {
        self.send(&mut data_store.commands);
        data_store.commands.clear();
    }
}

impl Drop for UsbCommandsOutputTask {
    fn drop(&mut self) {
        let mut commands: Vec<Command> = vec!();

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
