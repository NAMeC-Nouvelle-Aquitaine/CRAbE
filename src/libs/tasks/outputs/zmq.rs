use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{DataStore, Field, TeamColor};
use crate::libs::robot::{AllyRobot, EnemyRobot, Robot};
use crate::libs::tasks::task::Task;
use log::debug;
use serde::Serialize;
use zmq::Socket;

pub struct ZmqOutputTask {
    socket: Socket,
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
        let payload = serde_json::to_string(&data_store).expect("TODO: some meaningful message");

        debug!("{:?}", payload);
        self.socket.send(payload.as_str(), zmq::DONTWAIT).unwrap();
    }
}
