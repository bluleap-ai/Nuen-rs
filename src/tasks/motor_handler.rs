use embassy_time::{Instant, Timer};
use log::{info, warn};

use crate::{tasks::MOTOR_CYCLE, CanMotorBox};

#[embassy_executor::task]
pub async fn motor_task(channel: &'static CanMotorBox) {
    info!("Started MOTOR Task !!!");
    loop {
        let start = Instant::now();

        let frame =  channel.receive().await;
        let ms = Instant::now().duration_since(start).as_millis();
        if ms > MOTOR_CYCLE {
            warn!("MOTOR task done after {ms}ms > {MOTOR_CYCLE}ms");
        } else {
            Timer::after_millis(MOTOR_CYCLE - ms).await;
        }
    }
}
