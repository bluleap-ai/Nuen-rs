// use defmt::*;
use embassy_stm32::gpio::{Input, Output};

pub struct SwitchGearInput {
    pub kill_sw: Input<'static>,
    pub mode_sw: Input<'static>,
    pub side_stand_sw: Input<'static>,
    pub reverse_sw: Input<'static>,
    pub horn_sw: Input<'static>,
    pub pha_cos_pw_sw: Input<'static>,
    pub pha_cos_sw: Input<'static>,
    pub left_braker_sw: Input<'static>,
    pub right_braker_sw: Input<'static>,
    pub keyfob_a_sw: Input<'static>,
    pub keyfob_b_sw: Input<'static>,
    pub keyfob_c_sw: Input<'static>,
    pub keyfob_d_sw: Input<'static>,
    pub turn_right_sw: Input<'static>,
    pub turn_left_sw: Input<'static>,
}

pub struct BikeOutput {
    pub seat_lock: Output<'static>,
    pub tank_lock: Output<'static>,
    pub sound_engine: Output<'static>,
    pub braker_lamp: Output<'static>,
    pub turn_right_lamp: Output<'static>,
    pub pha_lamp: Output<'static>,
    pub cos_lamp: Output<'static>,
    pub license_lamp: Output<'static>,
    pub horn: Output<'static>,
    pub tail_lamp: Output<'static>,
    pub turn_left_lamp: Output<'static>,
}

#[allow(dead_code)]
impl BikeOutput {
    pub fn set_all(&mut self, is_on: bool) {
        if is_on {
            self.seat_lock.set_high();
            self.tank_lock.set_high();
            self.sound_engine.set_high();
            self.braker_lamp.set_high();
            self.turn_right_lamp.set_high();
            self.pha_lamp.set_high();
            self.cos_lamp.set_high();
            self.license_lamp.set_high();
            self.horn.set_high();
            self.tail_lamp.set_high();
            self.turn_left_lamp.set_high();
        } else {
            self.seat_lock.set_low();
            self.tank_lock.set_low();
            self.sound_engine.set_low();
            self.braker_lamp.set_low();
            self.turn_right_lamp.set_low();
            self.pha_lamp.set_low();
            self.cos_lamp.set_low();
            self.license_lamp.set_low();
            self.horn.set_low();
            self.tail_lamp.set_low();
            self.turn_left_lamp.set_low();
        }
    }

    pub fn seat_unlock(&mut self) {
        self.seat_lock.set_high();
    }

    pub fn seat_lock(&mut self) {
        self.seat_lock.set_low();
    }

    pub fn tank_lock(&mut self) {
        self.tank_lock.set_high();
    }

    pub fn tank_unlock(&mut self) {
        self.tank_lock.set_low();
    }

    pub fn sound_enable(&mut self) {
        self.sound_engine.set_high();
    }

    pub fn braker_lamp_en(&mut self) {
        self.seat_lock.set_low();
    }

    pub fn turn_right_lamp(&mut self) {
        self.seat_lock.set_low();
    }

    pub fn pha_lamp(&mut self) {
        self.seat_lock.set_low();
    }

    pub fn cos_lamp(&mut self) {
        self.seat_lock.set_low();
    }

    pub fn license_lamp(&mut self) {
        self.seat_lock.set_low();
    }

    pub fn horn_en(&mut self) {
        self.seat_lock.set_low();
    }

    pub fn tail_lamp(&mut self) {
        self.seat_lock.set_low();
    }

    pub fn turn_left_lamp(&mut self) {
        self.seat_lock.set_low();
    }
}

impl SwitchGearInput {
    pub fn kill_sw(&self) -> bool {
        self.kill_sw.is_high()
    }

    pub fn mode_sw(&self) -> bool {
        self.mode_sw.is_high()
    }

    pub fn ss_sw(&self) -> bool {
        self.side_stand_sw.is_high()
    }

    pub fn rev_sw(&self) -> bool {
        self.reverse_sw.is_high()
    }

    pub fn horn_sw(&self) -> bool {
        self.horn_sw.is_high()
    }

    pub fn pc_power_sw(&self) -> bool {
        self.pha_cos_pw_sw.is_high()
    }

    pub fn pc_sw(&self) -> bool {
        self.pha_cos_sw.is_high()
    }

    pub fn lb_sw(&self) -> bool {
        self.left_braker_sw.is_high()
    }

    pub fn rb_sw(&self) -> bool {
        self.right_braker_sw.is_high()
    }

    pub fn kf_a_sw(&self) -> bool {
        self.keyfob_a_sw.is_high()
    }

    pub fn kf_b_sw(&self) -> bool {
        self.keyfob_b_sw.is_high()
    }

    pub fn kf_c_sw(&self) -> bool {
        self.keyfob_c_sw.is_high()
    }

    pub fn kf_d_sw(&self) -> bool {
        self.keyfob_d_sw.is_high()
    }

    pub fn turn_r_sw(&self) -> bool {
        self.turn_left_sw.is_high()
    }

    pub fn turn_l_sw(&self) -> bool {
        self.turn_right_sw.is_high()
    }

    pub fn print_all(&self) {
        // println!(
        //     "SW_GEAR_STATUS: \n
        //         \tkill_sw: {}\n
        //         \tmode_sw: {}\n
        //         \tside_stand_sw: {}\n
        //         \treverse_sw: {}\n
        //         \thorn_sw: {}\n
        //         \tpha_cos_power_sw: {}\n
        //         \tpha_cos_sw: {}\n
        //         \tleft_braker_sw: {}\n
        //         \tright_braker_sw: {}\n
        //         \tkeyfob_A_sw: {}\n
        //         \tkeyfob_B_sw: {}\n
        //         \tkeyfob_C_sw: {}\n
        //         \tkeyfob_D_sw: {}\n
        //         \tturn_right_sw: {}\n
        //         \tturn_left_sw: {}\n",
        //     self.kill_sw(),
        //     self.mode_sw(),
        //     self.ss_sw(),
        //     self.rev_sw(),
        //     self.horn_sw(),
        //     self.pc_power_sw(),
        //     self.pc_sw(),
        //     self.lb_sw(),
        //     self.rb_sw(),
        //     self.kf_a_sw(),
        //     self.kf_b_sw(),
        //     self.kf_c_sw(),
        //     self.kf_d_sw(),
        //     self.turn_r_sw(),
        //     self.turn_l_sw()
        // )
    }
}
