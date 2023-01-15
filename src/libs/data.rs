use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::protobuf::game_controller_packet::Referee;
use crate::libs::protobuf::simulation_packet::RobotCommand;
use crate::libs::protobuf::vision_packet::{SslDetectionRobot, SslWrapperPacket};
use nalgebra::Point2;
use serde::{Deserialize, Serialize};
use crate::libs::protobuf::tools_packet;
use crate::libs::protobuf::tools_packet::AnnotationCircle;

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
    pub annotations: Annotations,
}

#[derive(Default)]
pub struct Annotations {
    pub annotations_packets: Vec<tools_packet::AnnotationWrapper>,
}

impl Annotations{
    fn add_circle(&mut self, id: &str, radius: f32, x: f32, y: f32) {
        let circle = tools_packet::annotation::AnnotationType::Circle(
            AnnotationCircle {
                center: Some(tools_packet::Point { x, y }),
                radius,
            }
        );

        let mut wrapper = tools_packet::AnnotationWrapper::default();
        let annotation = tools_packet::Annotation {
            annotation_type: Some(circle),
            fill_color : Some(tools_packet::Rgba { red: 0, green: 0, blue: 0, alpha: 0 }),
            shape_color: Some(tools_packet::Rgba { red: 0, green : 0, blue: 0, alpha: 1}),
        };

        let mut actions = tools_packet::annotation_wrapper::Actions::Add(tools_packet::Add {
            annotation: Some(annotation),
            id: id.to_string()
        });
        wrapper.actions = Some(actions);

        self.annotations_packets.push(wrapper);
    }
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

#[derive(Default, Clone)]
pub struct ControllableRobot {
    pub robot: Robot,
    pub command: Option<RobotCommand>,
    pub feedback: Option<ControllableRobotFeedback>,
}

#[derive(Default, Clone, Serialize)]
pub struct ControllableRobotFeedback {
    pub infrared: bool,
    // TODO: battery
}

// TODO : Move this directly on the filter ?
impl ControllableRobot {
    pub fn update_pose(&mut self, robot_detection_packet: &SslDetectionRobot) {
        self.robot.update_pose(robot_detection_packet);
    }
}
