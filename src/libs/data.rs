use crate::libs::constants::NUMBER_OF_ROBOTS;
use crate::libs::protobuf::game_controller_packet::Referee;
use crate::libs::robot::{AllyRobot, EnemyRobot};
use nalgebra::Point2;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Default, Serialize)]
pub struct DataStore {
    pub color: TeamColor,
    pub blue_on_positive_half: bool,
    pub ball: Option<Point2<f32>>,
    pub allies: [Option<AllyRobot>; NUMBER_OF_ROBOTS],
    pub enemies: [Option<EnemyRobot>; NUMBER_OF_ROBOTS],
    // TODO : Rename opponents
    //pub game_controller: Option<Referee>,
    pub field: Option<Field>,
}

impl DataStore {
    pub fn active_enemies_mut(&mut self) -> impl Iterator<Item = &mut EnemyRobot> {
        self.enemies.iter_mut().filter_map(|x| x.as_mut())
    }

    pub fn active_enemies(&mut self) -> impl Iterator<Item = &EnemyRobot> {
        self.enemies.iter().filter_map(|x| x.as_ref())
    }

    pub fn active_allies_mut(&mut self) -> impl Iterator<Item = &mut AllyRobot> {
        self.allies.iter_mut().filter_map(|x| x.as_mut())
    }

    pub fn active_allies(&mut self) -> impl Iterator<Item = &AllyRobot> {
        self.allies.iter().filter_map(|x| x.as_ref())
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

impl Display for TeamColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamColor::BLUE => write!(f, "blue"),
            TeamColor::YELLOW => write!(f, "yellow"),
        }
    }
}

impl Default for TeamColor {
    fn default() -> Self {
        TeamColor::BLUE
    }
}

#[derive(Clone, Serialize)]
pub enum Kick {
    StraightKick { power: f32 },
    ChipKick { power: f32 },
}

#[derive(Default, Clone, Serialize)]
pub struct Command {
    /// ID of the robot
    pub id: u8,
    /// Velocity forward in m.s-1 (towards the dribbler)
    pub forward_velocity: f32,
    /// Velocity to the left in m.s-1
    pub left_velocity: f32,
    /// Angular velocity rad.s-1 in (counter-clockwise)
    pub angular_velocity: f32,
    /// Order to charge the capacitor of the robot
    pub charge: bool,
    /// Order to kick the ball, if None doesn't KICK
    pub kick: Option<Kick>,
    /// Dribbler speed in rounds per minute rpm
    pub dribbler: f32,
}
