use crate::libs::cli::Cli;
use crate::libs::data::Command;

pub(crate) mod limit_speed;

pub trait Guard {
    fn with_cli(cli: &Cli) -> Self
    where
        Self: Sized;
    fn guard(&self, command: &mut Command);
}
