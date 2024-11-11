use crate::{
    io::{BikeOutput, SwitchGearInput},
    ScreenBox, ScreenRequest, SimulinkBox, SimulinkType,
};
use defmt::*;

#[embassy_executor::task]
pub async fn state_machine_task(
    sw_gear: SwitchGearInput,
    mut bike_output: BikeOutput,
    channel0: &'static ScreenBox,
    channel1: &'static SimulinkBox,
) {
    info!("hello simulink!");
    sw_gear.print_all();
    loop {
        // Check if receiving any data from other tasks.
        match channel1.receive().await {
            SimulinkType::KeyFob(state) => {
                info!("Receive keyfob state {}", state);
            }
            SimulinkType::Can(frame) => {
                info!("Receive Can Frame {:?}", frame);
                channel0.send(ScreenRequest::LeftIndicator).await;
            }
        }

        // Output
        bike_output.set_all(false);
    }
}
