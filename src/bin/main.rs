use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::{Duration, Instant};
use clap::{Command, Parser};
use software::libs::cli::Cli;
use software::libs::data::DataStore;

#[macro_use]
extern crate log;

use env_logger::Env;
use software::decision_making::pipeline::DecisionToolsPipeline;
use software::inputs_outputs::output::OutputTask;
use software::libs::constants::NUMBER_OF_ROBOTS;
use software::libs::tasks::inputs::input::VisionGcFilterInputTask;

fn main() {
    // Init the environnement
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    // Handle ctrl+c
    let running = Arc::new(AtomicBool::new(true));
    let shutdown = running.clone();

    ctrlc::set_handler(move || {
        shutdown.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut cli = Cli::parse();
    let mut data_store = DataStore::default();

    let mut input = VisionGcFilterInputTask::with_cli(&mut cli);
    let mut decision_tools = DecisionToolsPipeline::with_cli(&mut cli);
    let mut output = OutputTask::with_cli(&mut cli);

    while running.load(Ordering::SeqCst) {
        let start = Instant::now();

        // 1. Input the systems
        input.run(&mut data_store);
        let commands = decision_tools.run(&DataStore,);
        output.run(&mut data_store, commands);


        let elapsed: f64 = start.elapsed().as_millis() as f64;
        let sleep_time = Duration::from_millis(15).as_millis() as i64 - start.elapsed().as_millis() as i64;
        if sleep_time > 0 { sleep(Duration::from_millis(sleep_time as u64)); }

        if elapsed > max { max = elapsed; }
        if elapsed < min { min = elapsed; }

        trace!(
            "pipeline took {:>6} ms, max: {:>6} ms, min: {:>6} ms",
            elapsed,
            max,
            min
        );
    }
}
