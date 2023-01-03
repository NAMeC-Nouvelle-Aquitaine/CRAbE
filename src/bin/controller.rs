use clap::Parser;
use software::libs::cli::Cli;
use software::libs::data::DataStore;
use software::libs::pipeline::{run_pipeline, Pipeline};

use software::libs::tasks::filters::passoire::PassoireFilterTask;
use software::libs::tasks::inputs::game_controller::GameControllerInputTask;
use software::libs::tasks::inputs::vision::VisionInputTask;
use software::libs::tasks::outputs::sim_commands::SimCommandsOutputTask;
use software::libs::tasks::task::Task;

#[macro_use]
extern crate log;
use env_logger::Env;
use software::libs::tasks::inputs::zmq::ZmqInputTask;
use software::libs::tasks::outputs::usb_commands::UsbCommandsOutputTask;
use software::libs::tasks::outputs::zmq::ZmqOutputTask;

// TODO : Make port, address, interface for multicast to be changed

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    info!("starting up");

    let mut cli = Cli::parse();
    cli.real = true;

    let mut data_store = DataStore::default();

    let mut pipeline: Pipeline = vec![if cli.real {
        UsbCommandsOutputTask::with_cli_boxed(&mut cli)
    } else {
        SimCommandsOutputTask::with_cli_boxed(&mut cli)
    }];

    if cli.game_controller {
        pipeline.push(GameControllerInputTask::with_cli_boxed(&mut cli))
    }

    run_pipeline(&mut data_store, &mut pipeline);
}
