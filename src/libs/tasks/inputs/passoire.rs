use crate::libs::cli::Cli;
use crate::libs::data::{ControllableRobot, Field, Robot, TeamColor};
use crate::libs::protobuf::vision_packet::{SslDetectionRobot, SslWrapperPacket};
use crate::libs::{data, tasks};
use clap::Args;
use data::DataStore;
use log::{error, log, trace, warn};
use prost::Message;
use std::io::Cursor;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use tasks::task::Task;

// TODO : Rename buf to vision_buf
// TODO : Remove the vision_ip and add vision interface on cli option
const BUFFER_SIZE: usize = 4096;

pub struct VisionGcFilterInputTask {
    vision_thread: JoinHandle<()>,
    rx: Receiver<SslWrapperPacket>,
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

pub struct VisionThread {
    socket: UdpSocket,
    vision_buf: [u8; BUFFER_SIZE],
    tx: Sender<SslWrapperPacket>,
}

impl VisionGcFilterInputTask {}

impl VisionThread {
    fn with_cli(cli: &mut Cli, tx: mpsc::Sender<SslWrapperPacket>) -> Self {
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
            vision_buf: [0u8; BUFFER_SIZE],
            tx,
        }
    }
}

impl Task for VisionGcFilterInputTask {
    fn with_cli(cli: &mut Cli) -> Self {
        let (tx, rx) = mpsc::channel::<SslWrapperPacket>();

        let mut vision = VisionThread::with_cli(cli, tx);
        let vision_thread = std::thread::spawn(move || {
            let sock = vision.socket;

            loop {
                if let Ok(p_size) = sock.recv(&mut vision.vision_buf) {
                    let packet =
                        SslWrapperPacket::decode(Cursor::new(&vision.vision_buf[0..p_size]))
                            .expect("Error - Decoding the packet");

                    vision.tx.send(packet).expect("TODO: panic message");
                }
            }
        });

        Self { vision_thread, rx }
    }

    fn run(&mut self, data_store: &mut DataStore) {
        let (allies, enemies) = (&mut data_store.allies, &mut data_store.enemies);
        while let Ok(packet) = self.rx.try_recv() {
            match &packet.detection {
                None => {}
                Some(detection_frame) => {
                    if let Some(ball) = detection_frame.balls.get(0) {
                        data_store.ball.x = ball.x / 1000.0;
                        data_store.ball.y = ball.y / 1000.0;
                    }

                    let (robots_blue, robots_yellow) =
                        (&detection_frame.robots_blue, &detection_frame.robots_yellow);

                    // TODO: bounds check
                    match data_store.color {
                        TeamColor::YELLOW => {
                            self.update_robots(robots_yellow, robots_blue, allies, enemies)
                        }
                        TeamColor::BLUE => {
                            self.update_robots(robots_blue, robots_yellow, allies, enemies)
                        }
                    }
                }
            }

            // TODO : Move this on another filter
            match &packet.geometry {
                None => {}
                Some(geometry) => {
                    data_store.field = Some(Field {
                        length: geometry.field.field_length as f32 / 1000.0,
                        width: geometry.field.field_width as f32 / 1000.0,
                        goal_width: geometry.field.goal_width as f32 / 1000.0,
                        goal_depth: geometry.field.goal_depth as f32 / 1000.0,
                        center_radius: geometry.field.center_circle_radius.unwrap_or(500) as f32
                            / 1000.0, // TODO : Calculate the default with arcs
                        penalty_depth: geometry.field.penalty_area_depth.unwrap_or(1000) as f32
                            / 1000.0, // TODO : Calculate the default with arcs
                        penalty_width: geometry.field.penalty_area_width.unwrap_or(2000) as f32
                            / 1000.0, // TODO : Calculate the default with arcs
                    });
                }
            }
        }
    }
}

impl VisionGcFilterInputTask {
    fn update_robots(
        &self,
        allies: &Vec<SslDetectionRobot>,
        enemies: &Vec<SslDetectionRobot>,
        store_allies: &mut [ControllableRobot],
        store_enemies: &mut [Robot],
    ) {
        allies
            .into_iter()
            .filter(|r| r.robot_id.is_some())
            .for_each(
                |r| match store_allies.get_mut(r.robot_id.unwrap() as usize) {
                    None => {
                        error!(
                            "invalid robot id {} in detection packet",
                            r.robot_id.unwrap()
                        );
                    }
                    Some(robot) => {
                        robot.update_pose(r);
                    }
                },
            );

        enemies
            .into_iter()
            .filter(|r| r.robot_id.is_some())
            .for_each(
                |r| match store_enemies.get_mut(r.robot_id.unwrap() as usize) {
                    None => {
                        error!(
                            "invalid robot id {} in detection packet",
                            r.robot_id.unwrap()
                        );
                    }
                    Some(robot) => {
                        robot.update_pose(r);
                    }
                },
            );
    }
}
