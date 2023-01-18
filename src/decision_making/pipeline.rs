use crate::libs::cli::Cli;
use crate::libs::protobuf::vision_packet::{SslWrapperPacket};
use crate::libs::data::{Command, DataStore};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use log::info;
use crate::filters::detections::DetectionFilter;
use crate::filters::filter::FilterTask;
use crate::filters::game_controller::GameControllerFilter;
use crate::filters::geometry::GeometryFilter;
use crate::inputs_outputs::game_controller::GameController;
use crate::inputs_outputs::vision::Vision;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::pipeline::Pipeline;
use crate::libs::protobuf::game_controller_packet::Referee;

pub struct DecisionToolsPipeline {
}

impl DecisionToolsPipeline {
    pub fn with_cli(mut cli: &mut Cli) -> Self {
        Self {

        }
    }

    pub fn run(&mut self, data_store: &DataStore) -> Vec<Command> {
        let commands = vec![];

        // 1. Tools
        // ZMQ
        // Center Control

        // 2. Here put decision making

        commands
    }
}
