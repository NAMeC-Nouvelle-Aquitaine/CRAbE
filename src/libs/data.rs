use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::protobuf::game_controller_packet::Referee;
use crate::libs::protobuf::simulation_packet::{RobotCommand, RobotFeedback};
use crate::libs::protobuf::vision_packet::{SslDetectionRobot, SslWrapperPacket};
use nalgebra::Point2;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct DataStore {
    pub color: TeamColor,
    pub blue_on_positive_half: bool,
    pub ball: Point2<f32>,
    pub allies: [ControllableRobot; NUMBER_OF_ROBOTS],
    pub enemies: [Robot; NUMBER_OF_ROBOTS],
    pub vision: Vec<SslWrapperPacket>,
    pub game_controller: Option<Referee>,
    pub field: Option<Field>,
}

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
pub struct Field {
    pub width: f32,
    pub length: f32,
    pub goal_width: f32,
    pub goal_depth: f32,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum TeamColor {
    BLUE,
    YELLOW,
}

impl TeamColor {
    // TODO: properly implement Display
    pub fn to_string(&self) -> String {
        match self {
            TeamColor::BLUE => "blue".to_string(),
            TeamColor::YELLOW => "yellow".to_string(),
        }
    }
}

impl Default for TeamColor {
    fn default() -> Self {
        TeamColor::BLUE
    }
}

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
pub struct Robot {
    pub position: Point2<f32>,
    pub orientation: f32,
}

impl Robot {
    pub fn update_pose(&mut self, robot_detection_packet: &SslDetectionRobot) {
        self.position.x = robot_detection_packet.x;
        self.position.y = robot_detection_packet.y;
        if let Some(orientation) = robot_detection_packet.orientation {
            self.orientation = orientation;
        }
    }
}

#[derive(Default, Clone)]
pub struct ControllableRobot {
    pub robot: Robot,
    pub command: Option<RobotCommand>,
    pub feedback: Option<RobotFeedback>,
}

impl ControllableRobot {
    pub fn update_pose(&mut self, robot_detection_packet: &SslDetectionRobot) {
        self.robot.update_pose(robot_detection_packet);
    }
}
