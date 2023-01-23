use crate::libs::robot::Robot;
use nalgebra::{Isometry2, RealField, Vector2};

pub fn frame(x: f32, y: f32, orientation: f32) -> Isometry2<f32> {
    Isometry2::new(Vector2::new(x, y), orientation)
}

pub fn frame_inv(frame: Isometry2<f32>) -> Isometry2<f32> {
    frame.inverse()
}

pub fn robot_frame(robot: &Robot) -> Isometry2<f32> {
    frame(robot.position.x, robot.position.y, robot.orientation)
}

pub fn angle_wrap(alpha: f32) -> f32 {
    (alpha + f32::pi()) % (2.0 * f32::pi()) - f32::pi()
}
