use crate::inputs_outputs::output::OutputCommandSending;
use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{Command, Kick};
use crate::libs::protobuf::robot_packet::IaToMainBoard;
use crate::libs::robot::AllyRobotInfo;
use clap::Args;
use log::{debug, error};
use prost::Message;
use serialport::SerialPort;
use std::time::Duration;

#[derive(Args)]
pub struct USBClientCli {
    /// USB port of the mainboard
    #[arg(long, default_value = "/dev/ttyUSB0")]
    usb_port: String,
}

pub struct USBClient {
    port: Box<dyn SerialPort>,
}

impl USBClient {
    fn prepare_packet(&mut self, command: Command) -> IaToMainBoard {
        let (kicker_cmd, kick_power) = match command.kick {
            None => {
                (0, 0.0 as f32) // TODO : Remove this 0 and use the kicker enum
            }
            Some(Kick::StraightKick { power }) => (2, power),
            Some(Kick::ChipKick { power }) => (1, power),
        };

        IaToMainBoard {
            robot_id: command.id as u32,
            normal_speed: command.forward_velocity,
            tangential_speed: command.left_velocity,
            angular_speed: command.angular_velocity,
            motor_break: false,
            kicker_cmd,
            kick_power,
            charge: command.charge,
            dribbler: command.dribbler > 0.0,
        }
    }

    fn send(&mut self, packet: IaToMainBoard) {
        // TODO : Buffer on struct?
        let mut buf = Vec::new();
        buf.reserve(packet.encoded_len() + 1);
        buf.push(packet.encoded_len() as u8);
        packet.encode(&mut buf).unwrap();

        match self.port.write(&buf[0..packet.encoded_len() + 1]) {
            Ok(_v) => {
                debug!("sent order: {:?}", packet);
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    }

    // TODO : Add receive function
    fn receive(&self) -> [Option<AllyRobotInfo>; NUMBER_OF_ROBOTS] {
        Default::default()
    }
}

impl OutputCommandSending for USBClient {
    fn with_cli(cli: &Cli) -> Box<Self> {
        Box::new(Self {
            port: serialport::new(cli.usb_commands.usb_port.clone(), 115_200)
                .timeout(Duration::from_millis(1))
                .open()
                .expect("Failed to open port"),
        })
    }

    fn step(
        &mut self,
        commands: [Option<Command>; NUMBER_OF_ROBOTS],
    ) -> [Option<AllyRobotInfo>; NUMBER_OF_ROBOTS] {
        for command in commands.into_iter().filter_map(|x| x) {
            let packet = self.prepare_packet(command);
            self.send(packet);

            // TODO: FIX THIS RETURN
            // return self.receive();
        }
        Default::default()
    }
}

impl Drop for USBClient {
    fn drop(&mut self) {
        let mut commands: [Option<Command>; NUMBER_OF_ROBOTS] = Default::default();
        for (id, command) in commands.iter_mut().enumerate() {
            *command = Some(Command {
                id: id as u8,
                forward_velocity: 0.0,
                left_velocity: 0.0,
                angular_velocity: 0.0,
                charge: false,
                kick: None,
                dribbler: 0.0,
            });
        }

        self.step(commands);
    }
}
