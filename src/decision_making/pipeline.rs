use crate::decision_making::command_wrapper::CommandWrapper;
use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{Command, DataStore};

pub struct DecisionToolsPipeline;

impl DecisionToolsPipeline {
    pub fn with_cli(_cli: &Cli) -> Self {
        Self {}
    }

    pub fn run(&mut self, _data_store: &DataStore) -> [Option<Command>; NUMBER_OF_ROBOTS] {
        let command_wrapper = CommandWrapper::new();
        // 1. Tools
        // ZMQ
        // Center Control

        // 2. Here put decision making
        return command_wrapper.into_inner();
    }
}
