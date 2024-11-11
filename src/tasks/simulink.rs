use crate::{
    io::{BikeOutput, SwitchGearInput},
    state_machine::{StateControl, Vehiclestate},
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
    let mut state_control = StateControl::init(sw_gear);
    info!("hello simulink!");
    loop {
        // Check if receiving any data from other tasks.
        if let Ok(rx) = channel1.try_receive() {
            match rx {
                SimulinkType::KeyFob(state) => {
                    info!("Receive keyfob state {}", state);
                }
                SimulinkType::Can(frame) => {
                    info!("Receive Can Frame {:?}", frame);
                    channel0.send(ScreenRequest::LeftIndicator).await;
                }
            }
        }
        // update state depends on current input
        let current_state = state_control.update();
        info!("Current state is {}", current_state);

        // execute the specific task depending on current state
        match current_state {
            Vehiclestate::Lock => { /* do something in Lock state */ }
            Vehiclestate::Unlock => { /* do something in Unlock state */ }
            Vehiclestate::Riding => { /* do something in Riding state */ }
            Vehiclestate::Charging => { /* do something in Charging state */ }
        }

        // Output
        bike_output.set_all(false);
    }
}
