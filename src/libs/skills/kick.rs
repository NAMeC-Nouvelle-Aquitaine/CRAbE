use crate::libs::data::ControllableRobot;
use crate::libs::protobuf::simulation_packet::RobotCommand;

// as per ssl rules
const MAX_SPEED: f32 = 6.5;

pub enum KickType {
    Straight,
    Chip,
}

impl ControllableRobot {
    pub(crate) fn kick(&mut self, kick_type: KickType, kick_power: f32) {
        if self.command.is_none() {
            self.command = Some(RobotCommand::default());
        }

        if let Some(cmd) = &mut self.command {
            cmd.kick_speed = Some(kick_power * MAX_SPEED);

            match kick_type {
                KickType::Straight => {
                    cmd.kick_angle = Some(0.0);
                }
                KickType::Chip => {
                    cmd.kick_angle = Some(45.0);
                }
            }
        }
    }
}
