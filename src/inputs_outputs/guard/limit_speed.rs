use crate::inputs_outputs::guard::Guard;
use crate::libs::cli::Cli;
use crate::libs::data::Command;

pub struct LimitSpeed {
    max_linear: f32,
    max_angular: f32,
}

impl Guard for LimitSpeed {
    fn with_cli(cli: &Cli) -> LimitSpeed {
        Self {
            max_linear: 3.,
            max_angular: 3.14,
        } // TODO: Max speed from cli
    }
    fn guard(&self, command: &mut Command) {
        command.forward_velocity = command
            .forward_velocity
            .clamp(-self.max_linear, self.max_linear);
        command.left_velocity = command
            .left_velocity
            .clamp(-self.max_linear, self.max_linear);
        command.angular_velocity = command
            .angular_velocity
            .clamp(-self.max_angular, self.max_angular);
    }
}
