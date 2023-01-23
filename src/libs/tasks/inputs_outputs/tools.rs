use crate::libs::cli::Cli;
use crate::libs::data::{DataStore, Field, TeamColor};
use crate::libs::protobuf::tools_packet;
use crate::libs::robot::Robot;
use crate::libs::tasks::task::Task;
use prost::Message;
use serde::Serialize;
use std::net::UdpSocket;
use std::time::Instant;

pub struct ToolsInputOutputTask {
    socket: UdpSocket,
    port: u32,
    last_send: Instant,
}

#[derive(Serialize)]
struct ToolsData<'a> {
    store: &'a DataStore,
}

impl Task for ToolsInputOutputTask {
    fn with_cli(_cli: &mut Cli) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind the UDP Socket");

        socket
            .set_nonblocking(true)
            .expect("Failed to set socket to non-blocking mode");

        Self {
            socket,
            port: 10100, // TODO : Make cli port
            last_send: Instant::now(),
        }
    }

    fn run(&mut self, data_store: &mut DataStore) {
        if self.last_send.elapsed().as_millis() > 16 {
            self.last_send = Instant::now();
            let tools_data = ToolsData { store: &data_store };
            let packet = serde_json::to_string(&tools_data);
            let s = packet.unwrap().encode_to_vec();

            self.socket
                .send_to(&s, format!("127.0.0.1:{}", self.port))
                .expect("couldn't send data");
        }
    }
}
