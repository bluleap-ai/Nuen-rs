use crate::{SimulinkBox, SimulinkType};
use defmt::*;
use embassy_stm32::can::CanRx;

#[embassy_executor::task]
pub async fn can_rx_task(mut rx: CanRx<'static>, channel: &'static SimulinkBox) {
    info!("hello can tx!");
    loop {
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
}
