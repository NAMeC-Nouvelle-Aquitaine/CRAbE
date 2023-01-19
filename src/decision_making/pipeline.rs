use crate::decision_making::commands_wrapper::CommandsWrapper;
use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{Command, DataStore};


pub struct DecisionToolsPipeline {}

impl DecisionToolsPipeline {
    pub fn with_cli(_cli: &mut Cli) -> Self {
        Self {}
    }

    pub fn run(&mut self, _data_store: &DataStore) -> [Option<Command>; NUMBER_OF_ROBOTS] {
        let command_wrapper = CommandsWrapper::new();
        // 1. Tools
        // ZMQ
        // Center Control

        // 2. Here put decision making

        command_wrapper.get_commands();
    }
}
