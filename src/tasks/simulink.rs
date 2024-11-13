use crate::{
    io::{BikeOutput, SwitchGearInput},
    state_machine::{StateControl, Vehiclestate},
    tasks::SIM_APP_CYCLE,
    ScreenBox, ScreenRequest, SimulinkBox, SimulinkType,
};
use embassy_time::{Instant, Timer};
use log::{info, warn};

#[embassy_executor::task]
pub async fn state_machine_task(
    sw_gear: SwitchGearInput,
    mut bike_output: BikeOutput,
    channel0: &'static ScreenBox,
    channel1: &'static SimulinkBox,
) {
    let mut state_control = StateControl::init(sw_gear);
    bike_output.set_all(false);
    info!("hello simulink!");
    loop {
        let start = Instant::now();
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

        let ms = Instant::now().duration_since(start).as_millis();
        if ms > SIM_APP_CYCLE {
            warn!("Simapp task done after {ms}ms > {SIM_APP_CYCLE}ms");
        } else {
            Timer::after_millis(SIM_APP_CYCLE - ms).await;
        }
    }
}
