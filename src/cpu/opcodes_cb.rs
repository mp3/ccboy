use super::Cpu;
use crate::memory::Memory;

pub fn execute_cb(cpu: &mut Cpu, opcode: u8, memory: &mut Memory) -> u8 {
    let reg_index = opcode & 0x07;
    let bit_index = (opcode >> 3) & 0x07;
    let op_type = opcode >> 6;
    
    match op_type {
        0 => { // Rotations and shifts
            match bit_index {
                0 => rlc_reg(cpu, reg_index, memory),
                1 => rrc_reg(cpu, reg_index, memory),
                2 => rl_reg(cpu, reg_index, memory),
                3 => rr_reg(cpu, reg_index, memory),
                4 => sla_reg(cpu, reg_index, memory),
                5 => sra_reg(cpu, reg_index, memory),
                6 => swap_reg(cpu, reg_index, memory),
                7 => srl_reg(cpu, reg_index, memory),
                _ => unreachable!(),
            }
        }
        1 => { // BIT
            bit_reg(cpu, bit_index, reg_index, memory)
        }
        2 => { // RES
            res_reg(cpu, bit_index, reg_index, memory)
        }
        3 => { // SET
            set_reg(cpu, bit_index, reg_index, memory)
        }
        _ => unreachable!(),
    }
}

fn get_reg_value(cpu: &Cpu, index: u8, memory: &Memory) -> u8 {
    match index {
        0 => cpu.registers.b,
        1 => cpu.registers.c,
        2 => cpu.registers.d,
        3 => cpu.registers.e,
        4 => cpu.registers.h,
        5 => cpu.registers.l,
        6 => memory.read_byte(cpu.registers.hl()),
        7 => cpu.registers.a,
        _ => unreachable!(),
    }
}

fn set_reg_value(cpu: &mut Cpu, index: u8, value: u8, memory: &mut Memory) {
    match index {
        0 => cpu.registers.b = value,
        1 => cpu.registers.c = value,
        2 => cpu.registers.d = value,
        3 => cpu.registers.e = value,
        4 => cpu.registers.h = value,
        5 => cpu.registers.l = value,
        6 => memory.write_byte(cpu.registers.hl(), value),
        7 => cpu.registers.a = value,
        _ => unreachable!(),
    }
}

fn get_cycles(index: u8) -> u8 {
    if index == 6 { 16 } else { 8 }
}

fn get_bit_cycles(index: u8) -> u8 {
    if index == 6 { 12 } else { 8 }
}

// RLC - Rotate Left Circular
fn rlc_reg(cpu: &mut Cpu, index: u8, memory: &mut Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let carry = (value & 0x80) >> 7;
    let result = (value << 1) | carry;
    
    set_reg_value(cpu, index, result, memory);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry != 0);
    
    get_cycles(index)
}

// RRC - Rotate Right Circular
fn rrc_reg(cpu: &mut Cpu, index: u8, memory: &mut Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let carry = value & 0x01;
    let result = (value >> 1) | (carry << 7);
    
    set_reg_value(cpu, index, result, memory);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry != 0);
    
    get_cycles(index)
}

// RL - Rotate Left through Carry
fn rl_reg(cpu: &mut Cpu, index: u8, memory: &mut Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let old_carry = if cpu.registers.flag_c() { 1 } else { 0 };
    let new_carry = (value & 0x80) >> 7;
    let result = (value << 1) | old_carry;
    
    set_reg_value(cpu, index, result, memory);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(new_carry != 0);
    
    get_cycles(index)
}

// RR - Rotate Right through Carry
fn rr_reg(cpu: &mut Cpu, index: u8, memory: &mut Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let old_carry = if cpu.registers.flag_c() { 0x80 } else { 0 };
    let new_carry = value & 0x01;
    let result = (value >> 1) | old_carry;
    
    set_reg_value(cpu, index, result, memory);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(new_carry != 0);
    
    get_cycles(index)
}

// SLA - Shift Left Arithmetic
fn sla_reg(cpu: &mut Cpu, index: u8, memory: &mut Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let carry = (value & 0x80) >> 7;
    let result = value << 1;
    
    set_reg_value(cpu, index, result, memory);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry != 0);
    
    get_cycles(index)
}

// SRA - Shift Right Arithmetic (preserves sign bit)
fn sra_reg(cpu: &mut Cpu, index: u8, memory: &mut Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let carry = value & 0x01;
    let result = (value >> 1) | (value & 0x80);
    
    set_reg_value(cpu, index, result, memory);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry != 0);
    
    get_cycles(index)
}

// SWAP - Swap nibbles
fn swap_reg(cpu: &mut Cpu, index: u8, memory: &mut Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let result = ((value & 0x0F) << 4) | ((value & 0xF0) >> 4);
    
    set_reg_value(cpu, index, result, memory);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(false);
    
    get_cycles(index)
}

// SRL - Shift Right Logical
fn srl_reg(cpu: &mut Cpu, index: u8, memory: &mut Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let carry = value & 0x01;
    let result = value >> 1;
    
    set_reg_value(cpu, index, result, memory);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry != 0);
    
    get_cycles(index)
}

// BIT - Test bit
fn bit_reg(cpu: &mut Cpu, bit: u8, index: u8, memory: &Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let bit_mask = 1 << bit;
    
    cpu.registers.set_flag_z((value & bit_mask) == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(true);
    
    get_bit_cycles(index)
}

// RES - Reset bit
fn res_reg(cpu: &mut Cpu, bit: u8, index: u8, memory: &mut Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let bit_mask = !(1 << bit);
    let result = value & bit_mask;
    
    set_reg_value(cpu, index, result, memory);
    
    get_cycles(index)
}

// SET - Set bit
fn set_reg(cpu: &mut Cpu, bit: u8, index: u8, memory: &mut Memory) -> u8 {
    let value = get_reg_value(cpu, index, memory);
    let bit_mask = 1 << bit;
    let result = value | bit_mask;
    
    set_reg_value(cpu, index, result, memory);
    
    get_cycles(index)
}