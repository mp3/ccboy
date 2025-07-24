mod cpu;
mod gameboy;
mod joypad;
mod memory;
mod ppu;
mod timer;
mod boot_rom;
mod debug;

use wasm_bindgen::prelude::*;
pub use debug::CpuState;

#[wasm_bindgen]
pub struct Emulator {
    gameboy: gameboy::GameBoy,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        
        Self {
            gameboy: gameboy::GameBoy::new(),
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.gameboy.load_rom(rom_data);
    }

    pub fn step(&mut self) {
        self.gameboy.step();
    }

    pub fn run_frame(&mut self) {
        self.gameboy.run_frame();
    }

    pub fn get_screen_buffer(&self) -> Vec<u8> {
        self.gameboy.get_screen_buffer()
    }

    pub fn key_down(&mut self, key: u8) {
        self.gameboy.key_down(key);
    }

    pub fn key_up(&mut self, key: u8) {
        self.gameboy.key_up(key);
    }
    
    pub fn get_cpu_state(&self) -> CpuState {
        self.gameboy.get_cpu_state()
    }
    
    pub fn read_memory(&self, address: u16) -> u8 {
        self.gameboy.read_memory(address)
    }
    
    pub fn write_memory(&mut self, address: u16, value: u8) {
        self.gameboy.write_memory(address, value);
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

extern crate console_error_panic_hook;