use crate::{
    io::{BikeOutput, SwitchGearInput},
    ScreenBox, ScreenRequest, SimulinkBox, SimulinkType,
};
use log::info;

#[embassy_executor::task]
pub async fn state_machine_task(
    sw_gear: SwitchGearInput,
    mut bike_output: BikeOutput,
    channel0: &'static ScreenBox,
    channel1: &'static SimulinkBox,
) {
    loop {
        // Check if receiving any data from other tasks.
        if let Ok(data) = channel1.try_receive() {
            match data {
                SimulinkType::KeyFob(state) => {
                    info!("Receive keyfob state {}", state);
                }
                SimulinkType::Can(frame) => {
                    info!("Receive Can Frame {:?}", frame);
                    channel0.send(ScreenRequest::LeftIndicator).await;
                }
            }
        }

        // Check SW gear input
        sw_gear.print_all();

        // Output
        bike_output.set_all(false);
    }
}
