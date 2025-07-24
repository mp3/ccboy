use crate::memory::Memory;

const TIMER_INTERRUPT: u8 = 0x04;

pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    timer_counter: u32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            timer_counter: 0,
        }
    }

    pub fn update(&mut self, cycles: u8, memory: &mut Memory) {
        self.update_divider(cycles);
        
        if self.is_timer_enabled() {
            self.timer_counter += cycles as u32;
            
            let frequency = self.get_timer_frequency();
            while self.timer_counter >= frequency {
                self.timer_counter -= frequency;
                self.tima = self.tima.wrapping_add(1);
                
                if self.tima == 0 {
                    self.tima = self.tma;
                    memory.request_interrupt(TIMER_INTERRUPT);
                }
            }
        }
        
        self.sync_registers(memory);
    }

    fn update_divider(&mut self, cycles: u8) {
        self.div = self.div.wrapping_add(cycles as u16);
    }

    fn is_timer_enabled(&self) -> bool {
        (self.tac & 0x04) != 0
    }

    fn get_timer_frequency(&self) -> u32 {
        match self.tac & 0x03 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => unreachable!(),
        }
    }

    fn sync_registers(&mut self, memory: &mut Memory) {
        let div_value = memory.read_byte(0xFF04);
        if div_value != (self.div >> 8) as u8 {
            if div_value == 0 {
                self.div = 0;
            }
        }
        
        let tima_value = memory.read_byte(0xFF05);
        if tima_value != self.tima {
            self.tima = tima_value;
        }
        
        let tma_value = memory.read_byte(0xFF06);
        if tma_value != self.tma {
            self.tma = tma_value;
        }
        
        let tac_value = memory.read_byte(0xFF07);
        if tac_value != self.tac {
            self.tac = tac_value;
        }
        
        memory.write_byte(0xFF04, (self.div >> 8) as u8);
        memory.write_byte(0xFF05, self.tima);
        memory.write_byte(0xFF06, self.tma);
        memory.write_byte(0xFF07, self.tac);
    }
}