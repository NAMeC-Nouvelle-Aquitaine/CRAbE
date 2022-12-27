use crate::libs::cli::Cli;
use crate::libs::data::DataStore;
use crate::libs::protobuf::simulation_packet::robot_move_command::Command;
use crate::libs::protobuf::simulation_packet::{MoveLocalVelocity, RobotCommand, RobotMoveCommand};
use crate::libs::tasks::task::Task;

#[derive(Default)]
pub struct MoveForwardExampleTask;

impl Task for MoveForwardExampleTask {
    fn with_cli(_cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        // Create robot command
        let mut r = RobotCommand::default();
        r.id = 0;

        // Move Local Velocity
        let mut move_robot = MoveLocalVelocity::default();
        move_robot.forward = 1.0;

        let command = Command::LocalVelocity(move_robot);

        r.move_command = Some(RobotMoveCommand {
            command: Some(command),
        });

        if let Some(robot) = data_store.allies.get_mut(0) {
            robot.command = Some(r);
        }

        Ok(())
    }
}
