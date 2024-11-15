use crate::{tasks::CAN_RX_CYCLE, CanBmsBox, CanMotorBox, CanObcBox, SimulinkBox, SimulinkType};
use embassy_stm32::can::CanRx;
use embassy_time::{Instant, Timer};
use log::{info, warn};

#[embassy_executor::task]
pub async fn can_rx_task(
    mut rx: CanRx<'static>,
    channel0: &'static SimulinkBox,
    channel2: &'static CanBmsBox,
    channel3: &'static CanMotorBox,
    channel4: &'static CanObcBox,
) {
    info!("Started CANRX Task !!!");
    loop {
        let start = Instant::now();
        match rx.read().await {
            Ok(evelope) => {
                info!("Receive CAN Frame {:?}", evelope);
                channel0.send(SimulinkType::Can(evelope.frame)).await;
            }
            Err(e) => {
                info!("Failed to receive CAN Frame: {:?}", e);
            }
        }

        let ms = Instant::now().duration_since(start).as_millis();
        if ms > CAN_RX_CYCLE {
            warn!("CanRx task done after {ms}ms > {CAN_RX_CYCLE}ms");
        } else {
            Timer::after_millis(CAN_RX_CYCLE - ms).await;
        }
    }
}
