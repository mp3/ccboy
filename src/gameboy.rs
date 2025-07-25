use crate::cpu::Cpu;
use crate::joypad::Joypad;
use crate::memory::Memory;
use crate::ppu::Ppu;
use crate::timer::Timer;
use crate::debug::CpuState;
use crate::apu::Apu;
use crate::save_state::{SaveState, CpuSaveState, MemorySaveState};

const CYCLES_PER_FRAME: u32 = 70224;

pub struct GameBoy {
    cpu: Cpu,
    memory: Memory,
    ppu: Ppu,
    timer: Timer,
    joypad: Joypad,
    apu: Apu,
    cycles: u32,
}

impl GameBoy {
    pub fn new() -> Self {
        let memory = Memory::new();
        let cpu = Cpu::new();
        let ppu = Ppu::new();
        let timer = Timer::new();
        let joypad = Joypad::new();
        let apu = Apu::new();

        Self {
            cpu,
            memory,
            ppu,
            timer,
            joypad,
            apu,
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
        self.apu.update(cycles, &mut self.memory);
        
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
    
    pub fn get_audio_buffer(&mut self) -> Vec<f32> {
        self.apu.get_audio_buffer()
    }
    
    pub fn get_save_state(&self) -> Result<String, String> {
        let save_state = SaveState {
            cpu: CpuSaveState {
                a: self.cpu.registers.a,
                f: self.cpu.registers.f,
                b: self.cpu.registers.b,
                c: self.cpu.registers.c,
                d: self.cpu.registers.d,
                e: self.cpu.registers.e,
                h: self.cpu.registers.h,
                l: self.cpu.registers.l,
                sp: self.cpu.registers.sp,
                pc: self.cpu.registers.pc,
                ime: self.cpu.ime,
                halt: self.cpu.halt,
                cycles: self.cpu.cycles,
            },
            memory: self.create_memory_save_state(),
            cycles: self.cycles,
        };
        
        serde_json::to_string(&save_state).map_err(|e| e.to_string())
    }
    
    pub fn load_save_state(&mut self, state_json: &str) -> Result<(), String> {
        let save_state: SaveState = serde_json::from_str(state_json)
            .map_err(|e| e.to_string())?;
        
        // Restore CPU state
        self.cpu.registers.a = save_state.cpu.a;
        self.cpu.registers.f = save_state.cpu.f;
        self.cpu.registers.b = save_state.cpu.b;
        self.cpu.registers.c = save_state.cpu.c;
        self.cpu.registers.d = save_state.cpu.d;
        self.cpu.registers.e = save_state.cpu.e;
        self.cpu.registers.h = save_state.cpu.h;
        self.cpu.registers.l = save_state.cpu.l;
        self.cpu.registers.sp = save_state.cpu.sp;
        self.cpu.registers.pc = save_state.cpu.pc;
        self.cpu.ime = save_state.cpu.ime;
        self.cpu.halt = save_state.cpu.halt;
        self.cpu.cycles = save_state.cpu.cycles;
        
        // Restore memory state
        self.restore_memory_save_state(&save_state.memory);
        
        // Restore cycles
        self.cycles = save_state.cycles;
        
        Ok(())
    }
    
    pub fn get_save_data(&self) -> Vec<u8> {
        self.memory.get_cartridge_ram()
    }
    
    pub fn load_save_data(&mut self, data: &[u8]) {
        self.memory.load_cartridge_ram(data);
    }
    
    fn create_memory_save_state(&self) -> MemorySaveState {
        MemorySaveState {
            vram: self.memory.get_vram().to_vec(),
            wram: self.memory.get_wram().to_vec(),
            oam: self.memory.get_oam().to_vec(),
            io: self.memory.get_io().to_vec(),
            hram: self.memory.get_hram().to_vec(),
            interrupt_enable: self.memory.read_byte(0xFFFF),
            interrupt_flag: self.memory.read_byte(0xFF0F),
            boot_rom_enabled: self.memory.is_boot_rom_enabled(),
            cartridge_ram: self.memory.get_cartridge_ram_vec(),
            mbc_state: self.memory.get_mbc_state(),
        }
    }
    
    fn restore_memory_save_state(&mut self, state: &MemorySaveState) {
        self.memory.set_vram(&state.vram);
        self.memory.set_wram(&state.wram);
        self.memory.set_oam(&state.oam);
        self.memory.set_io(&state.io);
        self.memory.set_hram(&state.hram);
        self.memory.write_byte(0xFFFF, state.interrupt_enable);
        self.memory.write_byte(0xFF0F, state.interrupt_flag);
        self.memory.set_boot_rom_enabled(state.boot_rom_enabled);
        if let Some(ram) = &state.cartridge_ram {
            self.memory.load_cartridge_ram(ram);
        }
        self.memory.set_mbc_state(&state.mbc_state);
    }
}