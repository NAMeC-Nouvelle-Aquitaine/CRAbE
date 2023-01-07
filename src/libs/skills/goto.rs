use crate::libs::data::ControllableRobot;
use crate::libs::maths;
use crate::libs::protobuf::simulation_packet::robot_move_command::Command;
use crate::libs::protobuf::simulation_packet::{MoveLocalVelocity, RobotCommand, RobotMoveCommand};
use nalgebra::{Point2, Vector3};
extern crate nalgebra as na;

const GOTO_SPEED_MULTIPLIER: f32 = 3.0;

impl ControllableRobot {
    pub(crate) fn goto(&mut self, target: Vector3<f32>) -> bool {
        let (_arrived, order) = self.goto_compute_order(target);
        self.control(order.x, order.y, order.z);
        _arrived
    }

    pub(crate) fn control(&mut self, dx: f32, dy: f32, dturn: f32) {
        if self.command.is_none() {
            self.command = Some(RobotCommand::default());
        }

        if let Some(cmd) = &mut self.command {
            let mut move_robot = MoveLocalVelocity::default();
            move_robot.forward = dx;
            move_robot.left = dy;
            move_robot.angular = dturn;

            cmd.move_command = Some(RobotMoveCommand {
                command: Some(Command::LocalVelocity(move_robot)),
            });
        }
    }

    fn goto_compute_order(&self, target: Vector3<f32>) -> (bool, Vector3<f32>) {
        let x = target.x;
        let y = target.y;
        let orientation = target.z;
        // # x = min(self.x_max, max(self.x_min, x))
        // # y = min(self.y_max, max(self.y_min, y))
        let ti = maths::frame_inv(maths::robot_frame(&self.robot));
        let target_in_robot: Point2<f32> = ti * Point2::new(x, y);

        let error_x = target_in_robot[0];
        let error_y = target_in_robot[1];
        let error_orientation = maths::angle_wrap(orientation - self.robot.orientation);

        let arrived = Vector3::new(error_x, error_y, error_orientation).norm() < 0.115;
        let order = Vector3::new(
            GOTO_SPEED_MULTIPLIER * error_x,
            GOTO_SPEED_MULTIPLIER * error_y,
            1.5 * error_orientation,
        );

        (arrived, order)
    }
}
