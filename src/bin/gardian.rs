use std::sync::mpsc::channel;
use clap::Parser;
use software::libs::cli::Cli;
use software::libs::data::DataStore;
use software::libs::pipeline::{run_pipeline, Pipeline};

use software::libs::tasks::filters::passoire::PassoireFilterTask;
use software::libs::tasks::inputs::game_controller::GameControllerInputTask;
use software::libs::tasks::inputs::vision::VisionInputTask;
use software::libs::tasks::outputs::sim_commands::SimCommandsOutputTask;
use software::libs::tasks::outputs::usb_commands::UsbCommandsOutputTask;
use software::libs::tasks::task::Task;

#[macro_use]
extern crate log;
use env_logger::Env;
use software::libs::tasks::inputs::zmq::ZmqInputTask;
use software::libs::tasks::inputs_outputs::tools::ToolsInputOutputTask;
use software::libs::tasks::outputs::zmq::ZmqOutputTask;

// TODO : Make port, address, interface for multicast to be changed

fn vision_filter_init() {
    // Channel
    vision_thread.spawn();
    // FilterDataStore data;
    let pipeline_filter = vec![]; // Pipeline filter task
}

fn strategy_pipeline_init() { // commands
    // Strategy
    let strategy_pipeline = vec![
        // Striker::default...,
    ];
}

fn output_pipeline_init() {
    let pipeline_output = vec![]; // Garde | Output
}

/*impl Task {
    fn run(data: &DataStore) {
        pipeline.iter_mut().for_each(|task| {
            task.run(&mut data_store); // step
        });
    }
}
*/
fn main() {
    // INIT
    vision_filter_init();
    strategy_pipeline_init();
    output_pipeline_init();
    let mut data = DataStore::default();

    loop {
        // vision_gc_filter_run(&data); // pipeline
        // commands = strategy_pipeline_run(&data); // pipeline
        // output_pipeline_run(&data, &commands); // pipeline
    }

    // Autre proposition

    let mut pipeline = vec![];
    // vision_filter_init(&pipeline); // pipeline rempli par l'objet
    // strategy_pipeline_init(&pipeline); // pipeline rempli par l'objet
    // output_pipeline_init(&pipeline); // pipeline rempli par l'objet

    let mut data = DataStore::default();

    // pipeline.run(&pipeline, &data);
}
