use crate::libs::cli::Cli;
use crate::libs::protobuf::game_controller_packet::Referee;
use prost::Message;
use std::io::Cursor;
use std::net::{Ipv4Addr, UdpSocket};
use std::sync::mpsc;
use crate::inputs_outputs::multicast_client::MulticastClient;

const BUFFER_SIZE: usize = 4096;

pub struct GameController {
    multicast_client:MulticastClient<Referee>
}

impl GameController {
    pub fn with_cli(tx: mpsc::Sender<Referee>, _cli: &mut Cli) -> Self {
        Self { multicast_client: MulticastClient::with_cli(tx, "224.5.23.1".to_string(), 10003) }
    }

    pub fn run(&mut self) {
        self.multicast_client.run()
    }
}
