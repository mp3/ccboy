use crate::cpu::Cpu;
use crate::memory::Memory;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct CpuState {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub sp: u16,
    pub pc: u16,
    pub ime: bool,
    pub halt: bool,
}

impl CpuState {
    pub fn from_cpu(cpu: &Cpu) -> Self {
        Self {
            a: cpu.registers.a,
            b: cpu.registers.b,
            c: cpu.registers.c,
            d: cpu.registers.d,
            e: cpu.registers.e,
            h: cpu.registers.h,
            l: cpu.registers.l,
            f: cpu.registers.f,
            sp: cpu.registers.sp,
            pc: cpu.registers.pc,
            ime: cpu.ime,
            halt: cpu.halt,
        }
    }
}

#[wasm_bindgen]
impl CpuState {
    
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    
    pub fn flag_z(&self) -> bool {
        (self.f & 0x80) != 0
    }
    
    pub fn flag_n(&self) -> bool {
        (self.f & 0x40) != 0
    }
    
    pub fn flag_h(&self) -> bool {
        (self.f & 0x20) != 0
    }
    
    pub fn flag_c(&self) -> bool {
        (self.f & 0x10) != 0
    }
}

pub struct Debugger;

impl Debugger {
    pub fn disassemble(memory: &Memory, address: u16) -> (String, u8) {
        let opcode = memory.read_byte(address);
        let mut bytes_used = 1;
        
        let instruction = match opcode {
            0x00 => "NOP".to_string(),
            0x01 => {
                bytes_used = 3;
                let low = memory.read_byte(address + 1);
                let high = memory.read_byte(address + 2);
                format!("LD BC, ${:04X}", ((high as u16) << 8) | (low as u16))
            }
            0x02 => "LD (BC), A".to_string(),
            0x03 => "INC BC".to_string(),
            0x04 => "INC B".to_string(),
            0x05 => "DEC B".to_string(),
            0x06 => {
                bytes_used = 2;
                format!("LD B, ${:02X}", memory.read_byte(address + 1))
            }
            0x07 => "RLCA".to_string(),
            0x08 => {
                bytes_used = 3;
                let low = memory.read_byte(address + 1);
                let high = memory.read_byte(address + 2);
                format!("LD (${:04X}), SP", ((high as u16) << 8) | (low as u16))
            }
            0x09 => "ADD HL, BC".to_string(),
            0x0A => "LD A, (BC)".to_string(),
            0x0B => "DEC BC".to_string(),
            0x0C => "INC C".to_string(),
            0x0D => "DEC C".to_string(),
            0x0E => {
                bytes_used = 2;
                format!("LD C, ${:02X}", memory.read_byte(address + 1))
            }
            0x0F => "RRCA".to_string(),
            
            0x10 => {
                bytes_used = 2;
                "STOP".to_string()
            }
            0x11 => {
                bytes_used = 3;
                let low = memory.read_byte(address + 1);
                let high = memory.read_byte(address + 2);
                format!("LD DE, ${:04X}", ((high as u16) << 8) | (low as u16))
            }
            0x12 => "LD (DE), A".to_string(),
            0x13 => "INC DE".to_string(),
            0x14 => "INC D".to_string(),
            0x15 => "DEC D".to_string(),
            0x16 => {
                bytes_used = 2;
                format!("LD D, ${:02X}", memory.read_byte(address + 1))
            }
            0x17 => "RLA".to_string(),
            0x18 => {
                bytes_used = 2;
                let offset = memory.read_byte(address + 1) as i8;
                format!("JR {:+}", offset)
            }
            0x19 => "ADD HL, DE".to_string(),
            0x1A => "LD A, (DE)".to_string(),
            0x1B => "DEC DE".to_string(),
            0x1C => "INC E".to_string(),
            0x1D => "DEC E".to_string(),
            0x1E => {
                bytes_used = 2;
                format!("LD E, ${:02X}", memory.read_byte(address + 1))
            }
            0x1F => "RRA".to_string(),
            
            0x20 => {
                bytes_used = 2;
                let offset = memory.read_byte(address + 1) as i8;
                format!("JR NZ, {:+}", offset)
            }
            0x21 => {
                bytes_used = 3;
                let low = memory.read_byte(address + 1);
                let high = memory.read_byte(address + 2);
                format!("LD HL, ${:04X}", ((high as u16) << 8) | (low as u16))
            }
            0x22 => "LD (HL+), A".to_string(),
            0x23 => "INC HL".to_string(),
            0x24 => "INC H".to_string(),
            0x25 => "DEC H".to_string(),
            0x26 => {
                bytes_used = 2;
                format!("LD H, ${:02X}", memory.read_byte(address + 1))
            }
            0x27 => "DAA".to_string(),
            0x28 => {
                bytes_used = 2;
                let offset = memory.read_byte(address + 1) as i8;
                format!("JR Z, {:+}", offset)
            }
            0x29 => "ADD HL, HL".to_string(),
            0x2A => "LD A, (HL+)".to_string(),
            0x2B => "DEC HL".to_string(),
            0x2C => "INC L".to_string(),
            0x2D => "DEC L".to_string(),
            0x2E => {
                bytes_used = 2;
                format!("LD L, ${:02X}", memory.read_byte(address + 1))
            }
            0x2F => "CPL".to_string(),
            
            0x30 => {
                bytes_used = 2;
                let offset = memory.read_byte(address + 1) as i8;
                format!("JR NC, {:+}", offset)
            }
            0x31 => {
                bytes_used = 3;
                let low = memory.read_byte(address + 1);
                let high = memory.read_byte(address + 2);
                format!("LD SP, ${:04X}", ((high as u16) << 8) | (low as u16))
            }
            0x32 => "LD (HL-), A".to_string(),
            0x33 => "INC SP".to_string(),
            0x34 => "INC (HL)".to_string(),
            0x35 => "DEC (HL)".to_string(),
            0x36 => {
                bytes_used = 2;
                format!("LD (HL), ${:02X}", memory.read_byte(address + 1))
            }
            0x37 => "SCF".to_string(),
            0x38 => {
                bytes_used = 2;
                let offset = memory.read_byte(address + 1) as i8;
                format!("JR C, {:+}", offset)
            }
            0x39 => "ADD HL, SP".to_string(),
            0x3A => "LD A, (HL-)".to_string(),
            0x3B => "DEC SP".to_string(),
            0x3C => "INC A".to_string(),
            0x3D => "DEC A".to_string(),
            0x3E => {
                bytes_used = 2;
                format!("LD A, ${:02X}", memory.read_byte(address + 1))
            }
            0x3F => "CCF".to_string(),
            
            0x76 => "HALT".to_string(),
            0x7E => "LD A, (HL)".to_string(),
            0x77 => "LD (HL), A".to_string(),
            
            0xC3 => {
                bytes_used = 3;
                let low = memory.read_byte(address + 1);
                let high = memory.read_byte(address + 2);
                format!("JP ${:04X}", ((high as u16) << 8) | (low as u16))
            }
            0xC9 => "RET".to_string(),
            0xCB => {
                bytes_used = 2;
                let cb_opcode = memory.read_byte(address + 1);
                disassemble_cb(cb_opcode)
            }
            0xCD => {
                bytes_used = 3;
                let low = memory.read_byte(address + 1);
                let high = memory.read_byte(address + 2);
                format!("CALL ${:04X}", ((high as u16) << 8) | (low as u16))
            }
            
            0xE0 => {
                bytes_used = 2;
                format!("LDH (${:02X}), A", memory.read_byte(address + 1))
            }
            0xE2 => "LD (C), A".to_string(),
            0xEA => {
                bytes_used = 3;
                let low = memory.read_byte(address + 1);
                let high = memory.read_byte(address + 2);
                format!("LD (${:04X}), A", ((high as u16) << 8) | (low as u16))
            }
            
            0xF0 => {
                bytes_used = 2;
                format!("LDH A, (${:02X})", memory.read_byte(address + 1))
            }
            0xF2 => "LD A, (C)".to_string(),
            0xF3 => "DI".to_string(),
            0xFA => {
                bytes_used = 3;
                let low = memory.read_byte(address + 1);
                let high = memory.read_byte(address + 2);
                format!("LD A, (${:04X})", ((high as u16) << 8) | (low as u16))
            }
            0xFB => "EI".to_string(),
            0xFE => {
                bytes_used = 2;
                format!("CP ${:02X}", memory.read_byte(address + 1))
            }
            
            _ => format!("DB ${:02X}", opcode),
        };
        
        (instruction, bytes_used)
    }
}

fn disassemble_cb(opcode: u8) -> String {
    let reg_names = ["B", "C", "D", "E", "H", "L", "(HL)", "A"];
    let reg = opcode & 0x07;
    let bit = (opcode >> 3) & 0x07;
    
    match opcode >> 6 {
        0 => {
            match bit {
                0 => format!("RLC {}", reg_names[reg as usize]),
                1 => format!("RRC {}", reg_names[reg as usize]),
                2 => format!("RL {}", reg_names[reg as usize]),
                3 => format!("RR {}", reg_names[reg as usize]),
                4 => format!("SLA {}", reg_names[reg as usize]),
                5 => format!("SRA {}", reg_names[reg as usize]),
                6 => format!("SWAP {}", reg_names[reg as usize]),
                7 => format!("SRL {}", reg_names[reg as usize]),
                _ => unreachable!(),
            }
        }
        1 => format!("BIT {}, {}", bit, reg_names[reg as usize]),
        2 => format!("RES {}, {}", bit, reg_names[reg as usize]),
        3 => format!("SET {}, {}", bit, reg_names[reg as usize]),
        _ => unreachable!(),
    }
}