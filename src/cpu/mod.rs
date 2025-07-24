mod registers;
mod instructions;
mod opcodes;
mod opcodes_extended;
mod opcodes_alu;
mod opcodes_cb;

use registers::Registers;
use crate::memory::Memory;

pub struct Cpu {
    pub registers: Registers,
    pub ime: bool,  // Interrupt Master Enable
    pub halt: bool,
    pub cycles: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            ime: false,
            halt: false,
            cycles: 0,
        }
    }

    pub fn step(&mut self, memory: &mut Memory) -> u8 {
        if self.halt {
            return 4;
        }

        let opcode = self.fetch_byte(memory);
        let cycles = self.execute_opcode(opcode, memory);
        
        self.cycles += cycles as u32;
        cycles
    }

    pub fn handle_interrupts(&mut self, interrupts: u8, memory: &mut Memory) {
        if !self.ime && !self.halt {
            return;
        }

        for i in 0..5 {
            let interrupt_bit = 1 << i;
            if (interrupts & interrupt_bit) != 0 {
                self.halt = false;
                
                if self.ime {
                    self.ime = false;
                    memory.clear_interrupt(interrupt_bit);
                    
                    self.push_word(self.registers.pc, memory);
                    
                    self.registers.pc = match i {
                        0 => 0x0040, // V-Blank
                        1 => 0x0048, // LCD STAT
                        2 => 0x0050, // Timer
                        3 => 0x0058, // Serial
                        4 => 0x0060, // Joypad
                        _ => unreachable!(),
                    };
                }
                break;
            }
        }
    }

    pub fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let byte = memory.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    pub fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        (high << 8) | low
    }

    pub fn push_byte(&mut self, value: u8, memory: &mut Memory) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        memory.write_byte(self.registers.sp, value);
    }

    pub fn push_word(&mut self, value: u16, memory: &mut Memory) {
        self.push_byte((value >> 8) as u8, memory);
        self.push_byte(value as u8, memory);
    }

    pub fn pop_byte(&mut self, memory: &Memory) -> u8 {
        let value = memory.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        value
    }

    pub fn pop_word(&mut self, memory: &Memory) -> u16 {
        let low = self.pop_byte(memory) as u16;
        let high = self.pop_byte(memory) as u16;
        (high << 8) | low
    }

    fn execute_opcode(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        opcodes::execute(self, opcode, memory)
    }

    pub fn and_a(&mut self, value: u8) {
        self.registers.a &= value;
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(true);
        self.registers.set_flag_c(false);
    }

    pub fn xor_a(&mut self, value: u8) {
        self.registers.a ^= value;
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);
    }

    pub fn or_a(&mut self, value: u8) {
        self.registers.a |= value;
        self.registers.set_flag_z(self.registers.a == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);
    }

    pub fn cp_a(&mut self, value: u8) {
        let result = self.registers.a.wrapping_sub(value);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(true);
        self.registers.set_flag_h((self.registers.a & 0xF) < (value & 0xF));
        self.registers.set_flag_c(self.registers.a < value);
    }

    pub fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h((value & 0xF) == 0xF);
        result
    }

    pub fn dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(true);
        self.registers.set_flag_h((value & 0xF) == 0);
        result
    }

    pub fn add_a(&mut self, value: u8) {
        let (result, carry) = self.registers.a.overflowing_add(value);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h((self.registers.a & 0xF) + (value & 0xF) > 0xF);
        self.registers.set_flag_c(carry);
        self.registers.a = result;
    }

    pub fn adc_a(&mut self, value: u8) {
        let carry = if self.registers.flag_c() { 1 } else { 0 };
        let result = self.registers.a.wrapping_add(value).wrapping_add(carry);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h((self.registers.a & 0xF) + (value & 0xF) + carry > 0xF);
        self.registers.set_flag_c((self.registers.a as u16) + (value as u16) + (carry as u16) > 0xFF);
        self.registers.a = result;
    }

    pub fn sub_a(&mut self, value: u8) {
        let (result, carry) = self.registers.a.overflowing_sub(value);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(true);
        self.registers.set_flag_h((self.registers.a & 0xF) < (value & 0xF));
        self.registers.set_flag_c(carry);
        self.registers.a = result;
    }

    pub fn sbc_a(&mut self, value: u8) {
        let carry = if self.registers.flag_c() { 1 } else { 0 };
        let result = self.registers.a.wrapping_sub(value).wrapping_sub(carry);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(true);
        self.registers.set_flag_h((self.registers.a & 0xF) < (value & 0xF) + carry);
        self.registers.set_flag_c((self.registers.a as u16) < (value as u16) + (carry as u16));
        self.registers.a = result;
    }
}