#[cfg(test)]
mod save_state_tests {
    use ccboy::Emulator;

    // Note: Save state test using JsValue can only be run in WASM environment
    // We'll test the save data functionality which doesn't depend on JsValue
    
    #[test]
    fn test_save_data() {
        let mut emulator = Emulator::new();
        
        // Load a test ROM with RAM
        let mut test_rom = vec![0x00; 0x8000];
        test_rom[0x147] = 0x03; // MBC1 with RAM and battery
        test_rom[0x149] = 0x02; // 8KB RAM
        emulator.load_rom(&test_rom);
        
        // Enable cartridge RAM first
        emulator.write_memory(0x0000, 0x0A); // Enable RAM
        
        // Write some data to cartridge RAM
        emulator.write_memory(0xA000, 0x42);
        emulator.write_memory(0xA001, 0x43);
        
        // Get save data
        let save_data = emulator.get_save_data();
        assert!(!save_data.is_empty());
        
        // Create new emulator and load save data
        let mut emulator2 = Emulator::new();
        emulator2.load_rom(&test_rom);
        emulator2.load_save_data(&save_data);
        
        // Verify data was loaded
        assert_eq!(emulator2.read_memory(0xA000), 0x42);
        assert_eq!(emulator2.read_memory(0xA001), 0x43);
    }
}