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
use crate::filters::detections::DetectionFilter;
use crate::filters::filter::FilterTask;
use crate::filters::geometry::GeometryFilter;
use crate::inputs_outputs::vision::Vision;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::protobuf::tools_packet::Ball;

// TODO : Make port, address, interface for multicast to be changed

#[derive(Default)]
pub struct FilterStore {
    pub(crate) vision_packet: Vec<SslWrapperPacket>,
}

pub struct VisionGcFilterInputTask {
    vision_thread: JoinHandle<()>,
    filter_store: FilterStore,
    pub(crate) rx : Receiver<SslWrapperPacket>,
    pipeline: Vec<Box<dyn FilterTask>>,
}

impl Task for VisionGcFilterInputTask {
    fn with_cli(mut cli: &mut Cli) -> Self {
        let (tx, rx) = mpsc::channel::<SslWrapperPacket>();

        let mut vision = Vision::with_cli(tx, cli);
        let vision_thread = std::thread::spawn(move || {
            loop {
                vision.run();
            }
        });

        let filter_store = FilterStore::default();

        let task_pipeline: Vec<Box<dyn FilterTask>> = vec![
            GeometryFilter::with_cli(&mut cli),
            DetectionFilter::with_cli(&mut cli),
        ];

        Self { vision_thread, rx, filter_store, pipeline: task_pipeline}
    }

    fn run(&mut self, data_store: &mut DataStore) {
        // TODO : Do we want to put on a task ?
        // TODO : Do we want to put a max recv packet ?
        while let Ok(packet) = self.rx.try_recv() {
            self.filter_store.vision_packet.push(packet);
        }

        for task in &self.pipeline {
            task.step(&mut self.filter_store, data_store);
        }
    }
}
