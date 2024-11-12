use crate::{SimulinkBox, SimulinkType};
use embassy_stm32::can::CanRx;

#[embassy_executor::task]
pub async fn can_rx_task(mut rx: CanRx<'static>, channel: &'static SimulinkBox) {
    // println!("hello can tx!");
    loop {
        match rx.read().await {
            Ok(evelope) => {
                // println!("Receive CAN Frame {:?}", evelope);
                channel.send(SimulinkType::Can(evelope.frame)).await;
            }
            Err(e) => {
                // println!("Failed to receive CAN Frame: {:?}", e);
            }
        }
    }
}
