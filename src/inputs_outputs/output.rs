use crate::inputs_outputs::simulation_client::SimulationClient;
use crate::inputs_outputs::ubs_client::USBClient;
use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{Command, DataStore};
use crate::libs::robot::AllyRobotInfo;
use crate::libs::tasks::task::Task;
use clap::command;
use std::net::UdpSocket;

pub trait OutputCommandSending {
    fn with_cli(cli: &mut Cli) -> Self;
    fn step(&self, commands: Vec<Command>) -> AllyRobotInfo;
}

pub struct OutputTask {
    client: OutputCommandSending,
    // pipeline: Pipeline<dyn FilterTask>,
}

impl Task for OutputTask {
    fn with_cli(mut cli: &mut Cli) -> Self {
        Self {
            client: if cli.real {
                USBClient
            } else {
                SimulationClient
            },
        }
    }

    fn run(&mut self, data_store: &mut DataStore) {
        // 1. Guard
        // 2. Send the commands
        self.client.send(&data_store.commands);
    }
}
