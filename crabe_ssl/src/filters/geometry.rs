use crate::filters::filter::FilterTask;
use crate::libs::cli::Cli;
use crabe_framework::data::field::Field;
use crate::libs::data::{DataStore};
use crate::libs::tasks::inputs::input::FilterStore;

pub struct GeometryFilter;

impl FilterTask for GeometryFilter {
    fn with_cli(_cli: &Cli) -> Box<Self> {
        Box::new(Self)
    }

    fn step(&self, store: &mut FilterStore, data_store: &mut DataStore) {
        // TODO : Do we want to do everytime ?
        for packet in store.vision_packet.iter() {
            if let Some(geometry) = &packet.geometry {
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
