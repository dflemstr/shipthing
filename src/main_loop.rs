use crate::state;
use futures::lock;
use std::sync;
use std::time;

pub async fn run(state: sync::Arc<lock::Mutex<state::State>>) -> Result<(), failure::Error> {
    let mut ticks = tokio::time::interval(std::time::Duration::from_millis(1));
    let mut prev_instant = tokio::time::Instant::now();
    loop {
        let instant = ticks.tick().await;
        log::trace!("tick at {:?}", instant);
        let delay = instant.saturating_duration_since(prev_instant);
        prev_instant = instant;
        let dt = delay.as_secs_f64();
        if dt > 0.0 {
            state.lock().await.update(dt);
        }
    }
}
