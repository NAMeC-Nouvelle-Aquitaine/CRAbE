use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use serde_json::Value::String;
use crate::decision_making::commands_wrapper::CommandsWrapper;
use crate::libs::cli::Cli;
use crate::libs::data::{Command, DataStore};

const BUFFER_SIZE: usize = 4096;

struct Plankton {
    socket: UdpSocket,
    buf: [u8; BUFFER_SIZE],
}

impl Plankton {
    fn with_cli(cli: &Cli) -> Self {
        let ip = "224.5.23.2".to_string();
        let port = 11300;
        let ipv4 = Ipv4Addr::from_str(ip.as_str()).expect("TODO: Failed to parse vision server ip");
        let socket =
            UdpSocket::bind(format!("{}:{}", ip, port)).expect("Failed to bind the UDP Socket");
        socket
            .join_multicast_v4(&ipv4, &Ipv4Addr::UNSPECIFIED)
            .expect("Error to join multicast group");
        socket
            .set_nonblocking(true)
            .expect("Failed to set non blocking");

        Self{
            socket,
            buf: [0u8; BUFFER_SIZE]
        }
    }

    fn run(&mut self, commands_wrapper: &CommandsWrapper, store: &DataStore) {
        let json = serde_json::to_vec(&store).unwrap();
        self.socket.send(json.as_slice()).unwrap();

        if let Ok(p_size) = self.socket.recv(&mut self.buf) {
            let req: Vec<Command> = serde_json::from_slice(&self.buf[0..p_size]).unwrap();
            dbg!(req);
        }
    }
}
