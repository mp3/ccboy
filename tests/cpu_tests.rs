use ccboy::*;

#[cfg(test)]
mod cpu_tests {
    use super::*;

    fn create_test_emulator() -> Emulator {
        let mut emu = Emulator::new();
        let rom = create_test_rom(&[]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01); // Disable boot ROM
        emu.step(); // Execute LD SP, $FFFE
        emu.step(); // Execute JP 0x100
        emu
    }

    fn create_test_rom(instructions: &[u8]) -> Vec<u8> {
        let mut rom = vec![0xFF; 0x8000];
        // Initialize code at 0x0000
        rom[0x0000] = 0x31; // LD SP, $FFFE
        rom[0x0001] = 0xFE;
        rom[0x0002] = 0xFF;
        rom[0x0003] = 0xC3; // JP 0x0100
        rom[0x0004] = 0x00;
        rom[0x0005] = 0x01;
        
        // Copy instructions to 0x100
        for (i, &byte) in instructions.iter().enumerate() {
            rom[0x100 + i] = byte;
        }
        
        // Add Nintendo logo at 0x104-0x133 (required for boot)
        let logo = [
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
            0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
            0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        if instructions.len() <= 4 {
            // Only add logo if instructions don't overlap
            rom[0x104..0x134].copy_from_slice(&logo);
        }
        rom
    }

    #[test]
    fn test_nop_instruction() {
        let mut emu = Emulator::new();
        let rom = create_test_rom(&[
            0x00, // NOP
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        emu.step(); // LD SP
        emu.step(); // JP 0x100
        
        let initial_state = emu.get_cpu_state();
        assert_eq!(initial_state.pc, 0x100);
        emu.step(); // Execute NOP
        let final_state = emu.get_cpu_state();
        
        assert_eq!(final_state.pc, 0x101);
    }

    #[test]
    fn test_ld_immediate() {
        let mut emu = Emulator::new();
        let rom = create_test_rom(&[
            0x3E, 0x42, // LD A, $42
            0x06, 0x13, // LD B, $13
            0x0E, 0x37, // LD C, $37
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        emu.step(); // LD SP
        emu.step(); // JP 0x100
        
        emu.step(); // LD A, $42
        assert_eq!(emu.get_cpu_state().a, 0x42);
        
        emu.step(); // LD B, $13
        assert_eq!(emu.get_cpu_state().b, 0x13);
        
        emu.step(); // LD C, $37
        assert_eq!(emu.get_cpu_state().c, 0x37);
    }

    #[test]
    fn test_ld_register_to_register() {
        let mut emu = Emulator::new();
        let rom = create_test_rom(&[
            0x3E, 0x42, // LD A, $42
            0x47,       // LD B, A
            0x48,       // LD C, B
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        emu.step(); // LD SP
        emu.step(); // JP 0x100
        
        emu.step(); // LD A, $42
        emu.step(); // LD B, A
        assert_eq!(emu.get_cpu_state().b, 0x42);
        
        emu.step(); // LD C, B
        assert_eq!(emu.get_cpu_state().c, 0x42);
    }

    #[test]
    fn test_inc_dec() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x3E, 0x00, // LD A, $00
            0x3C,       // INC A
            0x3C,       // INC A
            0x3D,       // DEC A
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD A, $00
        emu.step(); // INC A
        assert_eq!(emu.get_cpu_state().a, 0x01);
        
        emu.step(); // INC A
        assert_eq!(emu.get_cpu_state().a, 0x02);
        
        emu.step(); // DEC A
        assert_eq!(emu.get_cpu_state().a, 0x01);
    }

    #[test]
    fn test_add_instruction() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x3E, 0x10, // LD A, $10
            0x06, 0x20, // LD B, $20
            0x80,       // ADD A, B
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD A, $10
        emu.step(); // LD B, $20
        emu.step(); // ADD A, B
        
        let state = emu.get_cpu_state();
        assert_eq!(state.a, 0x30);
        assert!(!state.flag_z()); // Not zero
        assert!(!state.flag_c()); // No carry
    }

    #[test]
    fn test_add_with_carry() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x3E, 0xFF, // LD A, $FF
            0x06, 0x01, // LD B, $01
            0x80,       // ADD A, B
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD A, $FF
        emu.step(); // LD B, $01
        emu.step(); // ADD A, B
        
        let state = emu.get_cpu_state();
        assert_eq!(state.a, 0x00);
        assert!(state.flag_z());  // Zero
        assert!(state.flag_c());  // Carry
    }

    #[test]
    fn test_sub_instruction() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x3E, 0x30, // LD A, $30
            0x06, 0x10, // LD B, $10
            0x90,       // SUB B
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD A, $30
        emu.step(); // LD B, $10
        emu.step(); // SUB B
        
        let state = emu.get_cpu_state();
        assert_eq!(state.a, 0x20);
        assert!(!state.flag_z()); // Not zero
        assert!(state.flag_n());  // Subtract flag
        assert!(!state.flag_c()); // No carry
    }

    #[test]
    fn test_and_or_xor() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x3E, 0xFF, // LD A, $FF
            0x06, 0x0F, // LD B, $0F
            0xA0,       // AND B
            0x3E, 0xF0, // LD A, $F0
            0xB0,       // OR B
            0x3E, 0xFF, // LD A, $FF
            0xA8,       // XOR B
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD A, $FF
        emu.step(); // LD B, $0F
        emu.step(); // AND B
        assert_eq!(emu.get_cpu_state().a, 0x0F);
        
        emu.step(); // LD A, $F0
        emu.step(); // OR B
        assert_eq!(emu.get_cpu_state().a, 0xFF);
        
        emu.step(); // LD A, $FF
        emu.step(); // XOR B
        assert_eq!(emu.get_cpu_state().a, 0xF0);
    }

    #[test]
    fn test_cp_instruction() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x3E, 0x42, // LD A, $42
            0xFE, 0x42, // CP $42
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD A, $42
        emu.step(); // CP $42
        
        let state = emu.get_cpu_state();
        assert_eq!(state.a, 0x42); // A unchanged
        assert!(state.flag_z());   // Zero flag set
        assert!(state.flag_n());   // Subtract flag set
    }

    #[test]
    fn test_jp_instruction() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0xC3, 0x50, 0x01, // JP $0150
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // JP $0150
        assert_eq!(emu.get_cpu_state().pc, 0x0150);
    }

    #[test]
    fn test_jr_instruction() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x18, 0x05, // JR +5
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        let initial_pc = emu.get_cpu_state().pc;
        emu.step(); // JR +5
        assert_eq!(emu.get_cpu_state().pc, initial_pc + 2 + 5);
    }

    #[test]
    fn test_call_ret() {
        let mut emu = Emulator::new();
        let mut rom = vec![0xFF; 0x8000]; // Initialize with 0xFF instead of 0x00
        
        // Set up stack pointer first
        rom[0x0000] = 0x31; // LD SP, $FFFE
        rom[0x0001] = 0xFE;
        rom[0x0002] = 0xFF;
        rom[0x0003] = 0xC3; // JP 0x0100
        rom[0x0004] = 0x00;
        rom[0x0005] = 0x01;
        
        // Program at 0x100
        rom[0x100] = 0xCD; // CALL $0150
        rom[0x101] = 0x50;
        rom[0x102] = 0x01;
        rom[0x103] = 0x00; // NOP (return here)
        
        // Subroutine at 0x150 (after Nintendo logo)
        rom[0x150] = 0xC9; // RET
        
        // Add Nintendo logo
        let logo = [
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
            0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
            0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        rom[0x104..0x134].copy_from_slice(&logo);
        
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        // Verify ROM was loaded correctly
        println!("ROM check: [0x150]=0x{:02X} (should be 0xC9)", emu.read_memory(0x150));
        
        // Execute LD SP, $FFFE
        emu.step();
        // Execute jump to 0x100
        emu.step();
        
        let initial_pc = emu.get_cpu_state().pc;
        assert_eq!(initial_pc, 0x100);
        
        let initial_sp = emu.get_cpu_state().sp;
        println!("Initial SP: 0x{:04X}", initial_sp);
        emu.step(); // CALL $0110
        
        let state = emu.get_cpu_state();
        println!("After CALL: PC=0x{:04X}, SP=0x{:04X}", state.pc, state.sp);
        
        // Check what's on the stack
        let stack_lo = emu.read_memory(state.sp);
        let stack_hi = emu.read_memory(state.sp + 1);
        println!("Stack contents: [SP]=0x{:02X}, [SP+1]=0x{:02X}", stack_lo, stack_hi);
        
        assert_eq!(state.pc, 0x0150);
        assert_eq!(state.sp, initial_sp - 2); // Stack pointer decreased
        
        // Check what instruction we're about to execute
        let next_instr = emu.read_memory(state.pc);
        println!("Instruction at 0x{:04X}: 0x{:02X} (should be 0xC9 for RET)", state.pc, next_instr);
        
        emu.step(); // RET
        let state = emu.get_cpu_state();
        println!("After RET: PC=0x{:04X}, SP=0x{:04X}", state.pc, state.sp);
        assert_eq!(state.pc, 0x0103); // Return address
        assert_eq!(state.sp, initial_sp); // Stack pointer restored
    }

    #[test]
    fn test_push_pop() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x01, 0x34, 0x12, // LD BC, $1234
            0xC5,             // PUSH BC
            0x01, 0x00, 0x00, // LD BC, $0000
            0xC1,             // POP BC
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD BC, $1234
        assert_eq!(emu.get_cpu_state().bc(), 0x1234);
        
        let initial_sp = emu.get_cpu_state().sp;
        emu.step(); // PUSH BC
        assert_eq!(emu.get_cpu_state().sp, initial_sp - 2);
        
        emu.step(); // LD BC, $0000
        assert_eq!(emu.get_cpu_state().bc(), 0x0000);
        
        emu.step(); // POP BC
        let state = emu.get_cpu_state();
        assert_eq!(state.bc(), 0x1234);
        assert_eq!(state.sp, initial_sp);
    }

    #[test]
    fn test_16bit_inc_dec() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x21, 0xFF, 0xFF, // LD HL, $FFFF
            0x23,             // INC HL
            0x2B,             // DEC HL
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD HL, $FFFF
        assert_eq!(emu.get_cpu_state().hl(), 0xFFFF);
        
        emu.step(); // INC HL
        assert_eq!(emu.get_cpu_state().hl(), 0x0000); // Wrap around
        
        emu.step(); // DEC HL
        assert_eq!(emu.get_cpu_state().hl(), 0xFFFF);
    }

    #[test]
    fn test_memory_operations() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x3E, 0x42,       // LD A, $42
            0x21, 0x00, 0xC0, // LD HL, $C000
            0x77,             // LD (HL), A
            0x3E, 0x00,       // LD A, $00
            0x7E,             // LD A, (HL)
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD A, $42
        emu.step(); // LD HL, $C000
        emu.step(); // LD (HL), A
        
        // Check memory was written
        assert_eq!(emu.read_memory(0xC000), 0x42);
        
        emu.step(); // LD A, $00
        assert_eq!(emu.get_cpu_state().a, 0x00);
        
        emu.step(); // LD A, (HL)
        assert_eq!(emu.get_cpu_state().a, 0x42);
    }

    #[test]
    fn test_conditional_jumps() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x3E, 0x00, // LD A, $00
            0x3C,       // INC A (sets Z=0)
            0x20, 0x02, // JR NZ, +2
            0x00,       // NOP (should be skipped)
            0x00,       // NOP (should be skipped)
            0x3D,       // DEC A (sets Z=1)
            0x28, 0x02, // JR Z, +2
            0x00,       // NOP (should be skipped)
            0x00,       // NOP (should be skipped)
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD A, $00
        emu.step(); // INC A
        let pc_before = emu.get_cpu_state().pc;
        emu.step(); // JR NZ, +2
        assert_eq!(emu.get_cpu_state().pc, pc_before + 2 + 2); // Jump taken
        
        emu.step(); // DEC A
        let pc_before = emu.get_cpu_state().pc;
        emu.step(); // JR Z, +2
        assert_eq!(emu.get_cpu_state().pc, pc_before + 2 + 2); // Jump taken
    }

    #[test]
    fn test_halt_instruction() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x76, // HALT
            0x00, // NOP (should not execute)
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // HALT
        let pc_after_halt = emu.get_cpu_state().pc;
        
        emu.step(); // Should stay halted
        assert_eq!(emu.get_cpu_state().pc, pc_after_halt);
    }

    #[test]
    fn test_interrupts() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0xFB,       // EI (Enable interrupts)
            0x00,       // NOP
            0x00,       // NOP
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // EI
        emu.step(); // NOP (interrupts enabled after this)
        
        // Enable V-Blank interrupt
        emu.write_memory(0xFFFF, 0x01); // Interrupt enable
        emu.write_memory(0xFF0F, 0x01); // Request V-Blank interrupt
        
        let pc_before = emu.get_cpu_state().pc;
        emu.step();
        
        // Should jump to interrupt vector
        assert_eq!(emu.get_cpu_state().pc, 0x0040); // V-Blank vector
    }

    #[test]
    fn test_cb_bit_operations() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x3E, 0x80,   // LD A, $80
            0xCB, 0x7F,   // BIT 7, A
            0x3E, 0x00,   // LD A, $00
            0xCB, 0x7F,   // BIT 7, A
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD A, $80
        emu.step(); // BIT 7, A
        assert!(!emu.get_cpu_state().flag_z()); // Bit 7 is set
        
        emu.step(); // LD A, $00
        emu.step(); // BIT 7, A
        assert!(emu.get_cpu_state().flag_z()); // Bit 7 is not set
    }

    #[test]
    fn test_rotate_operations() {
        let mut emu = create_test_emulator();
        let rom = create_test_rom(&[
            0x3E, 0x80, // LD A, $80
            0x07,       // RLCA
            0x3E, 0x01, // LD A, $01
            0x0F,       // RRCA
        ]);
        emu.load_rom(&rom);
        emu.write_memory(0xFF50, 0x01);
        
        emu.step(); // LD A, $80
        emu.step(); // RLCA
        let state = emu.get_cpu_state();
        assert_eq!(state.a, 0x01); // Rotated left
        assert!(state.flag_c());   // Carry set
        
        emu.step(); // LD A, $01
        emu.step(); // RRCA
        let state = emu.get_cpu_state();
        assert_eq!(state.a, 0x80); // Rotated right
        assert!(state.flag_c());   // Carry set
    }
}