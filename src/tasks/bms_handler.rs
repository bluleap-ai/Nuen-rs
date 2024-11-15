use embassy_time::{Instant, Timer};
use log::{info, warn};

use crate::{tasks::BMS_CYCLE, CanBmsBox};

#[embassy_executor::task]
pub async fn bms_task(channel: &'static CanBmsBox) {
    info!("Started BMS Task !!!");
    loop {
        let start = Instant::now();

        let frame =  channel.receive().await;
        let ms = Instant::now().duration_since(start).as_millis();
        if ms > BMS_CYCLE {
            warn!("BMS task done after {ms}ms > {BMS_CYCLE}ms");
        } else {
            Timer::after_millis(BMS_CYCLE - ms).await;
        }
    }
}
