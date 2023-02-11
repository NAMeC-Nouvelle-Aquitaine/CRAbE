use clap::Parser;
use crabe_ssl::libs::cli::Cli;
use crabe_ssl::libs::data::{Command, DataStore};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[macro_use]
extern crate log;

use env_logger::Env;
use crabe_ssl::inputs_outputs::output::OutputTask;
use crabe_ssl::libs::constants::NUMBER_OF_ROBOTS;
use crabe_ssl::libs::tasks::inputs::gamepad::GamepadInputTask;

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
    })
    .expect("Error setting Ctrl-C handler");

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let cli = Cli::parse();
    let mut data_store = DataStore::default();

    let mut gamepad = GamepadInputTask::with_cli(&cli);
    let mut output = OutputTask::with_cli(&cli);

    while running.load(Ordering::SeqCst) {
        let start = Instant::now();

        let mut commands: [Option<Command>; NUMBER_OF_ROBOTS] = Default::default();
        commands[0] = Some(gamepad.run(&data_store));
        output.run(&mut data_store, commands);

        let elapsed = start.elapsed().as_micros() as f64 / 1000.0;
        let sleep_time = Duration::from_millis(16).as_micros() - start.elapsed().as_micros();
        if sleep_time > 0 {
            sleep(Duration::from_micros(sleep_time as u64));
        }

        if elapsed > max {
            max = elapsed;
        }
        if elapsed < min {
            min = elapsed;
        }

        trace!(
            "pipeline took {:>6} ms, max: {:>6} ms, min: {:>6} ms",
            elapsed,
            max,
            min
        );
    }
}
