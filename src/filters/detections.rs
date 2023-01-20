use crate::filters::filter::FilterTask;
use crate::libs::cli::Cli;
use crate::libs::data::{DataStore, TeamColor};
use crate::libs::protobuf::vision_packet::SslDetectionRobot;
use crate::libs::robot::{AllyRobot, EnemyRobot};
use crate::libs::tasks::inputs::input::FilterStore;
use log::error;
use nalgebra::Point2;

pub struct DetectionFilter;

impl DetectionFilter {
    fn update_robots(
        &self,
        allies: &Vec<SslDetectionRobot>,
        enemies: &Vec<SslDetectionRobot>,
        store: &mut DataStore,
    ) {
        allies
            .into_iter()
            .filter(|r| r.robot_id.is_some())
            .for_each(
                |r| match store.allies.get_mut(r.robot_id.unwrap() as usize) {
                    None => {
                        error!(
                            "invalid ally robot id {} in detection packet",
                            r.robot_id.unwrap()
                        );
                    }
                    Some(Some(ally)) => {
                        ally.robot.update_pose(r);
                    }
                    Some(ally) => {
                        *ally = Some(AllyRobot {
                            robot: Default::default(),
                            info: None,
                        });
                    }
                },
            );

        enemies
            .into_iter()
            .filter(|r| r.robot_id.is_some())
            .for_each(
                |r| match store.enemies.get_mut(r.robot_id.unwrap() as usize) {
                    None => {
                        error!(
                            "invalid enemy robot id {} in detection packet",
                            r.robot_id.unwrap()
                        );
                    }
                    Some(Some(enemy)) => {
                        enemy.robot.update_pose(r);
                    }
                    Some(enemy) => {
                        *enemy = Some(EnemyRobot {
                            robot: Default::default(),
                            info: None,
                        });
                    }
                },
            );
    }
}

impl FilterTask for DetectionFilter {
    fn with_cli(_cli: &Cli) -> Box<Self> {
        Box::new(Self)
    }

    fn step(&self, store: &mut FilterStore, data_store: &mut DataStore) {
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

                let (robots_blue, robots_yellow) =
                    (&detection_frame.robots_blue, &detection_frame.robots_yellow);

                match data_store.color {
                    TeamColor::YELLOW => self.update_robots(robots_yellow, robots_blue, data_store),
                    TeamColor::BLUE => self.update_robots(robots_blue, robots_yellow, data_store),
                }
            }
        }
    }
}
