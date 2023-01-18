use crate::filters::filter::FilterTask;
use crate::libs::cli::Cli;
use crate::libs::data::{DataStore};
use crate::libs::tasks::inputs::input::FilterStore;

pub struct GameControllerFilter;

impl FilterTask for GameControllerFilter {
    fn with_cli(_cli: &mut Cli) -> Box<Self> {
        Box::new(Self {})
    }

    fn step(&self, store: &mut FilterStore, data_store: &mut DataStore) {
        // TODO : Make something better !
        // data_store.game_controller = Some(store.gc_packet.clone().into_iter().last().unwrap());
    }
}