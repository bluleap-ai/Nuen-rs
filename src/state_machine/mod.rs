use defmt::{info, Format};

use crate::io::SwitchGearInput;

#[derive(Format)]
pub enum Vehiclestate {
    Lock,
    Parking,
    Unlock,
    PreRiding,
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
        self.input.print_all();
        match self.state {
            Vehiclestate::Lock => self.handle_lock_state(),
            Vehiclestate::Parking => self.handle_parking_state(),
            Vehiclestate::Unlock => self.handle_unlock_state(),
            Vehiclestate::PreRiding => self.handle_preriding_state(),
            Vehiclestate::Riding => self.handle_riding_state(),
            Vehiclestate::Charging => self.handle_charging_state(),
        }
    }

    pub fn current_state(&self) -> &Vehiclestate {
        &self.state
    }

    fn handle_lock_state(&mut self) -> &Vehiclestate {
        // if keyfob B pressed --> parking state
        if self.input.kf_b_sw() {
            info!("change state from Lock to Parking");
            self.state = Vehiclestate::Parking;
        }
        &self.state
    }

    fn handle_parking_state(&mut self) -> &Vehiclestate {
        // if keyfob B pressed --> parking state
        if !self.input.turn_r_sw() || !self.input.turn_l_sw() {
            info!("change state from Parking to Unlock");
            self.state = Vehiclestate::Unlock;
        }
        &self.state
    }

    fn handle_preriding_state(&mut self) -> &Vehiclestate {
        // check Pin, BMS, MC, OBC, MCU temperature < 50
        info!("change state from Preriding to Riding");
        self.state = Vehiclestate::Riding;
        &self.state
    }

    fn handle_riding_state(&mut self) -> &Vehiclestate {
        &self.state
    }

    fn handle_charging_state(&mut self) -> &Vehiclestate {
        &self.state
    }

    fn handle_unlock_state(&mut self) -> &Vehiclestate {
        if self.input.ss_sw() {
            info!("change state from Unlock to PreRiding");
            self.state = Vehiclestate::PreRiding;
        }
        &self.state
    }
}
