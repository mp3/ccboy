use super::*;
use crate::memory::cartridge::Cartridge;

pub struct Memory {
    rom: Vec<u8>,
    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    oam: [u8; OAM_SIZE],
    io: [u8; IO_SIZE],
    hram: [u8; HRAM_SIZE],
    interrupt_enable: u8,
    interrupt_flag: u8,
    cartridge: Option<Cartridge>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            rom: vec![0; 0x8000],
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io: [0; IO_SIZE],
            hram: [0; HRAM_SIZE],
            interrupt_enable: 0,
            interrupt_flag: 0,
            cartridge: None,
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.cartridge = Some(Cartridge::new(rom_data));
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => {
                if let Some(cart) = &self.cartridge {
                    cart.read_byte(address)
                } else {
                    0xFF
                }
            }
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => {
                if let Some(cart) = &self.cartridge {
                    cart.read_ram(address - 0xA000)
                } else {
                    0xFF
                }
            }
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF,
            0xFF00..=0xFF7F => self.read_io(address),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => {
                if let Some(cart) = &mut self.cartridge {
                    cart.write_byte(address, value);
                }
            }
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xA000..=0xBFFF => {
                if let Some(cart) = &mut self.cartridge {
                    cart.write_ram(address - 0xA000, value);
                }
            }
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {}
            0xFF00..=0xFF7F => self.write_io(address, value),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.interrupt_enable = value,
        }
    }

    fn read_io(&self, address: u16) -> u8 {
        match address {
            0xFF0F => self.interrupt_flag,
            _ => self.io[(address - 0xFF00) as usize],
        }
    }

    fn write_io(&mut self, address: u16, value: u8) {
        match address {
            0xFF0F => self.interrupt_flag = value,
            _ => self.io[(address - 0xFF00) as usize] = value,
        }
    }

    pub fn get_triggered_interrupts(&self) -> u8 {
        self.interrupt_enable & self.interrupt_flag & 0x1F
    }

    pub fn clear_interrupt(&mut self, interrupt: u8) {
        self.interrupt_flag &= !interrupt;
    }

    pub fn request_interrupt(&mut self, interrupt: u8) {
        self.interrupt_flag |= interrupt;
    }

    pub fn update_joypad(&mut self, state: u8) {
        self.io[0] = state;
    }
}