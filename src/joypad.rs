pub struct Joypad {
    button_keys: u8,
    direction_keys: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            button_keys: 0x0F,
            direction_keys: 0x0F,
        }
    }

    pub fn key_down(&mut self, key: u8) {
        match key {
            0 => self.direction_keys &= !0x01, // Right
            1 => self.direction_keys &= !0x02, // Left  
            2 => self.direction_keys &= !0x04, // Up
            3 => self.direction_keys &= !0x08, // Down
            4 => self.button_keys &= !0x01,    // A
            5 => self.button_keys &= !0x02,    // B
            6 => self.button_keys &= !0x04,    // Select
            7 => self.button_keys &= !0x08,    // Start
            _ => {}
        }
    }

    pub fn key_up(&mut self, key: u8) {
        match key {
            0 => self.direction_keys |= 0x01,  // Right
            1 => self.direction_keys |= 0x02,  // Left
            2 => self.direction_keys |= 0x04,  // Up  
            3 => self.direction_keys |= 0x08,  // Down
            4 => self.button_keys |= 0x01,     // A
            5 => self.button_keys |= 0x02,     // B
            6 => self.button_keys |= 0x04,     // Select
            7 => self.button_keys |= 0x08,     // Start
            _ => {}
        }
    }

    pub fn get_state(&self) -> u8 {
        0xFF
    }
}