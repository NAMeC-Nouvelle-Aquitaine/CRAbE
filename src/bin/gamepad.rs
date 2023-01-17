use clap::Parser;
use software::libs::cli::Cli;
use software::libs::data::DataStore;
use software::libs::pipeline::{run_pipeline, Pipeline};

use software::libs::tasks::inputs::gamepad::GamepadInputTask;
use software::libs::tasks::task::Task;

#[macro_use]
extern crate log;
use env_logger::Env;
use software::inputs_outputs::output::OutputTask;

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "log")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    info!("starting up");

    let mut cli = Cli::parse();

    let mut data_store = DataStore::default();

    let mut pipeline: Pipeline<dyn Task> = vec![
        GamepadInputTask::with_cli_boxed(&mut cli),
        OutputTask::with_cli_boxed(&mut cli),
    ];

    run_pipeline(&mut data_store, &mut pipeline);
}
