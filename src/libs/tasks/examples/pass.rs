use crate::libs::cli::Cli;
use crate::libs::data::DataStore;
use crate::libs::tasks::task::Task;

#[derive(Default)]
pub struct PassExampleTask;

impl Task for PassExampleTask {
    fn with_cli(_cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) {
        let [ref mut sender, ref mut receiver, ..] = data_store.allies;

        receiver.dribble(1000.0);
        sender.pass(&data_store.ball, receiver);
    }
}
