use super::Cpu;
use crate::memory::Memory;

pub fn execute_alu(cpu: &mut Cpu, opcode: u8, memory: &mut Memory) -> u8 {
    match opcode {
        // Missing LD r, r instructions
        0x40 => { cpu.registers.b = cpu.registers.b; 4 }  // LD B, B
        0x41 => { cpu.registers.b = cpu.registers.c; 4 }  // LD B, C
        0x42 => { cpu.registers.b = cpu.registers.d; 4 }  // LD B, D
        0x43 => { cpu.registers.b = cpu.registers.e; 4 }  // LD B, E
        0x44 => { cpu.registers.b = cpu.registers.h; 4 }  // LD B, H
        0x45 => { cpu.registers.b = cpu.registers.l; 4 }  // LD B, L
        0x46 => { // LD B, (HL)
            let addr = cpu.registers.hl();
            cpu.registers.b = memory.read_byte(addr);
            8
        }
        
        0x48 => { cpu.registers.c = cpu.registers.b; 4 }  // LD C, B
        0x49 => { cpu.registers.c = cpu.registers.c; 4 }  // LD C, C
        0x4A => { cpu.registers.c = cpu.registers.d; 4 }  // LD C, D
        0x4B => { cpu.registers.c = cpu.registers.e; 4 }  // LD C, E
        0x4C => { cpu.registers.c = cpu.registers.h; 4 }  // LD C, H
        0x4D => { cpu.registers.c = cpu.registers.l; 4 }  // LD C, L
        0x4E => { // LD C, (HL)
            let addr = cpu.registers.hl();
            cpu.registers.c = memory.read_byte(addr);
            8
        }
        
        0x50 => { cpu.registers.d = cpu.registers.b; 4 }  // LD D, B
        0x51 => { cpu.registers.d = cpu.registers.c; 4 }  // LD D, C
        0x52 => { cpu.registers.d = cpu.registers.d; 4 }  // LD D, D
        0x53 => { cpu.registers.d = cpu.registers.e; 4 }  // LD D, E
        0x54 => { cpu.registers.d = cpu.registers.h; 4 }  // LD D, H
        0x55 => { cpu.registers.d = cpu.registers.l; 4 }  // LD D, L
        0x56 => { // LD D, (HL)
            let addr = cpu.registers.hl();
            cpu.registers.d = memory.read_byte(addr);
            8
        }
        
        0x58 => { cpu.registers.e = cpu.registers.b; 4 }  // LD E, B
        0x59 => { cpu.registers.e = cpu.registers.c; 4 }  // LD E, C
        0x5A => { cpu.registers.e = cpu.registers.d; 4 }  // LD E, D
        0x5B => { cpu.registers.e = cpu.registers.e; 4 }  // LD E, E
        0x5C => { cpu.registers.e = cpu.registers.h; 4 }  // LD E, H
        0x5D => { cpu.registers.e = cpu.registers.l; 4 }  // LD E, L
        0x5E => { // LD E, (HL)
            let addr = cpu.registers.hl();
            cpu.registers.e = memory.read_byte(addr);
            8
        }
        
        0x60 => { cpu.registers.h = cpu.registers.b; 4 }  // LD H, B
        0x61 => { cpu.registers.h = cpu.registers.c; 4 }  // LD H, C
        0x62 => { cpu.registers.h = cpu.registers.d; 4 }  // LD H, D
        0x63 => { cpu.registers.h = cpu.registers.e; 4 }  // LD H, E
        0x64 => { cpu.registers.h = cpu.registers.h; 4 }  // LD H, H
        0x65 => { cpu.registers.h = cpu.registers.l; 4 }  // LD H, L
        0x66 => { // LD H, (HL)
            let addr = cpu.registers.hl();
            cpu.registers.h = memory.read_byte(addr);
            8
        }
        
        0x68 => { cpu.registers.l = cpu.registers.b; 4 }  // LD L, B
        0x69 => { cpu.registers.l = cpu.registers.c; 4 }  // LD L, C
        0x6A => { cpu.registers.l = cpu.registers.d; 4 }  // LD L, D
        0x6B => { cpu.registers.l = cpu.registers.e; 4 }  // LD L, E
        0x6C => { cpu.registers.l = cpu.registers.h; 4 }  // LD L, H
        0x6D => { cpu.registers.l = cpu.registers.l; 4 }  // LD L, L
        0x6E => { // LD L, (HL)
            let addr = cpu.registers.hl();
            cpu.registers.l = memory.read_byte(addr);
            8
        }
        
        // LD (HL), r
        0x70 => {
            let addr = cpu.registers.hl();
            memory.write_byte(addr, cpu.registers.b);
            8
        }
        0x71 => {
            let addr = cpu.registers.hl();
            memory.write_byte(addr, cpu.registers.c);
            8
        }
        0x72 => {
            let addr = cpu.registers.hl();
            memory.write_byte(addr, cpu.registers.d);
            8
        }
        0x73 => {
            let addr = cpu.registers.hl();
            memory.write_byte(addr, cpu.registers.e);
            8
        }
        0x74 => {
            let addr = cpu.registers.hl();
            memory.write_byte(addr, cpu.registers.h);
            8
        }
        0x75 => {
            let addr = cpu.registers.hl();
            memory.write_byte(addr, cpu.registers.l);
            8
        }
        
        // LD (HL), n
        0x36 => {
            let addr = cpu.registers.hl();
            let value = cpu.fetch_byte(memory);
            memory.write_byte(addr, value);
            12
        }
        
        // ALU operations - ADD
        0x80 => { cpu.add_a(cpu.registers.b); 4 }  // ADD A, B
        0x81 => { cpu.add_a(cpu.registers.c); 4 }  // ADD A, C
        0x82 => { cpu.add_a(cpu.registers.d); 4 }  // ADD A, D
        0x83 => { cpu.add_a(cpu.registers.e); 4 }  // ADD A, E
        0x84 => { cpu.add_a(cpu.registers.h); 4 }  // ADD A, H
        0x85 => { cpu.add_a(cpu.registers.l); 4 }  // ADD A, L
        0x86 => { // ADD A, (HL)
            let addr = cpu.registers.hl();
            let value = memory.read_byte(addr);
            cpu.add_a(value);
            8
        }
        0x87 => { cpu.add_a(cpu.registers.a); 4 }  // ADD A, A
        0xC6 => { // ADD A, n
            let n = cpu.fetch_byte(memory);
            cpu.add_a(n);
            8
        }
        
        // ADC (Add with Carry)
        0x88 => { cpu.adc_a(cpu.registers.b); 4 }  // ADC A, B
        0x89 => { cpu.adc_a(cpu.registers.c); 4 }  // ADC A, C
        0x8A => { cpu.adc_a(cpu.registers.d); 4 }  // ADC A, D
        0x8B => { cpu.adc_a(cpu.registers.e); 4 }  // ADC A, E
        0x8C => { cpu.adc_a(cpu.registers.h); 4 }  // ADC A, H
        0x8D => { cpu.adc_a(cpu.registers.l); 4 }  // ADC A, L
        0x8E => { // ADC A, (HL)
            let addr = cpu.registers.hl();
            let value = memory.read_byte(addr);
            cpu.adc_a(value);
            8
        }
        0x8F => { cpu.adc_a(cpu.registers.a); 4 }  // ADC A, A
        0xCE => { // ADC A, n
            let n = cpu.fetch_byte(memory);
            cpu.adc_a(n);
            8
        }
        
        // SUB
        0x90 => { cpu.sub_a(cpu.registers.b); 4 }  // SUB B
        0x91 => { cpu.sub_a(cpu.registers.c); 4 }  // SUB C
        0x92 => { cpu.sub_a(cpu.registers.d); 4 }  // SUB D
        0x93 => { cpu.sub_a(cpu.registers.e); 4 }  // SUB E
        0x94 => { cpu.sub_a(cpu.registers.h); 4 }  // SUB H
        0x95 => { cpu.sub_a(cpu.registers.l); 4 }  // SUB L
        0x96 => { // SUB (HL)
            let addr = cpu.registers.hl();
            let value = memory.read_byte(addr);
            cpu.sub_a(value);
            8
        }
        0x97 => { cpu.sub_a(cpu.registers.a); 4 }  // SUB A
        0xD6 => { // SUB n
            let n = cpu.fetch_byte(memory);
            cpu.sub_a(n);
            8
        }
        
        // SBC (Subtract with Carry)
        0x98 => { cpu.sbc_a(cpu.registers.b); 4 }  // SBC A, B
        0x99 => { cpu.sbc_a(cpu.registers.c); 4 }  // SBC A, C
        0x9A => { cpu.sbc_a(cpu.registers.d); 4 }  // SBC A, D
        0x9B => { cpu.sbc_a(cpu.registers.e); 4 }  // SBC A, E
        0x9C => { cpu.sbc_a(cpu.registers.h); 4 }  // SBC A, H
        0x9D => { cpu.sbc_a(cpu.registers.l); 4 }  // SBC A, L
        0x9E => { // SBC A, (HL)
            let addr = cpu.registers.hl();
            let value = memory.read_byte(addr);
            cpu.sbc_a(value);
            8
        }
        0x9F => { cpu.sbc_a(cpu.registers.a); 4 }  // SBC A, A
        0xDE => { // SBC A, n
            let n = cpu.fetch_byte(memory);
            cpu.sbc_a(n);
            8
        }
        
        // AND
        0xA0 => { cpu.and_a(cpu.registers.b); 4 }  // AND B
        0xA1 => { cpu.and_a(cpu.registers.c); 4 }  // AND C
        0xA2 => { cpu.and_a(cpu.registers.d); 4 }  // AND D
        0xA3 => { cpu.and_a(cpu.registers.e); 4 }  // AND E
        0xA4 => { cpu.and_a(cpu.registers.h); 4 }  // AND H
        0xA5 => { cpu.and_a(cpu.registers.l); 4 }  // AND L
        0xA6 => { // AND (HL)
            let addr = cpu.registers.hl();
            let value = memory.read_byte(addr);
            cpu.and_a(value);
            8
        }
        0xE6 => { // AND n
            let n = cpu.fetch_byte(memory);
            cpu.and_a(n);
            8
        }
        
        // XOR
        0xA8 => { cpu.xor_a(cpu.registers.b); 4 }  // XOR B
        0xA9 => { cpu.xor_a(cpu.registers.c); 4 }  // XOR C
        0xAA => { cpu.xor_a(cpu.registers.d); 4 }  // XOR D
        0xAB => { cpu.xor_a(cpu.registers.e); 4 }  // XOR E
        0xAC => { cpu.xor_a(cpu.registers.h); 4 }  // XOR H
        0xAD => { cpu.xor_a(cpu.registers.l); 4 }  // XOR L
        0xAE => { // XOR (HL)
            let addr = cpu.registers.hl();
            let value = memory.read_byte(addr);
            cpu.xor_a(value);
            8
        }
        0xEE => { // XOR n
            let n = cpu.fetch_byte(memory);
            cpu.xor_a(n);
            8
        }
        
        // OR
        0xB0 => { cpu.or_a(cpu.registers.b); 4 }  // OR B
        0xB1 => { cpu.or_a(cpu.registers.c); 4 }  // OR C
        0xB2 => { cpu.or_a(cpu.registers.d); 4 }  // OR D
        0xB3 => { cpu.or_a(cpu.registers.e); 4 }  // OR E
        0xB4 => { cpu.or_a(cpu.registers.h); 4 }  // OR H
        0xB5 => { cpu.or_a(cpu.registers.l); 4 }  // OR L
        0xB6 => { // OR (HL)
            let addr = cpu.registers.hl();
            let value = memory.read_byte(addr);
            cpu.or_a(value);
            8
        }
        0xF6 => { // OR n
            let n = cpu.fetch_byte(memory);
            cpu.or_a(n);
            8
        }
        
        // CP (Compare)
        0xB8 => { cpu.cp_a(cpu.registers.b); 4 }  // CP B
        0xB9 => { cpu.cp_a(cpu.registers.c); 4 }  // CP C
        0xBA => { cpu.cp_a(cpu.registers.d); 4 }  // CP D
        0xBB => { cpu.cp_a(cpu.registers.e); 4 }  // CP E
        0xBC => { cpu.cp_a(cpu.registers.h); 4 }  // CP H
        0xBD => { cpu.cp_a(cpu.registers.l); 4 }  // CP L
        0xBE => { // CP (HL)
            let addr = cpu.registers.hl();
            let value = memory.read_byte(addr);
            cpu.cp_a(value);
            8
        }
        0xBF => { cpu.cp_a(cpu.registers.a); 4 }  // CP A
        
        _ => 0, // Return 0 to indicate this opcode should be handled elsewhere
    }
}