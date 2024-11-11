use defmt::Format;

use crate::io::SwitchGearInput;

#[derive(Format)]
pub enum Vehiclestate {
    Lock,
    Unlock,
    Riding,
    Charging,
}
pub struct StateControl {
    state: Vehiclestate,
    input: SwitchGearInput,
}

impl StateControl {
    pub fn init(input: SwitchGearInput) -> Self {
        StateControl {
            state: Vehiclestate::Lock,
            input,
        }
    }

    pub fn update(&mut self) -> &Vehiclestate {
        // check the input to make decision of state change
        &self.state
    }

    pub fn current_state(&self) -> &Vehiclestate {
        &self.state
    }
}
