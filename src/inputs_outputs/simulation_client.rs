use crate::inputs_outputs::output::OutputCommandSending;
use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{Command, KICK};
use crate::libs::protobuf::simulation_packet::{
    robot_move_command, MoveLocalVelocity, RobotCommand, RobotControl, RobotControlResponse,
    RobotMoveCommand,
};
use crate::libs::robot::AllyRobotInfo;
use log::debug;
use prost::Message;
use serialport::ClearBuffer::All;
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
    fn prepare_packet(&self, commands: &Vec<Command>) -> RobotControl {
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

        packet
    }

    fn send(&mut self, packet: RobotControl) {
        self.buf.reserve(packet.encoded_len());
        packet.encode(&mut self.buf).unwrap();

        self.socket
            .send_to(
                &buf[0..packet.encoded_len()],
                format!("127.0.0.1:{}", self.port),
            )
            .expect("couldn't send data");

        debug!("sent order: {:?}", packet);
    }

    fn receive(&mut self) -> Vec<AllyRobotInfo> {
        return match self.socket.recv(&mut self.buf) {
            Ok(p_size) => {
                let mut ally_info: Vec<AllyRobotInfo> = vec![]; // TODO: We don't have id !
                let packet = RobotControlResponse::decode(Cursor::new(&self.buf[0..p_size]))
                    .expect("Error - Decoding the packet");

                for robot_feedback in packet.feedback {
                    debug!(
                        "assigned feedback {:?} to robot #{}",
                        robot_feedback, robot_feedback.id
                    );

                    ally_info.push(AllyRobotInfo {
                        has_ball: robot_feedback.dribbler_ball_contact(),
                    })
                }

                ally_info
            }
            Err(e) => {
                vec![]
            }
        };
    }
}

impl OutputCommandSending for SimulationClient {
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

    fn step(&mut self, commands: Vec<Command>) -> Vec<AllyRobotInfo> {
        if commands.is_empty() {
            return vec![];
        }

        let mut packet = self.prepare_packet(&commands);
        self.send(packet);
        self.receive()
    }
}

impl Drop for SimulationClient {
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
        self.step(commands);
    }
}
