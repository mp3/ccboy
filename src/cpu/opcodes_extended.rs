use super::Cpu;
use crate::memory::Memory;

pub fn execute_extended(cpu: &mut Cpu, opcode: u8, memory: &mut Memory) -> u8 {
    match opcode {
        // More 8-bit loads
        0x02 => { // LD (BC), A
            let addr = cpu.registers.bc();
            memory.write_byte(addr, cpu.registers.a);
            8
        }
        0x12 => { // LD (DE), A
            let addr = cpu.registers.de();
            memory.write_byte(addr, cpu.registers.a);
            8
        }
        0x0A => { // LD A, (BC)
            let addr = cpu.registers.bc();
            cpu.registers.a = memory.read_byte(addr);
            8
        }
        0x1A => { // LD A, (DE)
            let addr = cpu.registers.de();
            cpu.registers.a = memory.read_byte(addr);
            8
        }
        0x22 => { // LD (HL+), A / LDI (HL), A
            let addr = cpu.registers.hl();
            memory.write_byte(addr, cpu.registers.a);
            cpu.registers.set_hl(addr.wrapping_add(1));
            8
        }
        0x2A => { // LD A, (HL+) / LDI A, (HL)
            let addr = cpu.registers.hl();
            cpu.registers.a = memory.read_byte(addr);
            cpu.registers.set_hl(addr.wrapping_add(1));
            8
        }
        0x32 => { // LD (HL-), A / LDD (HL), A
            let addr = cpu.registers.hl();
            memory.write_byte(addr, cpu.registers.a);
            cpu.registers.set_hl(addr.wrapping_sub(1));
            8
        }
        0x3A => { // LD A, (HL-) / LDD A, (HL)
            let addr = cpu.registers.hl();
            cpu.registers.a = memory.read_byte(addr);
            cpu.registers.set_hl(addr.wrapping_sub(1));
            8
        }
        
        // 16-bit arithmetic
        0x09 => { // ADD HL, BC
            let hl = cpu.registers.hl();
            let bc = cpu.registers.bc();
            let result = hl.wrapping_add(bc);
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h((hl & 0xFFF) + (bc & 0xFFF) > 0xFFF);
            cpu.registers.set_flag_c(hl > 0xFFFF - bc);
            cpu.registers.set_hl(result);
            8
        }
        0x19 => { // ADD HL, DE
            let hl = cpu.registers.hl();
            let de = cpu.registers.de();
            let result = hl.wrapping_add(de);
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h((hl & 0xFFF) + (de & 0xFFF) > 0xFFF);
            cpu.registers.set_flag_c(hl > 0xFFFF - de);
            cpu.registers.set_hl(result);
            8
        }
        0x29 => { // ADD HL, HL
            let hl = cpu.registers.hl();
            let result = hl.wrapping_add(hl);
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h((hl & 0xFFF) + (hl & 0xFFF) > 0xFFF);
            cpu.registers.set_flag_c(hl > 0xFFFF - hl);
            cpu.registers.set_hl(result);
            8
        }
        0x39 => { // ADD HL, SP
            let hl = cpu.registers.hl();
            let sp = cpu.registers.sp;
            let result = hl.wrapping_add(sp);
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h((hl & 0xFFF) + (sp & 0xFFF) > 0xFFF);
            cpu.registers.set_flag_c(hl > 0xFFFF - sp);
            cpu.registers.set_hl(result);
            8
        }
        
        // 16-bit inc/dec
        0x03 => { let v = cpu.registers.bc().wrapping_add(1); cpu.registers.set_bc(v); 8 }  // INC BC
        0x13 => { let v = cpu.registers.de().wrapping_add(1); cpu.registers.set_de(v); 8 }  // INC DE
        0x23 => { let v = cpu.registers.hl().wrapping_add(1); cpu.registers.set_hl(v); 8 }  // INC HL
        0x33 => { cpu.registers.sp = cpu.registers.sp.wrapping_add(1); 8 }  // INC SP
        
        0x0B => { let v = cpu.registers.bc().wrapping_sub(1); cpu.registers.set_bc(v); 8 }  // DEC BC
        0x1B => { let v = cpu.registers.de().wrapping_sub(1); cpu.registers.set_de(v); 8 }  // DEC DE
        0x2B => { let v = cpu.registers.hl().wrapping_sub(1); cpu.registers.set_hl(v); 8 }  // DEC HL
        0x3B => { cpu.registers.sp = cpu.registers.sp.wrapping_sub(1); 8 }  // DEC SP
        
        // Rotates
        0x07 => { // RLCA
            let carry = (cpu.registers.a & 0x80) >> 7;
            cpu.registers.a = (cpu.registers.a << 1) | carry;
            cpu.registers.set_flag_z(false);
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h(false);
            cpu.registers.set_flag_c(carry != 0);
            4
        }
        0x0F => { // RRCA
            let carry = cpu.registers.a & 0x01;
            cpu.registers.a = (cpu.registers.a >> 1) | (carry << 7);
            cpu.registers.set_flag_z(false);
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h(false);
            cpu.registers.set_flag_c(carry != 0);
            4
        }
        0x17 => { // RLA
            let carry = (cpu.registers.a & 0x80) >> 7;
            let old_carry = if cpu.registers.flag_c() { 1 } else { 0 };
            cpu.registers.a = (cpu.registers.a << 1) | old_carry;
            cpu.registers.set_flag_z(false);
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h(false);
            cpu.registers.set_flag_c(carry != 0);
            4
        }
        0x1F => { // RRA
            let carry = cpu.registers.a & 0x01;
            let old_carry = if cpu.registers.flag_c() { 0x80 } else { 0 };
            cpu.registers.a = (cpu.registers.a >> 1) | old_carry;
            cpu.registers.set_flag_z(false);
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h(false);
            cpu.registers.set_flag_c(carry != 0);
            4
        }
        
        // Conditional jumps
        0x20 => { // JR NZ, n
            let offset = cpu.fetch_byte(memory) as i8;
            if !cpu.registers.flag_z() {
                cpu.registers.pc = ((cpu.registers.pc as i32) + (offset as i32)) as u16;
                12
            } else {
                8
            }
        }
        0x28 => { // JR Z, n
            let offset = cpu.fetch_byte(memory) as i8;
            if cpu.registers.flag_z() {
                cpu.registers.pc = ((cpu.registers.pc as i32) + (offset as i32)) as u16;
                12
            } else {
                8
            }
        }
        0x30 => { // JR NC, n
            let offset = cpu.fetch_byte(memory) as i8;
            if !cpu.registers.flag_c() {
                cpu.registers.pc = ((cpu.registers.pc as i32) + (offset as i32)) as u16;
                12
            } else {
                8
            }
        }
        0x38 => { // JR C, n
            let offset = cpu.fetch_byte(memory) as i8;
            if cpu.registers.flag_c() {
                cpu.registers.pc = ((cpu.registers.pc as i32) + (offset as i32)) as u16;
                12
            } else {
                8
            }
        }
        
        // Conditional absolute jumps
        0xC2 => { // JP NZ, nn
            let addr = cpu.fetch_word(memory);
            if !cpu.registers.flag_z() {
                cpu.registers.pc = addr;
                16
            } else {
                12
            }
        }
        0xCA => { // JP Z, nn
            let addr = cpu.fetch_word(memory);
            if cpu.registers.flag_z() {
                cpu.registers.pc = addr;
                16
            } else {
                12
            }
        }
        0xD2 => { // JP NC, nn
            let addr = cpu.fetch_word(memory);
            if !cpu.registers.flag_c() {
                cpu.registers.pc = addr;
                16
            } else {
                12
            }
        }
        0xDA => { // JP C, nn
            let addr = cpu.fetch_word(memory);
            if cpu.registers.flag_c() {
                cpu.registers.pc = addr;
                16
            } else {
                12
            }
        }
        
        // Conditional calls
        0xC4 => { // CALL NZ, nn
            let addr = cpu.fetch_word(memory);
            if !cpu.registers.flag_z() {
                cpu.push_word(cpu.registers.pc, memory);
                cpu.registers.pc = addr;
                24
            } else {
                12
            }
        }
        0xCC => { // CALL Z, nn
            let addr = cpu.fetch_word(memory);
            if cpu.registers.flag_z() {
                cpu.push_word(cpu.registers.pc, memory);
                cpu.registers.pc = addr;
                24
            } else {
                12
            }
        }
        0xD4 => { // CALL NC, nn
            let addr = cpu.fetch_word(memory);
            if !cpu.registers.flag_c() {
                cpu.push_word(cpu.registers.pc, memory);
                cpu.registers.pc = addr;
                24
            } else {
                12
            }
        }
        0xDC => { // CALL C, nn
            let addr = cpu.fetch_word(memory);
            if cpu.registers.flag_c() {
                cpu.push_word(cpu.registers.pc, memory);
                cpu.registers.pc = addr;
                24
            } else {
                12
            }
        }
        
        // Conditional returns
        0xC0 => { // RET NZ
            if !cpu.registers.flag_z() {
                cpu.registers.pc = cpu.pop_word(memory);
                20
            } else {
                8
            }
        }
        0xC8 => { // RET Z
            if cpu.registers.flag_z() {
                cpu.registers.pc = cpu.pop_word(memory);
                20
            } else {
                8
            }
        }
        0xD0 => { // RET NC
            if !cpu.registers.flag_c() {
                cpu.registers.pc = cpu.pop_word(memory);
                20
            } else {
                8
            }
        }
        0xD8 => { // RET C
            if cpu.registers.flag_c() {
                cpu.registers.pc = cpu.pop_word(memory);
                20
            } else {
                8
            }
        }
        0xD9 => { // RETI
            cpu.registers.pc = cpu.pop_word(memory);
            cpu.ime = true;
            16
        }
        
        // RST instructions
        0xC7 => { cpu.push_word(cpu.registers.pc, memory); cpu.registers.pc = 0x00; 16 }  // RST 00H
        0xCF => { cpu.push_word(cpu.registers.pc, memory); cpu.registers.pc = 0x08; 16 }  // RST 08H
        0xD7 => { cpu.push_word(cpu.registers.pc, memory); cpu.registers.pc = 0x10; 16 }  // RST 10H
        0xDF => { cpu.push_word(cpu.registers.pc, memory); cpu.registers.pc = 0x18; 16 }  // RST 18H
        0xE7 => { cpu.push_word(cpu.registers.pc, memory); cpu.registers.pc = 0x20; 16 }  // RST 20H
        0xEF => { cpu.push_word(cpu.registers.pc, memory); cpu.registers.pc = 0x28; 16 }  // RST 28H
        0xF7 => { cpu.push_word(cpu.registers.pc, memory); cpu.registers.pc = 0x30; 16 }  // RST 30H
        0xFF => { cpu.push_word(cpu.registers.pc, memory); cpu.registers.pc = 0x38; 16 }  // RST 38H
        
        // Misc
        0x27 => { // DAA
            let mut a = cpu.registers.a;
            let mut adjust = 0;
            
            if cpu.registers.flag_h() || (!cpu.registers.flag_n() && (a & 0xF) > 9) {
                adjust |= 0x06;
            }
            
            if cpu.registers.flag_c() || (!cpu.registers.flag_n() && a > 0x99) {
                adjust |= 0x60;
                cpu.registers.set_flag_c(true);
            }
            
            if cpu.registers.flag_n() {
                a = a.wrapping_sub(adjust);
            } else {
                a = a.wrapping_add(adjust);
            }
            
            cpu.registers.a = a;
            cpu.registers.set_flag_z(a == 0);
            cpu.registers.set_flag_h(false);
            4
        }
        0x2F => { // CPL
            cpu.registers.a = !cpu.registers.a;
            cpu.registers.set_flag_n(true);
            cpu.registers.set_flag_h(true);
            4
        }
        0x37 => { // SCF
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h(false);
            cpu.registers.set_flag_c(true);
            4
        }
        0x3F => { // CCF
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h(false);
            cpu.registers.set_flag_c(!cpu.registers.flag_c());
            4
        }
        
        // I/O
        0xE0 => { // LDH (n), A
            let addr = 0xFF00 + cpu.fetch_byte(memory) as u16;
            memory.write_byte(addr, cpu.registers.a);
            12
        }
        0xF0 => { // LDH A, (n)
            let addr = 0xFF00 + cpu.fetch_byte(memory) as u16;
            cpu.registers.a = memory.read_byte(addr);
            12
        }
        0xE2 => { // LD (C), A
            let addr = 0xFF00 + cpu.registers.c as u16;
            memory.write_byte(addr, cpu.registers.a);
            8
        }
        0xF2 => { // LD A, (C)
            let addr = 0xFF00 + cpu.registers.c as u16;
            cpu.registers.a = memory.read_byte(addr);
            8
        }
        
        // Direct address loads
        0xEA => { // LD (nn), A
            let addr = cpu.fetch_word(memory);
            memory.write_byte(addr, cpu.registers.a);
            16
        }
        0xFA => { // LD A, (nn)
            let addr = cpu.fetch_word(memory);
            cpu.registers.a = memory.read_byte(addr);
            16
        }
        
        // SP operations
        0xE8 => { // ADD SP, n
            let offset = cpu.fetch_byte(memory) as i8 as i16 as u16;
            let sp = cpu.registers.sp;
            let result = sp.wrapping_add(offset);
            
            cpu.registers.set_flag_z(false);
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h((sp & 0xF) + (offset & 0xF) > 0xF);
            cpu.registers.set_flag_c((sp & 0xFF) + (offset & 0xFF) > 0xFF);
            
            cpu.registers.sp = result;
            16
        }
        0xF8 => { // LD HL, SP+n
            let offset = cpu.fetch_byte(memory) as i8 as i16 as u16;
            let sp = cpu.registers.sp;
            let result = sp.wrapping_add(offset);
            
            cpu.registers.set_flag_z(false);
            cpu.registers.set_flag_n(false);
            cpu.registers.set_flag_h((sp & 0xF) + (offset & 0xF) > 0xF);
            cpu.registers.set_flag_c((sp & 0xFF) + (offset & 0xFF) > 0xFF);
            
            cpu.registers.set_hl(result);
            12
        }
        0xF9 => { // LD SP, HL
            cpu.registers.sp = cpu.registers.hl();
            8
        }
        0x08 => { // LD (nn), SP
            let addr = cpu.fetch_word(memory);
            let sp = cpu.registers.sp;
            memory.write_byte(addr, sp as u8);
            memory.write_byte(addr.wrapping_add(1), (sp >> 8) as u8);
            20
        }
        
        // JP HL
        0xE9 => { // JP (HL)
            cpu.registers.pc = cpu.registers.hl();
            4
        }
        
        // STOP and extended INC/DEC
        0x10 => { // STOP 0
            cpu.fetch_byte(memory); // Consume the next byte (should be 0x00)
            // TODO: Implement proper STOP behavior
            4
        }
        0x34 => { // INC (HL)
            let addr = cpu.registers.hl();
            let value = memory.read_byte(addr);
            let result = cpu.inc(value);
            memory.write_byte(addr, result);
            12
        }
        0x35 => { // DEC (HL)
            let addr = cpu.registers.hl();
            let value = memory.read_byte(addr);
            let result = cpu.dec(value);
            memory.write_byte(addr, result);
            12
        }
        
        _ => 0, // Return 0 to indicate this opcode should be handled elsewhere
    }
}