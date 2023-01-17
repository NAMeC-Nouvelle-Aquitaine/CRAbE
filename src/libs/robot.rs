use crate::libs::protobuf::vision_packet::SslDetectionRobot;
use nalgebra::Point2;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Robot {
    pub id: u32,
    pub position: Point2<f32>,
    pub orientation: f32,
}

impl Robot {
    // TODO : Move this directly on the filter ?
    pub fn update_pose(&mut self, robot_detection_packet: &SslDetectionRobot) {
        self.id = robot_detection_packet.robot_id.unwrap();
        self.position.x = robot_detection_packet.x / 1000.0;
        self.position.y = robot_detection_packet.y / 1000.0;
        if let Some(orientation) = robot_detection_packet.orientation {
            self.orientation = orientation;
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnemyRobot {
    pub(crate) robot: Robot,
    info: Option<EnemyRobotInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AllyRobot {
    pub(crate) robot: Robot,
    pub(crate) info: Option<AllyRobotInfo>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AllyRobotInfo {
    /// Has the dribbler contact to the ball
    pub(crate) has_ball: bool,
    // TODO: battery
    // TODO: odometry
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnemyRobotInfo {
    /// Has the dribbler contact to the ball
    has_ball: bool,
}
