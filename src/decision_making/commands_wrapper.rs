use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::Command;
use log::{error, warn};

pub struct CommandsWrapper {
    commands: [Option<Command>; NUMBER_OF_ROBOTS],
}

impl CommandsWrapper {
    pub(crate) fn new() -> Self {
        Self {
            commands: Default::default(),
        }
    }

    pub fn add_command(&mut self, robot_id: usize, command: Command) {
        match self.commands.get_mut(robot_id) {
            None => {
                error!("invalid ally robot id {}", robot_id);
            }
            Some(mut robot_command) => {
                if robot_command.is_some() {
                    warn!("You give a command before for this robot, pay attention");
                }
                *robot_command = Some(command)
            }
        }
    }

    pub(crate) fn into_inner(self) -> [Option<Command>; NUMBER_OF_ROBOTS] {
        self.commands
    }
}
