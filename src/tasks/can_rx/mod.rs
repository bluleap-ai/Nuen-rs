use crate::{SimulinkBox, SimulinkType};
use embassy_stm32::can::CanRx;
use log::{error, info};

#[embassy_executor::task]
pub async fn can_rx_task(mut rx: CanRx<'static>, channel: &'static SimulinkBox) {
    match rx.read().await {
        Ok(evelope) => {
            info!("Receive CAN Frame {:?}", evelope);
            channel.send(SimulinkType::Can(evelope.frame)).await;
        }
        Err(e) => {
            error!("Failed to receive CAN Frame: {:?}", e);
        }
    }
}
