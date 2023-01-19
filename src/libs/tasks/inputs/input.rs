use crate::filters::detections::DetectionFilter;
use crate::filters::filter::FilterTask;
use crate::filters::game_controller::GameControllerFilter;
use crate::filters::geometry::GeometryFilter;
use crate::inputs_outputs::game_controller::GameController;
use crate::inputs_outputs::vision::Vision;
use crate::libs::cli::Cli;
use crate::libs::data::DataStore;
use crate::libs::pipeline::Pipeline;
use crate::libs::protobuf::game_controller_packet::Referee;
use crate::libs::protobuf::vision_packet::SslWrapperPacket;
use log::info;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

#[derive(Default)]
pub struct FilterStore {
    pub vision_packet: Vec<SslWrapperPacket>,
    pub gc_packet: Vec<Referee>,
}

pub struct VisionGcFilterInputTask {
    // vision_thread: JoinHandle<()>,
    filter_store: FilterStore,
    pub rx_vision: Receiver<SslWrapperPacket>,
    pub rx_gc: Receiver<Referee>,
    pipeline: Pipeline<dyn FilterTask>,
    is_gc: bool,
}

impl VisionGcFilterInputTask {
    pub fn with_cli(mut cli: &Cli) -> Self {
        let (tx_vision, rx_vision) = mpsc::channel::<SslWrapperPacket>();
        let (tx_gc, rx_gc) = mpsc::channel::<Referee>();

        let mut vision = Vision::with_cli(tx_vision, cli);
        std::thread::spawn(move || {
            info!("vision thread started");

            loop {
                vision.run();
            }
        });

        if cli.game_controller {
            let mut gc = GameController::with_cli(tx_gc, cli);
            std::thread::spawn(move || {
                info!("gc thread started");

                loop {
                    gc.run();
                }
            });
        }

        let filter_store = FilterStore::default();

        let mut task_pipeline: Pipeline<dyn FilterTask> = vec![
            GeometryFilter::with_cli(&cli),
            DetectionFilter::with_cli(&cli),
        ];

        if cli.game_controller {
            task_pipeline.push(GameControllerFilter::with_cli(&cli));
        }

        Self {
            rx_vision,
            rx_gc,
            filter_store,
            pipeline: task_pipeline,
            is_gc: cli.game_controller,
        }
    }

    pub fn run(&mut self, data_store: &mut DataStore) {
        // TODO : Here ? No we want to put on task !
        self.filter_store.vision_packet.clear();
        self.filter_store.gc_packet.clear();

        // TODO : Do we want to put on a task ?
        // TODO : Do we want to put a max recv packet ?
        self.filter_store
            .vision_packet
            .extend(self.rx_vision.try_iter());
        if self.is_gc {
            self.filter_store.gc_packet.extend(self.rx_gc.try_iter());
        }

        for task in &self.pipeline {
            task.step(&mut self.filter_store, data_store);
        }
    }
}
