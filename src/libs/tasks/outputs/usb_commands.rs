use crate::libs::cli::Cli;
use crate::libs::data::DataStore;
use crate::libs::tasks::task::Task;

#[derive(Default)]
pub struct UsbCommandsOutputTask;

impl Task for UsbCommandsOutputTask {
    fn with_cli(cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        for robot in data_store.allies.iter_mut() {
            if let Some(_cmd) = &robot.command {
                //TODO: add usb code
            }
        }

        Ok(())
    }
}
