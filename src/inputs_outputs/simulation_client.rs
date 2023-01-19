use crate::inputs_outputs::multicast_client::BUFFER_SIZE;
use crate::inputs_outputs::output::OutputCommandSending;
use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{Command, Kick};
use crate::libs::protobuf::simulation_packet::{
    robot_move_command, MoveLocalVelocity, RobotCommand, RobotControl, RobotControlResponse,
    RobotMoveCommand,
};
use crate::libs::robot::AllyRobotInfo;
use clap::Args;
use log::{debug, error};
use prost::Message;
use std::io::Cursor;
use std::net::UdpSocket;

#[derive(Args)]
pub struct SimulationClientCli {
    /// blue team simulation output port
    #[arg(long, default_value_t = 10301)]
    sim_blue_port: u32,

    /// blue team simulation output port
    #[arg(long, default_value_t = 10302)]
    sim_yellow_port: u32,
}

pub struct SimulationClient {
    socket: UdpSocket,
    buf: [u8; BUFFER_SIZE],
    port: u32,
}

impl SimulationClient {
    fn prepare_packet<'a>(&self, commands: impl Iterator<Item = &'a Command>) -> RobotControl {
        let mut packet = RobotControl::default();

        for command in commands {
            let (kick_speed, kick_angle) = match &command.kick {
                None => (0.0, 0.0),
                Some(Kick::StraightKick { power }) => (*power, 0.0),
                Some(Kick::ChipKick { power }) => (*power, 45.0),
            };

            let robot_command = RobotCommand {
                id: command.id as u32,
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

        packet
    }

    fn send(&mut self, packet: RobotControl) {
        // TODO : Buffer on struct?
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
    }

    fn receive(&mut self) -> [Option<AllyRobotInfo>; NUMBER_OF_ROBOTS] {
        let mut ally_info: [Option<AllyRobotInfo>; NUMBER_OF_ROBOTS] = Default::default(); // TODO: We don't have id !
        return match self.socket.recv(&mut self.buf) {
            Ok(p_size) => {
                let packet = RobotControlResponse::decode(Cursor::new(&self.buf[0..p_size]))
                    .expect("Error - Decoding the packet");

                for robot_feedback in packet.feedback {
                    debug!(
                        "assigned feedback {:?} to robot #{}",
                        robot_feedback, robot_feedback.id
                    );

                    match ally_info.get_mut(robot_feedback.id as usize) {
                        None => {}
                        Some(ally_info) => {
                            *ally_info = Some(AllyRobotInfo {
                                has_ball: robot_feedback.dribbler_ball_contact(),
                            });
                        }
                    }
                }

                ally_info
            }
            Err(_e) => {
                error!("couldn't recv from socket");
                ally_info
            }
        };
    }
}

impl OutputCommandSending for SimulationClient {
    fn with_cli(cli: &Cli) -> Box<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind the UDP Socket");

        socket
            .set_nonblocking(true)
            .expect("Failed to set socket to non-blocking mode");

        Box::new(Self {
            socket,
            port: if cli.y {
                cli.sim_commands.sim_yellow_port
            } else {
                cli.sim_commands.sim_blue_port
            },
            buf: [0u8; BUFFER_SIZE],
        })
    }

    fn step(
        &mut self,
        commands: [Option<Command>; NUMBER_OF_ROBOTS],
    ) -> [Option<AllyRobotInfo>; NUMBER_OF_ROBOTS] {
        if commands.is_empty() {
            return Default::default();
        }

        let packet = self.prepare_packet(commands.iter().filter_map(|x| x.as_ref()));
        self.send(packet);
        self.receive()
    }
}

impl Drop for SimulationClient {
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
