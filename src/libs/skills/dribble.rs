use crate::libs::data::ControllableRobot;
use crate::libs::protobuf::simulation_packet::RobotCommand;

impl ControllableRobot {
    pub(crate) fn dribble(&mut self, status: bool) {
        if self.command.is_none() {
            self.command = Some(RobotCommand::default());
        }

        if let Some(cmd) = &mut self.command {
            cmd.dribbler_speed = Some(if status { 1000.0 } else { 0.0 });
        }
    }
}
