use crate::{println, tasks::CAN_RX_CYCLE, SimulinkBox, SimulinkType};
use embassy_stm32::can::CanRx;
use embassy_time::{Instant, Timer};

#[embassy_executor::task]
pub async fn can_rx_task(mut rx: CanRx<'static>, channel: &'static SimulinkBox) {
    println!("hello can tx!");
    loop {
        let start = Instant::now();
        match rx.read().await {
            Ok(evelope) => {
                println!("Receive CAN Frame {:?}", evelope);
                channel.send(SimulinkType::Can(evelope.frame)).await;
            }
            Err(e) => {
                println!("Failed to receive CAN Frame: {:?}", e);
            }
        }

        let ms = Instant::now().duration_since(start).as_millis();
        if ms > CAN_RX_CYCLE {
            println!("WARN: CanRx task done after {ms}ms > {CAN_RX_CYCLE}ms");
        } else {
            Timer::after_millis(CAN_RX_CYCLE - ms).await;
        }
    }
}
