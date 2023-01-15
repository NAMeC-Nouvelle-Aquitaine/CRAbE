use std::io::Cursor;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc;
use clap::Args;
use prost::Message;
use crate::libs::cli::Cli;
use crate::libs::protobuf::vision_packet::SslWrapperPacket;

// TODO : Make port, address, interface for multicast to be changed
// TODO : Move this in constant
const BUFFER_SIZE: usize = 4096;

#[derive(Args, Clone)]
pub struct VisionCli {
    /// ip of the ssl vision server
    #[arg(long, default_value = "224.5.23.2")]
    vision_ip: String,

    /// port of the ssl vision server
    #[arg(long, default_value_t = 10020)]
    vision_port: u32,
}


pub struct Vision {
    socket: UdpSocket,
    vision_buf: [u8; BUFFER_SIZE],
    tx: mpsc::Sender<SslWrapperPacket>,
}


impl Vision {
    pub fn with_cli(tx: mpsc::Sender<SslWrapperPacket>, cli: &mut Cli) -> Self {
        let ipv4 = Ipv4Addr::from_str(cli.vision.vision_ip.as_str())
            .expect("TODO: Failed to parse vision server ip");
        let socket = UdpSocket::bind(format!(
            "{}:{}",
            cli.vision.vision_ip, cli.vision.vision_port
        ))
            .expect("Failed to bind the UDP Socket");
        socket
            .join_multicast_v4(&ipv4, &Ipv4Addr::UNSPECIFIED)
            .expect("Error to join multicast group");
        socket
            .set_nonblocking(true)
            .expect("Failed to set non blocking");

        Self {
            socket,
            vision_buf: [0u8; BUFFER_SIZE],
            tx,
        }
    }

    pub fn run(&mut self) {
        if let Ok(p_size) = self.socket.recv(&mut self.vision_buf) {
            let packet =
                SslWrapperPacket::decode(Cursor::new(&self.vision_buf[0..p_size]))
                    .expect("Error - Decoding the packet");

            self.tx.send(packet).expect("TODO: panic message");
        }
    }
}