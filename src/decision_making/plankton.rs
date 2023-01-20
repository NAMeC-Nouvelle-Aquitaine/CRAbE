use crate::decision_making::commands_wrapper::CommandsWrapper;
use crate::libs::cli::Cli;
use crate::libs::data::{Command, DataStore};
use log::debug;
use std::net::{Ipv4Addr, UdpSocket};

const BUFFER_SIZE: usize = 4096;

pub struct Plankton {
    socket: UdpSocket,
    buf: [u8; BUFFER_SIZE],
    addr: String,
}

impl Plankton {
    pub fn with_cli(cli: &Cli) -> Self {
        let ip = "127.0.0.1".to_string();
        let port = 11300;
        let addr = format!("{}:{}", ip, port);
        //let ipv4 = Ipv4Addr::from_str(ip.as_str()).expect("TODO: Failed to parse vision server ip");
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind the UDP Socket");
        /*socket
        .join_multicast_v4(&ipv4, &Ipv4Addr::UNSPECIFIED)
        .expect("Error to join multicast group");*/
        socket
            .set_nonblocking(true)
            .expect("Failed to set non blocking");

        Self {
            socket,
            buf: [0u8; BUFFER_SIZE],
            addr,
        }
    }

    pub fn step(&mut self, _commands_wrapper: &mut CommandsWrapper, store: &DataStore) {
        debug!("{:#?}", store);
        let json = serde_json::to_vec(&store).unwrap();
        self.socket
            .send_to(json.as_slice(), self.addr.as_str())
            .unwrap();

        if let Ok(p_size) = self.socket.recv(&mut self.buf) {
            let req: Vec<Command> =
                serde_json::from_slice(&self.buf[0..p_size]).expect("WHELP SOCKET ?");
            dbg!(req);
        }
    }
}
