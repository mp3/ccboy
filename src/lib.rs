mod cpu;
mod gameboy;
mod joypad;
mod memory;
mod ppu;
mod timer;

use wasm_bindgen::prelude::*;

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
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

extern crate console_error_panic_hook;