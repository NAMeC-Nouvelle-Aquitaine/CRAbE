use crate::inputs_outputs::simulation_client::SimulationClient;
use crate::inputs_outputs::usb_client::USBClient;
use crate::libs::cli::Cli;
use crate::libs::data::{Command, DataStore};
use crate::libs::robot::AllyRobotInfo;

pub trait OutputCommandSending {
    fn with_cli(cli: &mut Cli) -> Box<Self>
    where
        Self: Sized;
    fn step(&mut self, commands: &Vec<Command>) -> Vec<AllyRobotInfo>;
}

pub struct OutputTask {
    client: Box<dyn OutputCommandSending>,
    // pipeline: Pipeline<dyn FilterTask>,
}

impl OutputTask {
    pub fn with_cli(cli: &mut Cli) -> Self {
        Self {
            client: if cli.real {
                USBClient::with_cli(cli)
            } else {
                SimulationClient::with_cli(cli)
            },
        }
    }

    pub fn run(&mut self, _data_store: &mut DataStore, commands: Vec<Command>) {
        // 1. Guard

        // TODO : Speed Limit

        // 2. Send the commands
        self.client.step(&commands);
    }
}
