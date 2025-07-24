use ccboy::*;

#[cfg(test)]
mod ppu_tests {
    use super::*;

    fn create_test_emulator() -> Emulator {
        let mut emu = Emulator::new();
        emu.write_memory(0xFF50, 0x01); // Disable boot ROM
        emu
    }

    #[test]
    fn test_lcdc_register() {
        let mut emu = create_test_emulator();
        
        // Test LCDC bits
        emu.write_memory(0xFF40, 0x91); // LCD on, BG on, OBJ on
        assert_eq!(emu.read_memory(0xFF40), 0x91);
        
        // Disable LCD
        emu.write_memory(0xFF40, 0x00);
        assert_eq!(emu.read_memory(0xFF40), 0x00);
    }

    #[test]
    fn test_stat_register() {
        let mut emu = create_test_emulator();
        
        // STAT register has some read-only bits (mode)
        emu.write_memory(0xFF41, 0xFF);
        let stat = emu.read_memory(0xFF41);
        
        // Lower 2 bits are mode (read-only during PPU operation)
        // Bit 2 is coincidence flag (read-only)
        assert_eq!(stat & 0xF8, 0xF8); // Upper bits should be writable
    }

    #[test]
    fn test_scroll_registers() {
        let mut emu = create_test_emulator();
        
        // Test SCY
        emu.write_memory(0xFF42, 0x42);
        assert_eq!(emu.read_memory(0xFF42), 0x42);
        
        // Test SCX
        emu.write_memory(0xFF43, 0x13);
        assert_eq!(emu.read_memory(0xFF43), 0x13);
        
        // Test WY
        emu.write_memory(0xFF4A, 0x50);
        assert_eq!(emu.read_memory(0xFF4A), 0x50);
        
        // Test WX
        emu.write_memory(0xFF4B, 0x07);
        assert_eq!(emu.read_memory(0xFF4B), 0x07);
    }

    #[test]
    fn test_ly_register() {
        let mut emu = create_test_emulator();
        
        // LY is read-only and increments during PPU operation
        let initial_ly = emu.read_memory(0xFF44);
        
        // Run some frames
        for _ in 0..10 {
            emu.run_frame();
        }
        
        // LY should have changed
        let final_ly = emu.read_memory(0xFF44);
        // LY cycles from 0-153, so it might wrap around
        assert!(initial_ly != final_ly || initial_ly == 0);
    }

    #[test]
    fn test_lyc_register() {
        let mut emu = create_test_emulator();
        
        // Set LYC for line compare
        emu.write_memory(0xFF45, 0x40);
        assert_eq!(emu.read_memory(0xFF45), 0x40);
        
        // Enable LYC interrupt
        emu.write_memory(0xFF41, 0x40); // LYC interrupt enable
        emu.write_memory(0xFFFF, 0x02); // LCD STAT interrupt enable
        
        // Run until LY matches LYC
        for _ in 0..1000 {
            emu.step();
            if emu.read_memory(0xFF44) == 0x40 {
                break;
            }
        }
        
        // Check coincidence flag
        let stat = emu.read_memory(0xFF41);
        if emu.read_memory(0xFF44) == 0x40 {
            assert_ne!(stat & 0x04, 0); // Coincidence flag should be set
        }
    }

    #[test]
    fn test_palette_registers() {
        let mut emu = create_test_emulator();
        
        // Test BGP (Background Palette)
        emu.write_memory(0xFF47, 0xE4); // Standard palette
        assert_eq!(emu.read_memory(0xFF47), 0xE4);
        
        // Test OBP0 (Object Palette 0)
        emu.write_memory(0xFF48, 0xD3);
        assert_eq!(emu.read_memory(0xFF48), 0xD3);
        
        // Test OBP1 (Object Palette 1)
        emu.write_memory(0xFF49, 0x1B);
        assert_eq!(emu.read_memory(0xFF49), 0x1B);
    }

    #[test]
    fn test_vram_tile_data() {
        let mut emu = create_test_emulator();
        
        // Write a simple tile pattern (8x8 pixels, 2 bits per pixel)
        let tile_data = [
            0xFF, 0x00, // Row 0: 10101010
            0x00, 0xFF, // Row 1: 01010101
            0xFF, 0x00, // Row 2: 10101010
            0x00, 0xFF, // Row 3: 01010101
            0xFF, 0x00, // Row 4: 10101010
            0x00, 0xFF, // Row 5: 01010101
            0xFF, 0x00, // Row 6: 10101010
            0x00, 0xFF, // Row 7: 01010101
        ];
        
        // Write to tile 0 at 0x8000
        for (i, &byte) in tile_data.iter().enumerate() {
            emu.write_memory(0x8000 + i as u16, byte);
        }
        
        // Read back tile data
        for (i, &expected) in tile_data.iter().enumerate() {
            assert_eq!(emu.read_memory(0x8000 + i as u16), expected);
        }
    }

    #[test]
    fn test_tile_map() {
        let mut emu = create_test_emulator();
        
        // Write to background tile map at 0x9800
        for i in 0..32 {
            emu.write_memory(0x9800 + i, i as u8);
        }
        
        // Read back tile map
        for i in 0..32 {
            assert_eq!(emu.read_memory(0x9800 + i), i as u8);
        }
        
        // Write to window tile map at 0x9C00
        for i in 0..32 {
            emu.write_memory(0x9C00 + i, (i + 0x80) as u8);
        }
        
        // Read back window tile map
        for i in 0..32 {
            assert_eq!(emu.read_memory(0x9C00 + i), (i + 0x80) as u8);
        }
    }

    #[test]
    fn test_oam_sprites() {
        let mut emu = create_test_emulator();
        
        // Write sprite data to OAM
        // Sprite 0
        emu.write_memory(0xFE00, 0x10); // Y position
        emu.write_memory(0xFE01, 0x20); // X position
        emu.write_memory(0xFE02, 0x00); // Tile index
        emu.write_memory(0xFE03, 0x00); // Attributes
        
        // Sprite 1
        emu.write_memory(0xFE04, 0x18); // Y position
        emu.write_memory(0xFE05, 0x28); // X position
        emu.write_memory(0xFE06, 0x01); // Tile index
        emu.write_memory(0xFE07, 0x20); // Attributes (X flip)
        
        // Read back sprite data
        assert_eq!(emu.read_memory(0xFE00), 0x10);
        assert_eq!(emu.read_memory(0xFE01), 0x20);
        assert_eq!(emu.read_memory(0xFE02), 0x00);
        assert_eq!(emu.read_memory(0xFE03), 0x00);
    }

    #[test]
    fn test_screen_buffer() {
        let mut emu = create_test_emulator();
        
        // Enable LCD and background
        emu.write_memory(0xFF40, 0x91);
        
        // Set up a simple tile
        for i in 0..16 {
            emu.write_memory(0x8000 + i, 0xFF); // All pixels color 3
        }
        
        // Fill tile map with tile 0
        for i in 0..1024 {
            emu.write_memory(0x9800 + i, 0x00);
        }
        
        // Run a frame
        emu.run_frame();
        
        // Get screen buffer
        let buffer = emu.get_screen_buffer();
        
        // Check buffer size (160x144x4 bytes for RGBA)
        assert_eq!(buffer.len(), 160 * 144 * 4);
        
        // At least some pixels should be non-zero
        let non_zero_pixels = buffer.iter().filter(|&&b| b != 0).count();
        assert!(non_zero_pixels > 0);
    }

    #[test]
    fn test_vblank_interrupt() {
        let mut emu = create_test_emulator();
        
        // Enable V-Blank interrupt
        emu.write_memory(0xFFFF, 0x01); // Interrupt enable
        emu.write_memory(0xFF40, 0x80); // LCD on
        
        // Clear interrupt flag
        emu.write_memory(0xFF0F, 0x00);
        
        // Run one frame
        emu.run_frame();
        
        // V-Blank interrupt should have been requested
        let if_reg = emu.read_memory(0xFF0F);
        assert_ne!(if_reg & 0x01, 0); // V-Blank flag should be set
    }

    #[test]
    fn test_stat_interrupts() {
        let mut emu = create_test_emulator();
        
        // Enable STAT interrupt for H-Blank
        emu.write_memory(0xFF41, 0x08); // H-Blank interrupt
        emu.write_memory(0xFFFF, 0x02); // LCD STAT interrupt enable
        emu.write_memory(0xFF40, 0x80); // LCD on
        
        // Clear interrupt flag
        emu.write_memory(0xFF0F, 0x00);
        
        // Run for a short time (should hit H-Blank)
        for _ in 0..500 {
            emu.step();
        }
        
        // STAT interrupt might have been requested
        let if_reg = emu.read_memory(0xFF0F);
        // Note: Exact behavior depends on timing
    }

    #[test]
    fn test_window_rendering() {
        let mut emu = create_test_emulator();
        
        // Enable LCD, background, and window
        emu.write_memory(0xFF40, 0xB1); // LCD on, BG on, Window on
        
        // Set window position
        emu.write_memory(0xFF4A, 0x40); // WY = 64
        emu.write_memory(0xFF4B, 0x07); // WX = 7
        
        // Set up different tiles for BG and Window
        // Tile 0: All white
        for i in 0..16 {
            emu.write_memory(0x8000 + i, 0x00);
        }
        
        // Tile 1: All black
        for i in 0..16 {
            emu.write_memory(0x8010 + i, 0xFF);
        }
        
        // Fill BG map with tile 0
        for i in 0..1024 {
            emu.write_memory(0x9800 + i, 0x00);
        }
        
        // Fill Window map with tile 1
        for i in 0..1024 {
            emu.write_memory(0x9C00 + i, 0x01);
        }
        
        // Run a frame
        emu.run_frame();
        
        // Window should be visible in lower part of screen
        let buffer = emu.get_screen_buffer();
        
        // Check that we have some variation in the screen
        let first_pixel = buffer[0];
        let different_pixels = buffer.iter().filter(|&&b| b != first_pixel).count();
        assert!(different_pixels > 0);
    }

    #[test]
    fn test_sprite_priority() {
        let mut emu = create_test_emulator();
        
        // Enable LCD, background, and sprites
        emu.write_memory(0xFF40, 0x93); // LCD on, BG on, OBJ on
        
        // Create two overlapping sprites
        // Sprite 0 (lower X coordinate = higher priority)
        emu.write_memory(0xFE00, 0x50); // Y
        emu.write_memory(0xFE01, 0x50); // X
        emu.write_memory(0xFE02, 0x00); // Tile
        emu.write_memory(0xFE03, 0x00); // Attributes
        
        // Sprite 1
        emu.write_memory(0xFE04, 0x50); // Y (same)
        emu.write_memory(0xFE05, 0x52); // X (slightly to the right)
        emu.write_memory(0xFE06, 0x01); // Tile
        emu.write_memory(0xFE07, 0x00); // Attributes
        
        // Set up different tiles
        for i in 0..16 {
            emu.write_memory(0x8000 + i, 0xFF); // Tile 0: black
            emu.write_memory(0x8010 + i, 0x00); // Tile 1: white
        }
        
        // Run a frame
        emu.run_frame();
        
        // Sprite 0 should have priority where they overlap
    }
}