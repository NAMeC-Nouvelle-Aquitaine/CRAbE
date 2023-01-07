use crate::libs::cli::Cli;
use crate::libs::data::DataStore;

pub trait Task {
    fn with_cli(cli: &mut Cli) -> Self
    where
        Self: Sized;

    fn run(&mut self, data_store: &mut DataStore);

    fn with_cli_boxed(cli: &mut Cli) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self::with_cli(cli))
    }
}
