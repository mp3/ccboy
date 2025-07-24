use crate::cpu::Cpu;
use crate::joypad::Joypad;
use crate::memory::Memory;
use crate::ppu::Ppu;
use crate::timer::Timer;
use crate::debug::CpuState;

const CYCLES_PER_FRAME: u32 = 70224;

pub struct GameBoy {
    cpu: Cpu,
    memory: Memory,
    ppu: Ppu,
    timer: Timer,
    joypad: Joypad,
    cycles: u32,
}

impl GameBoy {
    pub fn new() -> Self {
        let memory = Memory::new();
        let cpu = Cpu::new();
        let ppu = Ppu::new();
        let timer = Timer::new();
        let joypad = Joypad::new();

        Self {
            cpu,
            memory,
            ppu,
            timer,
            joypad,
            cycles: 0,
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.memory.load_rom(rom_data);
    }

    pub fn step(&mut self) {
        let cycles = self.cpu.step(&mut self.memory);
        self.cycles += cycles as u32;
        
        self.timer.update(cycles, &mut self.memory);
        self.ppu.update(cycles, &mut self.memory);
        
        self.handle_interrupts();
    }

    pub fn run_frame(&mut self) {
        let target_cycles = self.cycles + CYCLES_PER_FRAME;
        
        while self.cycles < target_cycles {
            self.step();
        }
        
        self.cycles -= CYCLES_PER_FRAME;
    }

    pub fn get_screen_buffer(&self) -> Vec<u8> {
        self.ppu.get_screen_buffer()
    }

    pub fn key_down(&mut self, key: u8) {
        self.joypad.key_down(key);
        self.memory.update_joypad(self.joypad.get_state());
    }

    pub fn key_up(&mut self, key: u8) {
        self.joypad.key_up(key);
        self.memory.update_joypad(self.joypad.get_state());
    }

    fn handle_interrupts(&mut self) {
        let interrupts = self.memory.get_triggered_interrupts();
        if interrupts != 0 {
            self.cpu.handle_interrupts(interrupts, &mut self.memory);
        }
    }
    
    pub fn get_cpu_state(&self) -> CpuState {
        CpuState::from_cpu(&self.cpu)
    }
    
    pub fn read_memory(&self, address: u16) -> u8 {
        self.memory.read_byte(address)
    }
    
    pub fn write_memory(&mut self, address: u16, value: u8) {
        self.memory.write_byte(address, value);
    }
}