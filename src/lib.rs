mod cpu;
mod gameboy;
mod joypad;
mod memory;
mod ppu;
mod timer;
mod boot_rom;
mod debug;
mod apu;
mod save_state;

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
    
    pub fn get_audio_buffer(&mut self) -> Vec<f32> {
        self.gameboy.get_audio_buffer()
    }
    
    pub fn get_save_state(&self) -> JsValue {
        match self.gameboy.get_save_state() {
            Ok(state_json) => JsValue::from_str(&state_json),
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to create save state: {}", e).into());
                JsValue::NULL
            }
        }
    }
    
    pub fn load_save_state(&mut self, state: &str) -> bool {
        match self.gameboy.load_save_state(state) {
            Ok(_) => true,
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to load save state: {}", e).into());
                false
            }
        }
    }
    
    pub fn get_save_data(&self) -> Vec<u8> {
        self.gameboy.get_save_data()
    }
    
    pub fn load_save_data(&mut self, data: &[u8]) {
        self.gameboy.load_save_data(data);
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

extern crate console_error_panic_hook;