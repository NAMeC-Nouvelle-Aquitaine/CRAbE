use crate::inputs_outputs::guard::Guard;
use crate::inputs_outputs::guard::limit_speed::LimitSpeed;
use crate::inputs_outputs::simulation_client::SimulationClient;
use crate::inputs_outputs::usb_client::USBClient;
use crate::libs::cli::Cli;
use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::data::{Command, DataStore};
use crate::libs::pipeline::Pipeline;
use crate::libs::robot::AllyRobotInfo;

pub trait OutputCommandSending {
    fn with_cli(cli: &Cli) -> Box<Self>
    where
        Self: Sized;
    fn step(
        &mut self,
        commands: [Option<Command>; NUMBER_OF_ROBOTS],
    ) -> [Option<AllyRobotInfo>; NUMBER_OF_ROBOTS];
}

pub struct OutputTask {
    client: Box<dyn OutputCommandSending>,
    guards: Pipeline<dyn Guard>,
}

impl OutputTask {
    pub fn with_cli(cli: &Cli) -> Self {
        Self {
            client: if cli.real {
                USBClient::with_cli(cli)
            } else {
                SimulationClient::with_cli(cli)
            },
            guards: vec![Box::new(LimitSpeed::with_cli(&cli))],
        }
    }

    pub fn run(
        &mut self,
        _data_store: &mut DataStore,
        mut commands: [Option<Command>; NUMBER_OF_ROBOTS],
    ) -> [Option<AllyRobotInfo>; NUMBER_OF_ROBOTS] {
        commands
            .iter_mut()
            .filter_map(|c| c.as_mut())
            .for_each(|mut c| self.guards.iter().for_each(|g| g.guard(&mut c)));

        self.client.step(commands)
    }
}
