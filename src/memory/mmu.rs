use super::*;
use crate::memory::cartridge::Cartridge;
use crate::boot_rom::DMG_BOOT_ROM;
use crate::save_state::MbcSaveState;

pub struct Memory {
    #[allow(dead_code)]
    rom: Vec<u8>,
    pub(crate) vram: [u8; VRAM_SIZE],
    pub(crate) wram: [u8; WRAM_SIZE],
    pub(crate) oam: [u8; OAM_SIZE],
    pub(crate) io: [u8; IO_SIZE],
    pub(crate) hram: [u8; HRAM_SIZE],
    interrupt_enable: u8,
    interrupt_flag: u8,
    pub(crate) cartridge: Option<Cartridge>,
    boot_rom_enabled: bool,
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
            boot_rom_enabled: true,
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.cartridge = Some(Cartridge::new(rom_data));
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x00FF if self.boot_rom_enabled => {
                DMG_BOOT_ROM[address as usize]
            }
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
            0xFF50 => self.boot_rom_enabled = false, // Disable boot ROM
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
    
    // Save state methods
    pub fn get_vram(&self) -> &[u8] {
        &self.vram
    }
    
    pub fn get_wram(&self) -> &[u8] {
        &self.wram
    }
    
    pub fn get_oam(&self) -> &[u8] {
        &self.oam
    }
    
    pub fn get_io(&self) -> &[u8] {
        &self.io
    }
    
    pub fn get_hram(&self) -> &[u8] {
        &self.hram
    }
    
    pub fn is_boot_rom_enabled(&self) -> bool {
        self.boot_rom_enabled
    }
    
    pub fn get_cartridge_ram(&self) -> Vec<u8> {
        if let Some(cartridge) = &self.cartridge {
            cartridge.get_ram_data()
        } else {
            Vec::new()
        }
    }
    
    pub fn get_cartridge_ram_vec(&self) -> Option<Vec<u8>> {
        self.cartridge.as_ref().map(|c| c.get_ram_data())
    }
    
    pub fn get_mbc_state(&self) -> MbcSaveState {
        if let Some(cartridge) = &self.cartridge {
            cartridge.get_mbc_state()
        } else {
            MbcSaveState {
                rom_bank: 1,
                ram_bank: 0,
                ram_enabled: false,
            }
        }
    }
    
    pub fn set_vram(&mut self, data: &[u8]) {
        self.vram.copy_from_slice(data);
    }
    
    pub fn set_wram(&mut self, data: &[u8]) {
        self.wram.copy_from_slice(data);
    }
    
    pub fn set_oam(&mut self, data: &[u8]) {
        self.oam.copy_from_slice(data);
    }
    
    pub fn set_io(&mut self, data: &[u8]) {
        self.io.copy_from_slice(data);
    }
    
    pub fn set_hram(&mut self, data: &[u8]) {
        self.hram.copy_from_slice(data);
    }
    
    pub fn set_boot_rom_enabled(&mut self, enabled: bool) {
        self.boot_rom_enabled = enabled;
    }
    
    pub fn load_cartridge_ram(&mut self, data: &[u8]) {
        if let Some(cartridge) = &mut self.cartridge {
            cartridge.load_ram_data(data);
        }
    }
    
    pub fn set_mbc_state(&mut self, state: &MbcSaveState) {
        if let Some(cartridge) = &mut self.cartridge {
            cartridge.set_mbc_state(state);
        }
    }
}