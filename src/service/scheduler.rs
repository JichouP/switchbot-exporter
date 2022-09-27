use clokwerk::{Scheduler, TimeUnits};
use std::{thread, time::Duration};

pub fn setup_schedule<F>(f: F)
where
    F: 'static + Fn() + Send,
{
    let _join_handle = std::thread::spawn(move || {
        let mut scheduler = Scheduler::new();
        scheduler.every(1.seconds()).run(f);
        loop {
            scheduler.run_pending();
            thread::sleep(Duration::from_millis(10));
        }
    });
}
