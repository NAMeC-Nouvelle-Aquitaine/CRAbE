use std::sync::mpsc;
use clap::Args;
use crate::inputs_outputs::multicast_client::MulticastClient;
use crate::libs::cli::Cli;
use crate::libs::protobuf::vision_packet::SslWrapperPacket;

// TODO : Make port, address, interface for multicast to be changed

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
    multicast_client:MulticastClient<SslWrapperPacket>
}

impl Vision {
    pub fn with_cli(tx: mpsc::Sender<SslWrapperPacket>, cli: &mut Cli) -> Self {
        Self { multicast_client: MulticastClient::with_cli(tx, cli.vision.vision_ip.clone(), cli.vision.vision_port.clone()) }
    }

    pub fn run(&mut self) {
        self.multicast_client.run()
    }
}