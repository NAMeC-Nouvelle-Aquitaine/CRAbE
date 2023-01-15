use crate::libs::cli::Cli;
use crate::libs::protobuf::vision_packet::{SslWrapperPacket};
use crate::libs::{data, tasks};
use data::DataStore;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use log::info;
use tasks::task::Task;
use crate::filters::detections::DetectionFilter;
use crate::filters::filter::FilterTask;
use crate::filters::geometry::GeometryFilter;
use crate::inputs_outputs::vision::Vision;
use crate::libs::pipeline::Pipeline;

// TODO : Make port, address, interface for multicast to be changed

#[derive(Default)]
pub struct FilterStore {
    pub(crate) vision_packet: Vec<SslWrapperPacket>,
}

pub struct VisionGcFilterInputTask {
    // vision_thread: JoinHandle<()>,
    filter_store: FilterStore,
    pub(crate) rx : Receiver<SslWrapperPacket>,
    pipeline: Pipeline<dyn FilterTask>,
}

impl Task for VisionGcFilterInputTask {
    fn with_cli(mut cli: &mut Cli) -> Self {
        let (tx, rx) = mpsc::channel::<SslWrapperPacket>();

        let mut vision = Vision::with_cli(tx, cli);
        std::thread::spawn(move || {
            info!("vision thread started");

            loop {
                vision.run();
            }
        });

        let filter_store = FilterStore::default();

        let task_pipeline: Pipeline<dyn FilterTask> = vec![
            GeometryFilter::with_cli(&mut cli),
            DetectionFilter::with_cli(&mut cli),
        ];

        Self { rx, filter_store, pipeline: task_pipeline}
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
