use ccboy::*;

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    fn init_emulator_at_0x100(emu: &mut Emulator) {
        // Skip boot ROM and jump to 0x100
        emu.write_memory(0xFF50, 0x01);
        emu.step(); // LD SP
        emu.step(); // JP 0x100
    }
    
    fn create_rom_with_program(program: &[u8]) -> Vec<u8> {
        let mut rom = vec![0xFF; 0x8000];
        
        // Initialize code
        rom[0x0000] = 0x31; // LD SP, $FFFE
        rom[0x0001] = 0xFE;
        rom[0x0002] = 0xFF;
        rom[0x0003] = 0xC3; // JP 0x0100
        rom[0x0004] = 0x00;
        rom[0x0005] = 0x01;
        
        // If program is short, put it at 0x100
        // If it would overlap with Nintendo logo (0x104), put it at 0x150
        if program.len() <= 4 {
            rom[0x100..0x100 + program.len()].copy_from_slice(program);
        } else {
            // Jump to program at 0x150
            rom[0x100] = 0xC3; // JP 0x0150
            rom[0x101] = 0x50;
            rom[0x102] = 0x01;
            rom[0x150..0x150 + program.len()].copy_from_slice(program);
        }
        
        // Add Nintendo logo
        let logo = [
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
            0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
            0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        rom[0x104..0x134].copy_from_slice(&logo);
        
        rom
    }

    fn create_simple_rom() -> Vec<u8> {
        let mut rom = vec![0xFF; 0x8000];
        
        // Initialize code
        rom[0x0000] = 0x31; // LD SP, $FFFE
        rom[0x0001] = 0xFE;
        rom[0x0002] = 0xFF;
        rom[0x0003] = 0xC3; // JP 0x0100
        rom[0x0004] = 0x00;
        rom[0x0005] = 0x01;
        
        // Simple program that increments a counter in memory
        let program = [
            0x3E, 0x00,       // LD A, $00
            0x21, 0x00, 0xC0, // LD HL, $C000
            0x77,             // LD (HL), A    ; Store counter
            // Loop:
            0x7E,             // LD A, (HL)    ; Load counter
            0x3C,             // INC A         ; Increment
            0x77,             // LD (HL), A    ; Store back
            0xFE, 0x10,       // CP $10        ; Compare with 16
            0x20, 0xF8,       // JR NZ, -8     ; Loop if not equal
            0x76,             // HALT
        ];
        
        // Copy program to ROM starting at 0x150 (after Nintendo logo)
        rom[0x150..0x150 + program.len()].copy_from_slice(&program);
        
        // Jump to our program
        rom[0x100] = 0xC3; // JP 0x0150
        rom[0x101] = 0x50;
        rom[0x102] = 0x01;
        
        // Add Nintendo logo
        let logo = [
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
            0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
            0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        rom[0x104..0x134].copy_from_slice(&logo);
        
        // Set header checksum
        let mut checksum: u8 = 0;
        for i in 0x134..=0x14C {
            checksum = checksum.wrapping_sub(rom[i]).wrapping_sub(1);
        }
        rom[0x14D] = checksum;
        
        rom
    }

    #[test]
    fn test_simple_program_execution() {
        let mut emu = Emulator::new();
        let rom = create_simple_rom();
        emu.load_rom(&rom);
        
        init_emulator_at_0x100(&mut emu);
        
        // Run for a limited number of steps to avoid infinite loop
        for _ in 0..1000 {
            let state = emu.get_cpu_state();
            if state.halt {
                break;
            }
            emu.step();
        }
        
        // Check that counter reached 16
        assert_eq!(emu.read_memory(0xC000), 0x10);
        assert!(emu.get_cpu_state().halt);
    }

    #[test]
    fn test_interrupt_handling() {
        let mut emu = Emulator::new();
        
        // Program that enables interrupts and waits
        let program = [
            0x3E, 0x01,       // LD A, $01
            0xE0, 0xFF,       // LDH ($FF), A  ; Enable V-Blank interrupt
            0xFB,             // EI
            0x00,             // NOP
            // Loop:
            0x18, 0xFE,       // JR -2         ; Infinite loop
        ];
        
        let mut rom = create_rom_with_program(&program);
        
        // V-Blank interrupt handler at 0x40
        rom[0x40] = 0x3E;  // LD A, $42
        rom[0x41] = 0x42;
        rom[0x42] = 0xEA;  // LD ($C000), A
        rom[0x43] = 0x00;
        rom[0x44] = 0xC0;
        rom[0x45] = 0xC9;  // RETI
        
        emu.load_rom(&rom);
        init_emulator_at_0x100(&mut emu);
        
        // Run until interrupts are enabled
        for _ in 0..10 {
            emu.step();
        }
        
        // Trigger V-Blank interrupt
        emu.write_memory(0xFF0F, 0x01);
        
        // Run a few more steps
        for _ in 0..10 {
            emu.step();
        }
        
        // Check that interrupt handler ran
        assert_eq!(emu.read_memory(0xC000), 0x42);
    }

    #[test]
    fn test_timer_operation() {
        let mut emu = Emulator::new();
        
        // Program that sets up timer
        let program = [
            0x3E, 0x00,       // LD A, $00
            0xE0, 0x05,       // LDH ($05), A  ; TIMA = 0
            0x3E, 0x10,       // LD A, $10
            0xE0, 0x06,       // LDH ($06), A  ; TMA = $10
            0x3E, 0x05,       // LD A, $05     ; Start timer, clock/16
            0xE0, 0x07,       // LDH ($07), A  ; TAC
            // Wait loop
            0x00,             // NOP
            0x18, 0xFD,       // JR -3
        ];
        
        let rom = create_rom_with_program(&program);
        emu.load_rom(&rom);
        init_emulator_at_0x100(&mut emu);
        
        // Set up timer interrupt
        emu.write_memory(0xFFFF, 0x04); // Enable timer interrupt
        
        // Run program to set up timer
        for _ in 0..20 {
            emu.step();
        }
        
        let initial_tima = emu.read_memory(0xFF05);
        
        // Run many cycles to let timer tick
        for _ in 0..1000 {
            emu.step();
        }
        
        let final_tima = emu.read_memory(0xFF05);
        
        // Timer should have changed
        assert_ne!(initial_tima, final_tima);
    }

    #[test]
    fn test_stack_operations() {
        let mut emu = Emulator::new();
        
        // Program that uses stack extensively
        let program = [
            0x31, 0xFE, 0xFF, // LD SP, $FFFE
            0x01, 0x34, 0x12, // LD BC, $1234
            0x11, 0x78, 0x56, // LD DE, $5678
            0xC5,             // PUSH BC
            0xD5,             // PUSH DE
            0x01, 0x00, 0x00, // LD BC, $0000
            0x11, 0x00, 0x00, // LD DE, $0000
            0xD1,             // POP DE
            0xC1,             // POP BC
            0x76,             // HALT
        ];
        
        let rom = create_rom_with_program(&program);
        emu.load_rom(&rom);
        init_emulator_at_0x100(&mut emu);
        
        // Run until halt
        while !emu.get_cpu_state().halt {
            emu.step();
        }
        
        let state = emu.get_cpu_state();
        assert_eq!(state.bc(), 0x1234); // BC restored
        assert_eq!(state.de(), 0x5678); // DE restored
        assert_eq!(state.sp, 0xFFFE); // Stack pointer restored
    }

    #[test]
    fn test_conditional_execution() {
        let mut emu = Emulator::new();
        
        // Program that tests various conditions
        let program = [
            0x3E, 0x00,       // LD A, $00
            0xB7,             // OR A          ; Set Z flag
            0x28, 0x02,       // JR Z, +2      ; Should jump
            0x3C,             // INC A         ; Should be skipped
            0x3C,             // INC A         ; Should be skipped
            0x3E, 0xFF,       // LD A, $FF
            0x3C,             // INC A         ; A = 0, C = 1, Z = 1
            0x38, 0x02,       // JR C, +2      ; Should jump
            0x3C,             // INC A         ; Should be skipped
            0x3C,             // INC A         ; Should be skipped
            0xEA, 0x00, 0xC0, // LD ($C000), A ; Store result
            0x76,             // HALT
        ];
        
        let rom = create_rom_with_program(&program);
        emu.load_rom(&rom);
        init_emulator_at_0x100(&mut emu);
        
        // Run until halt
        while !emu.get_cpu_state().halt {
            emu.step();
        }
        
        // A should still be 0 (both jumps taken)
        assert_eq!(emu.read_memory(0xC000), 0x00);
    }

    #[test]
    fn test_arithmetic_operations() {
        let mut emu = Emulator::new();
        
        // Program that tests arithmetic
        let program = [
            // Test ADD
            0x3E, 0x35,       // LD A, $35
            0x06, 0x27,       // LD B, $27
            0x80,             // ADD A, B      ; A = $5C
            0xEA, 0x00, 0xC0, // LD ($C000), A
            
            // Test SUB
            0x3E, 0x5C,       // LD A, $5C
            0x06, 0x27,       // LD B, $27
            0x90,             // SUB B         ; A = $35
            0xEA, 0x01, 0xC0, // LD ($C001), A
            
            // Test half-carry
            0x3E, 0x0F,       // LD A, $0F
            0xC6, 0x01,       // ADD A, $01    ; A = $10, H = 1
            0xEA, 0x02, 0xC0, // LD ($C002), A
            
            0x76,             // HALT
        ];
        
        let rom = create_rom_with_program(&program);
        emu.load_rom(&rom);
        init_emulator_at_0x100(&mut emu);
        
        // Run until halt
        while !emu.get_cpu_state().halt {
            emu.step();
        }
        
        assert_eq!(emu.read_memory(0xC000), 0x5C); // ADD result
        assert_eq!(emu.read_memory(0xC001), 0x35); // SUB result
        assert_eq!(emu.read_memory(0xC002), 0x10); // Half-carry test
    }

    #[test]
    fn test_bit_operations() {
        let mut emu = Emulator::new();
        
        // Program that tests bit operations
        let program = [
            // Test AND
            0x3E, 0xFF,       // LD A, $FF
            0xE6, 0x0F,       // AND $0F       ; A = $0F
            0xEA, 0x00, 0xC0, // LD ($C000), A
            
            // Test OR
            0x3E, 0xF0,       // LD A, $F0
            0xF6, 0x0F,       // OR $0F        ; A = $FF
            0xEA, 0x01, 0xC0, // LD ($C001), A
            
            // Test XOR
            0x3E, 0xFF,       // LD A, $FF
            0xEE, 0x0F,       // XOR $0F       ; A = $F0
            0xEA, 0x02, 0xC0, // LD ($C002), A
            
            // Test bit manipulation
            0x3E, 0x00,       // LD A, $00
            0xCB, 0xC7,       // SET 0, A      ; A = $01
            0xCB, 0xCF,       // SET 1, A      ; A = $03
            0xCB, 0x87,       // RES 0, A      ; A = $02
            0xEA, 0x03, 0xC0, // LD ($C003), A
            
            0x76,             // HALT
        ];
        
        let rom = create_rom_with_program(&program);
        emu.load_rom(&rom);
        init_emulator_at_0x100(&mut emu);
        
        // Run until halt
        while !emu.get_cpu_state().halt {
            emu.step();
        }
        
        assert_eq!(emu.read_memory(0xC000), 0x0F); // AND result
        assert_eq!(emu.read_memory(0xC001), 0xFF); // OR result
        assert_eq!(emu.read_memory(0xC002), 0xF0); // XOR result
        assert_eq!(emu.read_memory(0xC003), 0x02); // SET/RES result
    }

    #[test]
    fn test_16bit_arithmetic() {
        let mut emu = Emulator::new();
        
        // Program that tests 16-bit operations
        let program = [
            0x21, 0xFF, 0xFF, // LD HL, $FFFF
            0x01, 0x01, 0x00, // LD BC, $0001
            0x09,             // ADD HL, BC    ; HL = $0000, C = 1
            0x7C,             // LD A, H
            0xEA, 0x00, 0xC0, // LD ($C000), A ; Should be $00
            0x7D,             // LD A, L
            0xEA, 0x01, 0xC0, // LD ($C001), A ; Should be $00
            
            0x21, 0x00, 0x10, // LD HL, $1000
            0x23,             // INC HL        ; HL = $1001
            0x2B,             // DEC HL        ; HL = $1000
            0x7C,             // LD A, H
            0xEA, 0x02, 0xC0, // LD ($C002), A ; Should be $10
            
            0x76,             // HALT
        ];
        
        let rom = create_rom_with_program(&program);
        emu.load_rom(&rom);
        init_emulator_at_0x100(&mut emu);
        
        // Run until halt
        while !emu.get_cpu_state().halt {
            emu.step();
        }
        
        assert_eq!(emu.read_memory(0xC000), 0x00); // H after overflow
        assert_eq!(emu.read_memory(0xC001), 0x00); // L after overflow
        assert_eq!(emu.read_memory(0xC002), 0x10); // H after INC/DEC
    }

    #[test]
    fn test_indirect_memory_access() {
        let mut emu = Emulator::new();
        
        // Program that tests indirect addressing
        let program = [
            // Test (HL) addressing
            0x21, 0x00, 0xC0, // LD HL, $C000
            0x3E, 0x42,       // LD A, $42
            0x77,             // LD (HL), A
            0x23,             // INC HL
            0x3E, 0x13,       // LD A, $13
            0x77,             // LD (HL), A
            
            // Test (BC) and (DE) addressing
            0x01, 0x02, 0xC0, // LD BC, $C002
            0x3E, 0x37,       // LD A, $37
            0x02,             // LD (BC), A
            
            0x11, 0x03, 0xC0, // LD DE, $C003
            0x3E, 0x73,       // LD A, $73
            0x12,             // LD (DE), A
            
            // Test reading
            0x21, 0x00, 0xC0, // LD HL, $C000
            0x7E,             // LD A, (HL)    ; A = $42
            0x23,             // INC HL
            0x46,             // LD B, (HL)    ; B = $13
            0x0A,             // LD A, (BC)    ; A = $37
            0x1A,             // LD A, (DE)    ; A = $73
            
            0x76,             // HALT
        ];
        
        let rom = create_rom_with_program(&program);
        emu.load_rom(&rom);
        init_emulator_at_0x100(&mut emu);
        
        // Run until halt
        while !emu.get_cpu_state().halt {
            emu.step();
        }
        
        assert_eq!(emu.read_memory(0xC000), 0x42);
        assert_eq!(emu.read_memory(0xC001), 0x13);
        assert_eq!(emu.read_memory(0xC002), 0x37);
        assert_eq!(emu.read_memory(0xC003), 0x73);
        assert_eq!(emu.get_cpu_state().a, 0x73);
    }

    #[test]
    fn test_boot_rom_execution() {
        let mut emu = Emulator::new();
        
        // Don't skip boot ROM
        // The boot ROM should initialize the system
        
        // Run boot ROM (it's quite long)
        for _ in 0..10000 {
            let pc = emu.get_cpu_state().pc;
            emu.step();
            
            // Boot ROM ends by jumping to 0x100
            if pc == 0x100 {
                break;
            }
        }
        
        // Check that boot ROM completed
        let state = emu.get_cpu_state();
        assert_eq!(state.pc, 0x100); // Should be at cartridge entry point
        
        // Boot ROM should have disabled itself
        assert_eq!(emu.read_memory(0xFF50), 0x01);
    }
}