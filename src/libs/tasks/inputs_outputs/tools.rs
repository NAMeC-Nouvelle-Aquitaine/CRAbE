use crate::libs::cli::Cli;
use crate::libs::data::{ControllableRobot, DataStore, Field, Robot, TeamColor};
use crate::libs::protobuf::simulation_packet::robot_move_command::Command::LocalVelocity;
use crate::libs::protobuf::simulation_packet::{MoveLocalVelocity, RobotCommand, RobotMoveCommand};
use crate::libs::protobuf::tools_packet;
use crate::libs::protobuf::tools_packet::{Commands, ToolsPacket};
use crate::libs::tasks::task::Task;
use clap::Command;
use prost::Message;
use std::io::Cursor;
use std::net::UdpSocket;

const BUFFER_SIZE: usize = 4096;

pub struct ToolsInputOutputTask {
    socket: UdpSocket,
    buf: [u8; BUFFER_SIZE],
    port: u32,
}

impl From<Field> for tools_packet::Field {
    fn from(value: Field) -> Self {
        Self {
            length: value.length,
            width: value.width,
            center_radius: value.center_radius,
            goal_width: value.goal_width,
            goal_depth: value.goal_depth,
            penalty_width: value.penalty_width,
            penalty_depth: value.penalty_depth,
        }
    }
}

impl From<Robot> for tools_packet::Robot {
    fn from(value: Robot) -> Self {
        Self {
            id: value.id,
            x: value.position.x,
            y: value.position.y,
            orientation: value.orientation,
        }
    }
}

impl From<ControllableRobot> for tools_packet::Robot {
    fn from(value: ControllableRobot) -> Self {
        Self {
            id: value.robot.id,
            x: value.robot.position.x,
            y: value.robot.position.y,
            orientation: value.robot.orientation,
        }
    }
}

impl tools_packet::SoftwarePacket {
    fn with_data_store(value: &DataStore) -> tools_packet::SoftwarePacket {
        let color = if let TeamColor::BLUE = value.color {
            tools_packet::Color::Blue
        } else {
            tools_packet::Color::Yellow
        };
        let field = tools_packet::Field::from(value.field.unwrap_or_default());

        let allies = value
            .allies
            .clone()
            .map(|r| tools_packet::Robot::from(r))
            .to_vec();
        let opponents = value.enemies.map(|r| tools_packet::Robot::from(r)).to_vec();

        tools_packet::SoftwarePacket {
            field: Option::from(field),
            color_team: color as i32,
            allies,
            opponents,
            ball: Option::from(tools_packet::Ball {
                x: value.ball.x,
                y: value.ball.y,
            }),
        }
    }
}

impl Task for ToolsInputOutputTask {
    fn with_cli(cli: &mut Cli) -> Self {
        let socket = UdpSocket::bind("127.0.0.1:10101").expect("Failed to bind the UDP Socket");

        socket
            .set_nonblocking(true)
            .expect("Failed to set socket to non-blocking mode");

        Self {
            socket,
            port: 10100, // TODO : Make cli port
            buf: [0u8; BUFFER_SIZE],
        }
    }

    fn run(&mut self, data_store: &mut DataStore) {
        let mut packet = tools_packet::SoftwarePacket::with_data_store(data_store);

        let mut buf = Vec::new();
        buf.reserve(packet.encoded_len());
        packet.encode(&mut buf).unwrap();

        self.socket
            .send_to(
                &buf[0..packet.encoded_len()],
                format!("127.0.0.1:{}", self.port),
            )
            .expect("couldn't send data");

        match self.socket.recv(&mut self.buf) {
            Ok(p_size) => {
                let packet = ToolsPacket::decode(Cursor::new(&self.buf[0..p_size]))
                    .expect("Error - Decoding the packet");

                match packet.commands {
                    Some(command) => {
                        dbg!("{}", &command);
                        let mut r = RobotCommand::default();
                        let mut move_robot = MoveLocalVelocity::default();

                        move_robot.forward = command.normal_speed;
                        move_robot.left = command.tangent_speed;
                        move_robot.angular = command.angular_speed;

                        if command.dribble {
                            r.dribbler_speed = Some(2.0);
                        }

                        r.move_command = Some(RobotMoveCommand {
                            command: Some(LocalVelocity(move_robot)),
                        });

                        if let Some(robot) = data_store.allies.get_mut(r.id as usize) {
                            robot.command = Some(r);
                        }
                    }
                    _ => {}
                }
                println!("receive");
            }
            Err(_e) => {}
        }

        // debug!("sent order: {:?}", packet);
    }
}
