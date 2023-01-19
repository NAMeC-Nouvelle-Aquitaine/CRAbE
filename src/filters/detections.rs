use crate::filters::filter::FilterTask;
use crate::libs::cli::Cli;
use crate::libs::data::{DataStore, TeamColor};
use crate::libs::protobuf::vision_packet::SslDetectionRobot;
use crate::libs::robot::{AllyRobot, Robot};
use crate::libs::tasks::inputs::input::FilterStore;
use log::error;

pub struct DetectionFilter;

/*
struct FilterResult<'a, 'b, T>
where
    &'b mut T: Into<&'a mut Robot>,
{
    robot: &'b mut T,
}

 */
impl DetectionFilter {
    /*
    fn update_robot_poses<'a, T>(
        robots: impl Iterator<Item = &'a SslDetectionRobot>,
        stored_robots: &'a mut [Option<T>],
    ) where
        &'a mut T: Into<&'a mut Robot>,
    {
        // TODO: Move into separate file?
        robots
            .filter(|r| r.robot_id.is_some())
            .filter_map(|stored| {
                if let Some(robot) = stored_robots.get_mut(stored.robot_id.unwrap() as usize) {
                    if let Some(robot) = robot {
                        return Some(FilterResult { stored, robot });
                    }

                    error!(
                        "robot id {} in detection packet isn't active",
                        stored.robot_id.unwrap()
                    );
                } else {
                    error!(
                        "invalid robot id {} in detection packet",
                        stored.robot_id.unwrap()
                    );
                }

                return None;
            })
            .for_each(|filter_result| {
                let robot: &mut Robot = filter_result.robot.into();
                robot.update_pose(&filter_result.stored);
            });
    }
     */

    fn update_robots(
        &self,
        allies: &Vec<SslDetectionRobot>,
        enemies: &Vec<SslDetectionRobot>,
        store: &mut DataStore,
    ) {
        /*
        Self::update_robot_poses(allies.iter(), store.allies.as_mut_slice());
        Self::update_robot_poses(enemies.iter(), store.enemies.as_mut_slice());
         */

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
                        error!(
                            "ally robot id {} in detection packet isn't active",
                            r.robot_id.unwrap()
                        );

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
                    Some(None) => {
                        error!(
                            "enemy robot id {} in detection packet isn't active",
                            r.robot_id.unwrap()
                        );
                    }
                    Some(Some(robot)) => {
                        robot.robot.update_pose(r);
                    }
                },
            );
    }
}

impl FilterTask for DetectionFilter {
    fn with_cli(_cli: &Cli) -> Box<Self> {
        Box::new(Self {})
    }

    fn step(&self, store: &mut FilterStore, data_store: &mut DataStore) {
        for packet in store.vision_packet.iter() {
            if let Some(detection_frame) = &packet.detection {
                if let Some(detected_ball) = detection_frame.balls.get(0) {
                    if let Some(ref mut ball) = data_store.ball {
                        ball.x = detected_ball.x / 1000.0;
                        ball.y = detected_ball.y / 1000.0;
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
