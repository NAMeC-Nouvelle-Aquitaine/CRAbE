use log::error;
use crate::filters::filter::FilterTask;
use crate::libs::cli::Cli;
use crate::libs::data::{DataStore, TeamColor};
use crate::libs::protobuf::vision_packet::SslDetectionRobot;
use crate::libs::tasks::inputs::input::FilterStore;

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
                |r| match store.enemies.get_mut(r.robot_id.unwrap() as usize) {
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

impl FilterTask for DetectionFilter {
    fn with_cli(_cli: &mut Cli) -> Box<Self> {
        Box::new(Self {})
    }

    fn step(&self, store: &mut FilterStore, data_store: &mut DataStore) {


        for packet in store.vision_packet.iter() {
            if let Some(detection_frame) = &packet.detection {
                if let Some(ball) = detection_frame.balls.get(0) {
                    data_store.ball.x = ball.x / 1000.0;
                    data_store.ball.y = ball.y / 1000.0;
                }

                let (robots_blue, robots_yellow) =
                    (&detection_frame.robots_blue, &detection_frame.robots_yellow);

                match data_store.color {
                    TeamColor::YELLOW => {
                        self.update_robots(robots_yellow, robots_blue, data_store)
                    }
                    TeamColor::BLUE => {
                        self.update_robots(robots_blue, robots_yellow, data_store)
                    }
                }
            }
        }
    }
}