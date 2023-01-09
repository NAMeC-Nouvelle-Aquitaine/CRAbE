use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{
    ControllableRobot, ControllableRobotFeedback, DataStore, Field, Robot, TeamColor,
};
use crate::libs::tasks::task::Task;
use log::debug;
use serde::Serialize;
use zmq::Socket;

pub struct ZmqOutputTask {
    socket: Socket,
}

#[derive(Serialize)]
pub struct ZmqPacket {
    pub color: TeamColor,
    pub blue_on_positive_half: bool,
    pub ball: [f32; 2],
    pub robots: ZmqPacketRobots,
    pub field: Option<Field>,
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
    pub feedback: Option<ControllableRobotFeedback>,
}

impl From<Robot> for ZmqRobot {
    fn from(value: Robot) -> Self {
        Self {
            position: [value.position.x, value.position.y],
            orientation: value.orientation,
            feedback: None,
        }
    }
}

impl From<ControllableRobot> for ZmqRobot {
    fn from(value: ControllableRobot) -> Self {
        Self {
            position: [value.robot.position.x, value.robot.position.y],
            orientation: value.robot.orientation,
            feedback: value.feedback,
        }
    }
}

impl ZmqPacket {
    fn with_data_store(value: &DataStore) -> Self {
        let yellow;
        let blue;
        if let TeamColor::BLUE = value.color {
            blue = value.allies.clone().map(|r| ZmqRobot::from(r));
            yellow = value.enemies.map(|r| ZmqRobot::from(r));
        } else {
            blue = value.enemies.map(|r| ZmqRobot::from(r));
            yellow = value.allies.clone().map(|r| ZmqRobot::from(r));
        };

        Self {
            color: value.color,
            blue_on_positive_half: value.blue_on_positive_half,
            ball: [value.ball.x, value.ball.y],
            robots: ZmqPacketRobots { blue, yellow },
            field: value.field,
        }
    }
}

impl Default for ZmqOutputTask {
    fn default() -> Self {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::PUB).unwrap();
        socket.set_sndtimeo(1).expect("Failed to set snd timeout");
        socket.bind("tcp://127.0.0.1:7557").unwrap();

        Self { socket }
    }
}

impl Task for ZmqOutputTask {
    fn with_cli(_cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) {
        let payload = serde_json::to_string(&ZmqPacket::with_data_store(data_store))
            .expect("TODO: some meaningful message");

        debug!("{:?}", payload);
        self.socket.send(payload.as_str(), zmq::DONTWAIT).unwrap();
    }
}
