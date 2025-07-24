use super::{Cpu, opcodes_extended, opcodes_alu};
use crate::memory::Memory;

pub fn execute(cpu: &mut Cpu, opcode: u8, memory: &mut Memory) -> u8 {
    match opcode {
        // 8-bit loads
        0x06 => { cpu.registers.b = cpu.fetch_byte(memory); 8 }  // LD B, n
        0x0E => { cpu.registers.c = cpu.fetch_byte(memory); 8 }  // LD C, n
        0x16 => { cpu.registers.d = cpu.fetch_byte(memory); 8 }  // LD D, n
        0x1E => { cpu.registers.e = cpu.fetch_byte(memory); 8 }  // LD E, n
        0x26 => { cpu.registers.h = cpu.fetch_byte(memory); 8 }  // LD H, n
        0x2E => { cpu.registers.l = cpu.fetch_byte(memory); 8 }  // LD L, n
        0x3E => { cpu.registers.a = cpu.fetch_byte(memory); 8 }  // LD A, n
        
        // LD r1, r2
        0x7F => { cpu.registers.a = cpu.registers.a; 4 }  // LD A, A
        0x78 => { cpu.registers.a = cpu.registers.b; 4 }  // LD A, B
        0x79 => { cpu.registers.a = cpu.registers.c; 4 }  // LD A, C
        0x7A => { cpu.registers.a = cpu.registers.d; 4 }  // LD A, D
        0x7B => { cpu.registers.a = cpu.registers.e; 4 }  // LD A, E
        0x7C => { cpu.registers.a = cpu.registers.h; 4 }  // LD A, H
        0x7D => { cpu.registers.a = cpu.registers.l; 4 }  // LD A, L
        
        0x47 => { cpu.registers.b = cpu.registers.a; 4 }  // LD B, A
        0x4F => { cpu.registers.c = cpu.registers.a; 4 }  // LD C, A
        0x57 => { cpu.registers.d = cpu.registers.a; 4 }  // LD D, A
        0x5F => { cpu.registers.e = cpu.registers.a; 4 }  // LD E, A
        0x67 => { cpu.registers.h = cpu.registers.a; 4 }  // LD H, A
        0x6F => { cpu.registers.l = cpu.registers.a; 4 }  // LD L, A
        
        // LD A, (HL)
        0x7E => { 
            let addr = cpu.registers.hl();
            cpu.registers.a = memory.read_byte(addr);
            8
        }
        
        // LD (HL), r
        0x77 => {
            let addr = cpu.registers.hl();
            memory.write_byte(addr, cpu.registers.a);
            8
        }
        
        // 16-bit loads
        0x01 => { let v = cpu.fetch_word(memory); cpu.registers.set_bc(v); 12 }  // LD BC, nn
        0x11 => { let v = cpu.fetch_word(memory); cpu.registers.set_de(v); 12 }  // LD DE, nn
        0x21 => { let v = cpu.fetch_word(memory); cpu.registers.set_hl(v); 12 }  // LD HL, nn
        0x31 => { cpu.registers.sp = cpu.fetch_word(memory); 12 }  // LD SP, nn
        
        // Stack operations
        0xF5 => { cpu.push_word(cpu.registers.af(), memory); 16 }  // PUSH AF
        0xC5 => { cpu.push_word(cpu.registers.bc(), memory); 16 }  // PUSH BC
        0xD5 => { cpu.push_word(cpu.registers.de(), memory); 16 }  // PUSH DE
        0xE5 => { cpu.push_word(cpu.registers.hl(), memory); 16 }  // PUSH HL
        
        0xF1 => { let v = cpu.pop_word(memory); cpu.registers.set_af(v); 12 }  // POP AF
        0xC1 => { let v = cpu.pop_word(memory); cpu.registers.set_bc(v); 12 }  // POP BC
        0xD1 => { let v = cpu.pop_word(memory); cpu.registers.set_de(v); 12 }  // POP DE
        0xE1 => { let v = cpu.pop_word(memory); cpu.registers.set_hl(v); 12 }  // POP HL
        
        // Jumps
        0xC3 => { cpu.registers.pc = cpu.fetch_word(memory); 16 }  // JP nn
        0x18 => { // JR n
            let offset = cpu.fetch_byte(memory) as i8;
            cpu.registers.pc = ((cpu.registers.pc as i32) + (offset as i32)) as u16;
            12
        }
        
        // Calls
        0xCD => { // CALL nn
            let addr = cpu.fetch_word(memory);
            cpu.push_word(cpu.registers.pc, memory);
            cpu.registers.pc = addr;
            24
        }
        
        // Returns
        0xC9 => { // RET
            cpu.registers.pc = cpu.pop_word(memory);
            16
        }
        
        // ALU operations
        0xA7 => { cpu.and_a(cpu.registers.a); 4 }  // AND A
        0xAF => { cpu.xor_a(cpu.registers.a); 4 }  // XOR A
        0xB7 => { cpu.or_a(cpu.registers.a); 4 }   // OR A
        0xFE => { // CP n
            let n = cpu.fetch_byte(memory);
            cpu.cp_a(n);
            8
        }
        
        // Increment/Decrement
        0x3C => { cpu.registers.a = cpu.inc(cpu.registers.a); 4 }  // INC A
        0x04 => { cpu.registers.b = cpu.inc(cpu.registers.b); 4 }  // INC B
        0x0C => { cpu.registers.c = cpu.inc(cpu.registers.c); 4 }  // INC C
        0x14 => { cpu.registers.d = cpu.inc(cpu.registers.d); 4 }  // INC D
        0x1C => { cpu.registers.e = cpu.inc(cpu.registers.e); 4 }  // INC E
        0x24 => { cpu.registers.h = cpu.inc(cpu.registers.h); 4 }  // INC H
        0x2C => { cpu.registers.l = cpu.inc(cpu.registers.l); 4 }  // INC L
        
        0x3D => { cpu.registers.a = cpu.dec(cpu.registers.a); 4 }  // DEC A
        0x05 => { cpu.registers.b = cpu.dec(cpu.registers.b); 4 }  // DEC B
        0x0D => { cpu.registers.c = cpu.dec(cpu.registers.c); 4 }  // DEC C
        0x15 => { cpu.registers.d = cpu.dec(cpu.registers.d); 4 }  // DEC D
        0x1D => { cpu.registers.e = cpu.dec(cpu.registers.e); 4 }  // DEC E
        0x25 => { cpu.registers.h = cpu.dec(cpu.registers.h); 4 }  // DEC H
        0x2D => { cpu.registers.l = cpu.dec(cpu.registers.l); 4 }  // DEC L
        
        // Control
        0x00 => 4,  // NOP
        0x76 => {   // HALT
            cpu.halt = true;
            4
        }
        0xF3 => { cpu.ime = false; 4 }  // DI
        0xFB => { cpu.ime = true; 4 }   // EI
        
        // CB prefix
        0xCB => {
            let cb_opcode = cpu.fetch_byte(memory);
            execute_cb(cpu, cb_opcode, memory) + 4
        }
        
        _ => {
            // Try ALU opcodes first
            let cycles = opcodes_alu::execute_alu(cpu, opcode, memory);
            if cycles != 0 {
                return cycles;
            }
            
            // Try extended opcodes
            let cycles = opcodes_extended::execute_extended(cpu, opcode, memory);
            if cycles == 0 {
                panic!("Unimplemented opcode: 0x{:02X} at PC: 0x{:04X}", opcode, cpu.registers.pc.wrapping_sub(1));
            }
            cycles
        }
    }
}

fn execute_cb(cpu: &mut Cpu, opcode: u8, memory: &mut Memory) -> u8 {
    super::opcodes_cb::execute_cb(cpu, opcode, memory)
}