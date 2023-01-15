use crate::libs::cli::Cli;
use crate::libs::data::DataStore;
use crate::libs::tasks::inputs::input::FilterStore;

pub trait FilterTask {
    fn with_cli(cli: &mut Cli) -> Box<Self> where Self: Sized;
    fn step(&self, store: &mut FilterStore, data_store: &mut DataStore);
}