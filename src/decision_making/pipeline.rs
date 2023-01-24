use crate::decision_making::commands_wrapper::CommandsWrapper;
use crate::decision_making::plankton::Plankton;
use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{Command, DataStore};
use log::error;

//TODO: Add possibility to doesn't run plankton (in cli, by default deactivate plankton)
pub struct DecisionToolsPipeline {
    plankton: Plankton,
}

impl DecisionToolsPipeline {
    pub fn with_cli(cli: &Cli) -> Self {
        Self {
            plankton: Plankton::with_cli(cli),
        }
    }

    pub fn run(&mut self, data_store: &DataStore) -> [Option<Command>; NUMBER_OF_ROBOTS] {
        let mut command_wrapper = CommandsWrapper::new();
        // 1. Tools
        // Plankton
        if let Err(e) = self.plankton.step(&mut command_wrapper, data_store) {
            error!("{}", e);
        }
        // Center Control

        // 2. Here put decision making

        command_wrapper.into_inner()
    }
}
