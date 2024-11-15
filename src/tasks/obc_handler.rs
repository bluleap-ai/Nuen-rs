use embassy_time::{Instant, Timer};
use log::{info, warn};

use crate::{tasks::OBC_CYCLE, CanObcBox};

#[embassy_executor::task]
pub async fn obc_task(channel: &'static CanObcBox) {
    info!("Started OBC Task !!!");
    loop {
        let start = Instant::now();

        let frame =  channel.receive().await;
        let ms = Instant::now().duration_since(start).as_millis();
        if ms > OBC_CYCLE {
            warn!("OBC task done after {ms}ms > {OBC_CYCLE}ms");
        } else {
            Timer::after_millis(OBC_CYCLE - ms).await;
        }
    }
}
