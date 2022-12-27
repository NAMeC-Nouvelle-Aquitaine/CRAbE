use crate::libs::cli::Cli;
use crate::libs::protobuf::vision_packet::SslWrapperPacket;
use crate::libs::{data, tasks};
use clap::{Args};
use data::DataStore;
use log::{trace, warn};
use prost::Message;
use std::io::Cursor;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use std::time::{Duration, Instant};
use tasks::task::Task;

const BUFFER_SIZE: usize = 4096;

pub struct VisionInputTask {
    socket: UdpSocket,
    buf: [u8; BUFFER_SIZE],
}

#[derive(Args, Clone)]
pub struct VisionInputTaskCli {
    /// ip of the ssl vision server
    #[arg(long, default_value = "224.5.23.2")]
    vision_ip: String,

    /// port of the ssl vision server
    #[arg(long, default_value_t = 10020)]
    vision_port: u32,
}

impl Task for VisionInputTask {
    fn with_cli(cli: &mut Cli) -> Self {
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
            .set_read_timeout(Some(Duration::from_millis(15)))
            .expect("Failed to set read timeout");

        // socket
        //     .set_nonblocking(true)
        //     .expect("Failed to set non blocking");

        Self {
            socket,
            buf: [0u8; BUFFER_SIZE],
        }
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        // clear the old packets
        data_store.vision.clear();

        let start = Instant::now();
        let mut recieved_packets_count = 0;

        while let Ok(p_size) = self.socket.recv(&mut self.buf) {
            let packet = SslWrapperPacket::decode(Cursor::new(&self.buf[0..p_size]))
                .expect("Error - Decoding the packet");
            data_store.vision.push(packet);

            recieved_packets_count += 1;

            if recieved_packets_count > 50 {
                break;
            }
        }

        if recieved_packets_count == 0 {
            warn!("check that Vision or Simulation is running");
        } else {
            trace!(
                "grabbed a frame (cnt: {}), took {} us",
                recieved_packets_count,
                start.elapsed().as_micros()
            );
        }

        Ok(())
    }
}
