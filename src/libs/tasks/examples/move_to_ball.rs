use crate::libs::cli::Cli;
use crate::libs::data::DataStore;
use crate::libs::skills::kick::KickType;
use crate::libs::tasks::task::Task;
use nalgebra::Vector3;

#[derive(Default)]
pub struct MoveToBallExampleTask;

impl Task for MoveToBallExampleTask {
    fn with_cli(cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        let robot_to_ball = data_store.ball - data_store.allies[0].robot.position;
        let angle_to_ball = robot_to_ball.y.atan2(robot_to_ball.x);

        data_store.allies[0].goto(Vector3::new(
            data_store.ball.x,
            data_store.ball.y,
            angle_to_ball,
        ));

        if robot_to_ball.norm() < 103.0 {
            data_store.allies[0].kick(KickType::Chip, 1.0);
            println!("KICKED ! (distance to ball: {})", robot_to_ball.norm());
        }

        Ok(())
    }
}
