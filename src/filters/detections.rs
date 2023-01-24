use crate::filters::filter::FilterTask;
use crate::libs::cli::Cli;
use crate::libs::data::{DataStore, TeamColor};
use crate::libs::protobuf::vision_packet::SslDetectionRobot;
use crate::libs::robot::{AllyRobot, AsRobot, EnemyRobot, Robot};
use crate::libs::tasks::inputs::input::FilterStore;
use log::error;
use nalgebra::Point2;

pub struct DetectionFilter;

impl DetectionFilter {
    fn update_robot_poses<'a, T>(
        detected_robots: impl Iterator<Item = &'a SslDetectionRobot>,
        robots: &mut [Option<T>],
    ) where
        T: Default + AsRobot + From<Robot>,
    {
        detected_robots.for_each(|r| {
            if let Some(robot_id) = r.robot_id {
                if let Some(robot) = robots.get_mut(robot_id as usize) {
                    if robot.is_none() {
                        *robot = Some(T::from(Robot::default()));
                    }
                    robot.as_mut().unwrap().as_robot().update_pose(&r);
                } else {
                    error!(
                        "invalid robot id {} in detection packet",
                        r.robot_id.unwrap()
                    );
                }
            }
        });
    }

    fn update_robots(
        &self,
        allies: &Vec<SslDetectionRobot>,
        enemies: &Vec<SslDetectionRobot>,
        store: &mut DataStore,
    ) {
        Self::update_robot_poses(allies.iter(), &mut store.allies);
        Self::update_robot_poses(enemies.iter(), &mut store.enemies);
    }
}

impl FilterTask for DetectionFilter {
    fn with_cli(_cli: &Cli) -> Box<Self> {
        Box::new(Self)
    }

    fn step(&self, store: &mut FilterStore, data_store: &mut DataStore) {
        if store.vision_packet.is_empty() {
            error!("no vision packets found");
        }

        for packet in store.vision_packet.iter() {
            if let Some(detection_frame) = &packet.detection {
                if let Some(detected_ball) = detection_frame.balls.get(0) {
                    if let Some(ref mut ball) = data_store.ball {
                        ball.x = detected_ball.x / 1000.0;
                        ball.y = detected_ball.y / 1000.0;
                    } else {
                        data_store.ball = Some(Point2::new(detected_ball.x, detected_ball.y));
                    }
                }

                let (allies, enemies) = match data_store.color {
                    TeamColor::YELLOW => {
                        (&detection_frame.robots_yellow, &detection_frame.robots_blue)
                    }
                    TeamColor::BLUE => {
                        (&detection_frame.robots_blue, &detection_frame.robots_yellow)
                    }
                };

                self.update_robots(allies, enemies, data_store);
            }
        }
    }
}
