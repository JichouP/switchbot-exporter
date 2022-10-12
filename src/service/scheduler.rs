use clokwerk::{AsyncScheduler, TimeUnits};
use crossbeam_channel::{select, Receiver};
use std::{
    future::Future,
    time::{Duration, Instant},
};

pub async fn setup<F, Fut>(f: F, ctrl_c_events: Receiver<()>, ticks: Receiver<Instant>) -> ()
where
    F: 'static + Fn() -> Fut + Send,
    Fut: 'static + Future<Output = ()> + Send,
{
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(60.seconds()).run(f);
    loop {
        select! {
            recv(ticks) -> _ => {
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
