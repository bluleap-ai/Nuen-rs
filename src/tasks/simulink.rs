use crate::{
    io::{BikeOutput, SwitchGearInput},
    println,
    state_machine::{StateControl, Vehiclestate},
    ScreenBox, ScreenRequest, SimulinkBox, SimulinkType,
};
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn state_machine_task(
    sw_gear: SwitchGearInput,
    mut bike_output: BikeOutput,
    channel0: &'static ScreenBox,
    channel1: &'static SimulinkBox,
) {
    let mut state_control = StateControl::init(sw_gear);
    bike_output.set_all(false);
    println!("hello simulink!");
    loop {
        // Check if receiving any data from other tasks.
        if let Ok(rx) = channel1.try_receive() {
            match rx {
                SimulinkType::KeyFob(state) => {
                    println!("Receive keyfob state {}", state);
                }
                SimulinkType::Can(frame) => {
                    println!("Receive Can Frame {:?}", frame);
                    channel0.send(ScreenRequest::LeftIndicator).await;
                }
            }
        }
        // update state depends on current input
        let current_state = state_control.update();

        // execute the specific task depending on current state
        match current_state {
            Vehiclestate::Lock => { /* do something in Lock state */ }
            Vehiclestate::Parking => {
                channel0.send(ScreenRequest::Power(true)).await;
            }
            Vehiclestate::Unlock => { /* do something in Unlock state */ }
            Vehiclestate::Riding => {
                channel0.send(ScreenRequest::Ready).await;
            }
            Vehiclestate::PreRiding => { /* do something in PreRiding state */ }
            Vehiclestate::Charging => { /* do something in Charging state */ }
        }

        Timer::after_millis(10).await;
    }
}
