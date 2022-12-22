use crate::libs::data;
use crate::libs::tasks::task::Task;
use data::DataStore;
use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub type Pipeline = Vec<Box<dyn Task>>;

pub fn run_pipeline(mut data_store: &mut DataStore, pipeline: &mut Pipeline) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        let start = Instant::now();

        pipeline.iter_mut().for_each(|task| {
            task.run(&mut data_store)
                .expect("TODO: some good error lmao")
        });

        info!(
            "pipeline took {:>6} ms",
            start.elapsed().as_micros() as f64 / 1000.0
        );
    }
}
