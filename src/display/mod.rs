pub struct CanMessage {
    pub id: u32,
    pub data: [u8; 8],
}

pub struct SegLcd {
    status_1: CanMessage,
    status_2: CanMessage,
    status_3: CanMessage,
}

impl SegLcd {
    pub fn init() -> Self {
        let status_1 = CanMessage {
            id: 0x10F810A3,
            data: [0x00; 8],
        };
        let status_2 = CanMessage {
            id: 0x10F8109A,
            data: [0x00; 8],
        };
        let status_3 = CanMessage {
            id: 0x1800F907,
            data: [0x00; 8],
        };
        SegLcd {
            status_1,
            status_2,
            status_3,
        }
    }

    pub fn lcd_on(&mut self) -> CanMessage {
        self.status_1.data[1] |= 0x01;
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn lcd_off(&mut self) -> CanMessage {
        self.status_1.data[1] &= !0x01;
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn left_ind_on(&mut self) -> CanMessage {
        self.status_1.data[0] |= 0x01;
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn left_ind_off(&mut self) -> CanMessage {
        self.status_1.data[0] &= !0x01;
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn right_ind_on(&mut self) -> CanMessage {
        self.status_1.data[0] |= 0x01 << 1;
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn right_ind_off(&mut self) -> CanMessage {
        self.status_1.data[0] &= !(0x01 << 1);
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn pha_on(&mut self) -> CanMessage {
        self.status_1.data[0] |= 0x01 << 2;
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn pha_off(&mut self) -> CanMessage {
        self.status_1.data[0] &= !(0x01 << 2);
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn abs_on(&mut self) -> CanMessage {
        self.status_1.data[0] |= 0x01 << 3;
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn abs_off(&mut self) -> CanMessage {
        self.status_1.data[0] &= !(0x01 << 3);
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn rdy_on(&mut self) -> CanMessage {
        self.status_1.data[0] |= 0x01 << 4;
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn rdy_off(&mut self) -> CanMessage {
        self.status_1.data[0] &= !(0x01 << 4);
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn odo_reset_on(&mut self) -> CanMessage {
        self.status_1.data[1] |= 0x01 << 2;
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn odo_reset_off(&mut self) -> CanMessage {
        self.status_1.data[1] &= !(0x01 << 2);
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn trip_mode_on(&mut self) -> CanMessage {
        self.status_1.data[1] |= 0x01 << 3;
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }

    pub fn trip_mode_off(&mut self) -> CanMessage {
        self.status_1.data[1] &= !(0x01 << 3);
        CanMessage {
            id: self.status_1.id,
            data: self.status_1.data,
        }
    }
}
