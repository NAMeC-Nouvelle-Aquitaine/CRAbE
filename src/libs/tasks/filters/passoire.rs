use crate::libs::cli::Cli;
use crate::libs::data::{DataStore, Field, TeamColor};
use crate::libs::protobuf::vision_packet::SslDetectionRobot;
use crate::libs::tasks::task::Task;

#[derive(Default)]
pub struct PassoireFilterTask;

impl Task for PassoireFilterTask {
    fn with_cli(_cli: &mut Cli) -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn run(&mut self, data_store: &mut DataStore) -> Result<(), String> {
        let mut packets = data_store.vision.iter();

        while let Some(packet) = packets.next() {
            match &packet.detection {
                None => {}
                Some(detection_frame) => {
                    if let Some(ball) = detection_frame.balls.get(0) {
                        data_store.ball.x = ball.x;
                        data_store.ball.y = ball.y;
                    }

                    let (robots_blue, robots_yellow) =
                        (&detection_frame.robots_blue, &detection_frame.robots_yellow);

                    // TODO: bounds check
                    match data_store.color {
                        TeamColor::YELLOW => {
                            robots_yellow.into_iter()
                                .filter(|r| r.robot_id.is_some())
                                .for_each(|r| {
                                    data_store.allies[r.robot_id.unwrap() as usize].update_pose(r);
                                });

                            robots_blue.into_iter()
                                .filter(|r| r.robot_id.is_some())
                                .for_each(|r| {
                                    data_store.enemies[r.robot_id.unwrap() as usize].update_pose(r);
                                });
                        },
                        TeamColor::BLUE => {
                            robots_blue.into_iter()
                                .filter(|r| r.robot_id.is_some())
                                .for_each(|r| {
                                    data_store.allies[r.robot_id.unwrap() as usize].update_pose(r);
                                });

                            robots_yellow.into_iter()
                                .filter(|r| r.robot_id.is_some())
                                .for_each(|r| {
                                    data_store.enemies[r.robot_id.unwrap() as usize].update_pose(r);
                                });
                        },
                    }
                }
            }

            // TODO : Do we need to update only one time ?
            match &packet.geometry {
                None => {}
                Some(geometry) => {
                    data_store.field = Some(Field {
                        length : geometry.field.field_length as f32 / 1000.0,
                        width:  geometry.field.field_width as f32 / 1000.0,
                        goal_width : geometry.field.goal_width as f32 / 1000.0,
                        goal_depth : geometry.field.goal_depth as f32 / 1000.0
                    });
                }
            }
        }

        Ok(())
    }
}
