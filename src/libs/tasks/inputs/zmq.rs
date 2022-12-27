use crate::libs::cli::Cli;
use crate::libs::data::{DataStore};
use crate::libs::skills::kick::KickType;
use crate::libs::tasks::task::Task;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use zmq::{Socket, DONTWAIT};

pub struct ZmqInputTask {
    socket: Socket,
}

// TODO : Necessary ?
impl Default for ZmqInputTask {
    fn default() -> Self {
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::REP).unwrap();
        socket.set_rcvtimeo(1).expect("Failed to set rcv timeout");
        socket.set_sndtimeo(1).expect("Failed to set snd timeout");
        socket.bind("tcp://127.0.0.1:7558").unwrap();

        Self { socket }
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
    Leds { r: u8, g: u8, b: u8 },

    Dribble { speed: f32 },
}

#[derive(Deserialize)]
pub struct ZmqInputTaskReq {
    key: String, // TODO : Remove this key
    color: String,
    number: i32,
    // command: String,
    params: Command,
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

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        let mut msg = zmq::Message::new();

        if let Ok(_) = self.socket.recv(&mut msg, DONTWAIT) {
            debug!("Received {}", msg.as_str().unwrap());
            let req: ZmqInputTaskReq = serde_json::from_str(msg.as_str().unwrap()).unwrap();
            let rep = process_command(req, data_store);
            let rep_payload = serde_json::to_string(&rep).unwrap();
            self.socket.send(rep_payload.as_str(), DONTWAIT).unwrap();
        }

        Ok(())
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
                        true => KickType::Chip,
                        false => KickType::Straight,
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
            Command::Leds { .. } => {
                error!("ROBOTS DON'T EVEN HAVE LEDS");
                response.message = "Robots don't have leds ..".to_string();
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
