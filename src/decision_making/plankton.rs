use crate::decision_making::commands_wrapper::CommandsWrapper;
use crate::libs::cli::Cli;
use crate::libs::data::{DataStore, Kick};
use log::debug;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, UdpSocket};

const BUFFER_SIZE: usize = 4096;

pub struct Plankton {
    socket: UdpSocket,
    buf: [u8; BUFFER_SIZE],
    addr: String,
}

impl Plankton {
    pub fn with_cli(cli: &Cli) -> Self {
        let ip = "0.0.0.0".to_string();
        let port = 11301;
        let addr = format!("{}:{}", ip, port);
        //let ipv4 = Ipv4Addr::from_str(ip.as_str()).expect("TODO: Failed to parse vision server ip");
        let socket = UdpSocket::bind("0.0.0.0:11300").expect("Failed to bind the UDP Socket");

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
        self.socket.send_to(&json[..], self.addr.as_str()).unwrap();

        if let Ok(p_size) = self.socket.recv(&mut self.buf) {
            let rep = String::from_utf8(self.buf[0..p_size].to_vec()).unwrap();
            println!("{:?}", rep);

            //let req: Vec<Command> =
            //serde_json::from_slice(&self.buf[0..p_size]).expect("WHELP SOCKET ?");
            //dbg!(req);
        }
    }
}

#[derive(Deserialize)]
// #[serde(tag = "command", content = "params")]
#[serde(untagged)]
pub enum Command {
    // #[serde(rename(deserialize = "kick"))]
    Kick { power: f32, chip_kick: bool },
    // #[serde(rename(deserialize = "control"))]
    Control { dx: f32, dy: f32, dturn: f32 },
    // #[serde(rename(deserialize = "led"))]
    Dribble { speed: f32 },
}

#[derive(Deserialize)]
pub struct ZmqInputTaskReq {
    color: String,
    number: i32,
    // command: String,
    params: Command, // TODO : Send multiple commands at the same time
}

#[derive(Serialize)]
pub struct ZmqInputTaskRep {
    succeeded: bool,
    message: String,
}

impl Task for ZmqInputTask {
    fn with_cli(_cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) {
        let mut msg = zmq::Message::new();

        if let Ok(_) = self.socket.recv(&mut msg, DONTWAIT) {
            debug!("Received {}", msg.as_str().unwrap());
            let req: ZmqInputTaskReq = serde_json::from_str(msg.as_str().unwrap()).unwrap();
            let rep = process_command(req, data_store);
            let rep_payload = serde_json::to_string(&rep).unwrap();
            self.socket.send(rep_payload.as_str(), DONTWAIT).unwrap();
        }
    }
}

fn process_command(command: ZmqInputTaskReq, data_store: &mut DataStore) -> ZmqInputTaskRep {
    let mut response = ZmqInputTaskRep {
        succeeded: false,
        message: "Unknown error".to_string(),
    };

    let team: String = data_store.color.to_string();
    if command.color == team {
        match command.params {
            Command::Kick { power, chip_kick } => {
                data_store.allies[command.number as usize].kick(
                    match chip_kick {
                        true => Kick::ChipKick,
                        false => Kick::Straight,
                    },
                    power,
                );
                response.succeeded = true;
                response.message = "Ok".to_string();
            }
            Command::Control { dx, dy, dturn } => {
                data_store.allies[command.number as usize].control(dx, dy, dturn);
                response.succeeded = true;
                response.message = "Ok".to_string();
            }
            Command::Dribble { speed } => {
                data_store.allies[command.number as usize].dribble(speed);
                response.succeeded = true;
                response.message = "Ok".to_string();
            }
        }
    } else {
        response.message =
            format!("Unknown robot: {}{}", command.color, command.number).to_string();
    }

    response
}
