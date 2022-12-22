use crate::libs::cli::Cli;
use crate::libs::data::{DataStore, Robot, TeamColor};
use crate::libs::tasks::task::Task;
use nalgebra::Point2;
use serde::Serialize;
use zmq::Socket;
use crate::libs::constants::NUMBER_OF_ROBOTS;


pub struct ZmqOutputTask {
    socket: Socket,
}

#[derive(Serialize)]
pub struct ZmqPacket {
    pub color: TeamColor,
    pub blue_on_positive_half: bool,
    pub ball: [f32; 2],
    pub robots: ZmqPacketRobots,
}

#[derive(Serialize)]
pub struct ZmqPacketRobots {
    blue: [ZmqRobot; NUMBER_OF_ROBOTS],
    yellow: [ZmqRobot; NUMBER_OF_ROBOTS],
}

#[derive(Serialize)]
pub struct ZmqRobot {
    pub position: [f32; 2],
    pub orientation: f32,
}

impl From<Robot> for ZmqRobot {
    fn from(value: Robot) -> Self {
        Self {
            position: [value.position.x, value.position.y],
            orientation: value.orientation,
        }
    }
}

impl ZmqPacket {
    fn with_data_store(value: &DataStore) -> Self {
        let yellow;
        let blue;
        if let TeamColor::BLUE = value.color {
            blue = value
                .allies
                .clone()
                .map(|cr| cr.robot)
                .map(|r| ZmqRobot::from(r));
            yellow = value.enemies.map(|r| ZmqRobot::from(r));
        } else {
            blue = value.enemies.map(|r| ZmqRobot::from(r));
            yellow = value
                .allies
                .clone()
                .map(|cr| cr.robot)
                .map(|r| ZmqRobot::from(r));
        };

        Self {
            color: value.color,
            blue_on_positive_half: value.blue_on_positive_half,
            ball: [value.ball.x, value.ball.y],
            robots: ZmqPacketRobots { blue, yellow },
        }
    }
}

impl Default for ZmqOutputTask {
    fn default() -> Self {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::PUB).unwrap();
        socket.bind("tcp://127.0.0.1:7557").unwrap();

        Self { socket }
    }
}

impl Task for ZmqOutputTask {
    fn with_cli(cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        let payload = serde_json::to_string(&ZmqPacket::with_data_store(data_store))
            .expect("TODO: some meaningful message");
        self.socket.send(payload.as_str(), zmq::DONTWAIT).unwrap();

        Ok(())
    }
}