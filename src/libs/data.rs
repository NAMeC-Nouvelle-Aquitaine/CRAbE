use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::protobuf::game_controller_packet::Referee;
use nalgebra::Point2;
use serde::{Deserialize, Serialize};
use crate::libs::protobuf::vision_packet::SslDetectionRobot;

#[derive(Default)]
pub struct DataStore {
    pub color: TeamColor,
    pub blue_on_positive_half: bool,
    pub ball: Point2<f32>,
    pub allies: [Robot; NUMBER_OF_ROBOTS],
    pub enemies: [Robot; NUMBER_OF_ROBOTS], // TODO : Rename opponents
    pub game_controller: Option<Referee>,
    pub field: Option<Field>,
    pub commands: Vec<Command>,
    pub feedback: [Feedback; NUMBER_OF_ROBOTS]
}

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
pub struct Field {
    pub width: f32,
    pub length: f32,
    pub goal_width: f32,
    pub goal_depth: f32,
    pub penalty_depth: f32,
    pub penalty_width: f32,
    pub center_radius: f32,
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

#[derive(Default, Clone, Serialize)]
pub struct Feedback {
    /// ID of the robot
    pub id: u8,
    /// Has the dribbler contact to the ball
    pub infrared: bool,
    // TODO: battery
}

#[derive(Clone, Serialize)]
pub enum KICK {
    STRAIGHT_KICK { power: f32 },
    CHIP_KICK { power: f32 },
}

#[derive(Default, Clone, Serialize)]
pub struct Command {
    /// ID of the robot
    pub id: u32,
    /// Velocity forward in m.s-1 (towards the dribbler)
    pub forward_velocity: f32,
    /// Velocity to the left in m.s-1
    pub left_velocity: f32,
    /// Angular velocity rad.s-1 in (counter-clockwise)
    pub angular_velocity: f32,
    /// Order to charge the capacitor of the robot
    pub charge: bool,
    /// Order to kick the ball, if None doesn't KICK
    pub kick: Option<KICK>,
    /// Dribbler speed in rounds per minute rpm
    pub dribbler: f32,
}