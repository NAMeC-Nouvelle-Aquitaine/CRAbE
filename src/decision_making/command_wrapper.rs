use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::Command;
use log::{error, warn};

pub struct CommandWrapper {
    commands: [Option<Command>; NUMBER_OF_ROBOTS],
}

impl CommandWrapper {
    pub(crate) fn new() -> Self {
        Self {
            commands: Default::default(),
        }
    }

    pub(crate) fn add_command(&mut self, robot_id: usize, command: Command) {
        if let Some(robot_command) = self.commands.get_mut(robot_id) {
            if robot_command.is_some() {
                warn!("You give a command before for this robot, pay attention");
            }

            *robot_command = Some(command);
        } else {
            error!("invalid ally robot id {}", robot_id);
        }
    }

    pub(crate) fn into_inner(self) -> [Option<Command>; NUMBER_OF_ROBOTS] {
        self.commands
    }
}
