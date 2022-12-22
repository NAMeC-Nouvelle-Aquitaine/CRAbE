use crate::libs::data::ControllableRobot;
use crate::libs::skills::kick::KickType::Straight;
use log::info;
use nalgebra::{Point2, Vector3};

impl ControllableRobot {
    pub(crate) fn pass(&mut self, ball: &Point2<f32>, receiver: &mut ControllableRobot) {
        // sender goes to ball and shoots it towards receiver
        self.shoot_to(ball, &receiver.robot.position);

        // receiver looks at ball, ready to receive
        let receiver_to_ball = ball - receiver.robot.position;
        let receiver_to_ball_angle = receiver_to_ball.y.atan2(receiver_to_ball.x);
        receiver.goto(Vector3::new(
            receiver.robot.position.x,
            receiver.robot.position.y,
            receiver_to_ball_angle,
        ));
    }

    fn shoot_to(&mut self, ball: &Point2<f32>, pos: &Point2<f32>) {
        let robot_to_ball = ball - self.robot.position;
        let ball_to_pos = pos - ball;

        self.dribble(1000.0);
        if self.goto(Vector3::new(
            ball.x,
            ball.y,
            robot_to_ball.y.atan2(robot_to_ball.x),
        )) {
            info!("PASS - at ball: {}", true);

            let angle = ball_to_pos.y.atan2(ball_to_pos.x);
            if self.goto(Vector3::new(ball.x, ball.y, angle)) {
                if (self.robot.orientation - angle).abs() < 0.05 {
                    self.dribble(0.0);
                    self.kick(Straight, 0.5);
                }
            }
        }
    }
}
