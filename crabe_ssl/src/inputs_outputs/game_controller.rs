use crate::inputs_outputs::multicast_client::MulticastClient;
use crate::libs::cli::Cli;
use crabe_protobuf::game_controller_packet::Referee;
use std::sync::mpsc;

pub struct GameController {
    multicast_client: MulticastClient<Referee>,
}

impl GameController {
    pub fn with_cli(tx: mpsc::Sender<Referee>, _cli: &Cli) -> Self {
        Self {
            multicast_client: MulticastClient::with_cli(tx, "224.5.23.1".to_string(), 10003),
        }
    }

    pub fn run(&mut self) {
        self.multicast_client.run()
    }
}
