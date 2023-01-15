use crate::libs::cli::Cli;
use crate::libs::protobuf::game_controller_packet::Referee;
use prost::Message;
use std::io::Cursor;
use std::net::{Ipv4Addr, UdpSocket};
use std::sync::mpsc;

const BUFFER_SIZE: usize = 4096;

pub struct GameControllerInputTask {
    socket: UdpSocket,
    gc_buf: [u8; BUFFER_SIZE],
    tx: mpsc::Sender<Referee>,

}

impl GameControllerInputTask {
    pub fn with_cli(tx: mpsc::Sender<Referee>, _cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        let ipv4 = Ipv4Addr::new(224, 5, 23, 1);
        let socket = UdpSocket::bind("224.5.23.1:10003").expect("Failed to bind the UDP Socket");
        socket
            .join_multicast_v4(&ipv4, &Ipv4Addr::UNSPECIFIED)
            .expect("Error to join multicast group");
        socket
            .set_nonblocking(true)
            .expect("Failed to set non blocking");

        Self {
            socket,
            gc_buf: [0u8; BUFFER_SIZE],
            tx,
        }
    }

    pub fn run(&mut self) {
        if let Ok(p_size) = self.socket.recv(&mut self.gc_buf) {
            let packet = Referee::decode(Cursor::new(&self.gc_buf[0..p_size]))
                .expect("Error - Decoding the packet");


            self.tx.send(packet).expect("TODO: panic message");
        }
    }
}
