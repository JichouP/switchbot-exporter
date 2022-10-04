use clokwerk::{AsyncScheduler, TimeUnits};
use std::{future::Future, time::Duration};
use tokio::task::JoinHandle;

pub async fn setup<F, Fut>(f: F) -> JoinHandle<()>
where
    F: 'static + Fn() -> Fut + Send,
    Fut: 'static + Future<Output = ()> + Send,
{
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(60.seconds()).run(f);
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
