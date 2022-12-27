use crate::libs::cli::Cli;
use crate::libs::data::DataStore;
use crate::libs::tasks::task::Task;

#[derive(Default)]
pub struct BallPrinterOutputTask;

// TODO : Remove this on the output
impl Task for BallPrinterOutputTask {
    fn with_cli(_cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        println!("{:?}", data_store.ball);
        println!("{:?}", data_store.allies[0].robot.position);

        Ok(())
    }
}
