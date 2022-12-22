use crate::libs::{data, tasks};

use crate::libs::cli::Cli;
use crate::libs::protobuf::game_controller_packet::Referee;
use data::DataStore;
use prost::Message;
use std::io::Cursor;
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;
use tasks::task::Task;

pub struct GameControllerInputTask {
    socket: UdpSocket,
}

impl Default for GameControllerInputTask {
    fn default() -> Self {
        let ipv4 = Ipv4Addr::new(224, 5, 23, 1);
        let socket = UdpSocket::bind("224.5.23.1:10003").expect("Failed to bind the UDP Socket");
        socket
            .join_multicast_v4(&ipv4, &Ipv4Addr::UNSPECIFIED)
            .expect("Error to join multicast group");
        socket
            .set_read_timeout(Some(Duration::from_millis(5)))
            .expect("Failed to set read timeout");

        Self { socket }
    }
}

impl Task for GameControllerInputTask {
    fn with_cli(cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        let mut buf = [0u8; 4096];
        match self.socket.recv_from(&mut buf) {
            Ok((p_size, _)) => {
                let packet = Referee::decode(Cursor::new(&buf[0..p_size]))
                    .expect("Error - Decoding the packet");
                data_store.game_controller = Some(packet);
            }
            Err(_err) => {
                println!("Is GameController is running ?");
            }
        };

        Ok(())
    }
}
