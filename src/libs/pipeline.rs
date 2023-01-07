use crate::libs::data;
use crate::libs::tasks::task::Task;
use data::DataStore;
use log::trace;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub type Pipeline = Vec<Box<dyn Task>>;

pub fn run_pipeline(mut data_store: &mut DataStore, pipeline: &mut Pipeline) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut min = f64::MAX;
    let mut max = f64::MIN;
    while running.load(Ordering::SeqCst) {
        let start = Instant::now();

        pipeline.iter_mut().for_each(|task| {
            task.run(&mut data_store);
        });

        sleep(Duration::from_millis(10));
        let elasped: f64 = start.elapsed().as_micros() as f64 / 1000.0;

        if elasped > max {
            max = elasped;
        }
        if elasped < min {
            min = elasped;
        }

        trace!(
            "pipeline took {:>6} ms, max: {:>6} ms, min: {:>6} ms",
            elasped,
            max,
            min
        );
    }
}
