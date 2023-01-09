use crate::libs::cli::Cli;
use crate::libs::data::{ControllableRobot, DataStore, Field, Robot, TeamColor};
use crate::libs::protobuf::vision_packet::SslDetectionRobot;
use crate::libs::tasks::task::Task;
use log::{error, trace};

#[derive(Default)]
pub struct PassoireFilterTask;

impl PassoireFilterTask {
    fn update_robots(
        &self,
        allies: &Vec<SslDetectionRobot>,
        enemies: &Vec<SslDetectionRobot>,
        store_allies: &mut [ControllableRobot],
        store_enemies: &mut [Robot],
    ) {
        allies
            .into_iter()
            .filter(|r| r.robot_id.is_some())
            .for_each(
                |r| match store_allies.get_mut(r.robot_id.unwrap() as usize) {
                    None => {
                        error!(
                            "invalid robot id {} in detection packet",
                            r.robot_id.unwrap()
                        );
                    }
                    Some(robot) => {
                        robot.update_pose(r);
                    }
                },
            );

        enemies
            .into_iter()
            .filter(|r| r.robot_id.is_some())
            .for_each(
                |r| match store_enemies.get_mut(r.robot_id.unwrap() as usize) {
                    None => {
                        error!(
                            "invalid robot id {} in detection packet",
                            r.robot_id.unwrap()
                        );
                    }
                    Some(robot) => {
                        robot.update_pose(r);
                    }
                },
            );
    }
}

impl Task for PassoireFilterTask {
    fn with_cli(_cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) {
        let (packets, allies, enemies) = (
            &mut data_store.vision,
            &mut data_store.allies,
            &mut data_store.enemies,
        );
        for packet in packets.iter_mut() {
            match &packet.detection {
                None => {}
                Some(detection_frame) => {
                    if let Some(ball) = detection_frame.balls.get(0) {
                        data_store.ball.x = ball.x / 1000.0;
                        data_store.ball.y = ball.y / 1000.0;
                    }

                    let (robots_blue, robots_yellow) =
                        (&detection_frame.robots_blue, &detection_frame.robots_yellow);

                    // TODO: bounds check
                    match data_store.color {
                        TeamColor::YELLOW => {
                            self.update_robots(robots_yellow, robots_blue, allies, enemies)
                        }
                        TeamColor::BLUE => {
                            self.update_robots(robots_blue, robots_yellow, allies, enemies)
                        }
                    }
                }
            }

            // TODO : Move this on another filter
            match &packet.geometry {
                None => {}
                Some(geometry) => {
                    data_store.field = Some(Field {
                        length: geometry.field.field_length as f32 / 1000.0,
                        width: geometry.field.field_width as f32 / 1000.0,
                        goal_width: geometry.field.goal_width as f32 / 1000.0,
                        goal_depth: geometry.field.goal_depth as f32 / 1000.0,
                        center_radius: geometry.field.center_circle_radius.unwrap_or(500) as f32
                            / 1000.0, // TODO : Calculate the default with arcs
                        penalty_depth: geometry.field.penalty_area_depth.unwrap_or(1000) as f32
                            / 1000.0, // TODO : Calculate the default with arcs
                        penalty_width: geometry.field.penalty_area_width.unwrap_or(2000) as f32
                            / 1000.0, // TODO : Calculate the default with arcs
                    });
                }
            }
        }
    }
}
