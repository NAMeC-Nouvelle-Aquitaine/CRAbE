use crate::inputs_outputs::output::OutputCommandSending;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::Command;
use crate::libs::robot::AllyRobotInfo;

pub struct USBClient;

impl OutputCommandSending for USBClient {
    fn send(commands: [Command; NUMBER_OF_ROBOTS]) -> AllyRobotInfo {
        todo!()
    }
}
