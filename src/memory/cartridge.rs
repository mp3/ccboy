use crate::save_state::MbcSaveState;

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enabled: bool,
    mbc_type: MbcType,
}

#[derive(Debug, Clone, Copy)]
enum MbcType {
    NoMbc,
    Mbc1,
    Mbc3,
    Mbc5,
}

impl Cartridge {
    pub fn new(rom_data: &[u8]) -> Self {
        let mbc_type = match rom_data.get(0x147) {
            Some(0x00) => MbcType::NoMbc,
            Some(0x01..=0x03) => MbcType::Mbc1,
            Some(0x0F..=0x13) => MbcType::Mbc3,
            Some(0x19..=0x1E) => MbcType::Mbc5,
            _ => MbcType::NoMbc,
        };

        let ram_size = match rom_data.get(0x149) {
            Some(0x00) => 0,
            Some(0x01) => 0x800,
            Some(0x02) => 0x2000,
            Some(0x03) => 0x8000,
            Some(0x04) => 0x20000,
            Some(0x05) => 0x10000,
            _ => 0,
        };

        Self {
            rom: rom_data.to_vec(),
            ram: vec![0; ram_size],
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            mbc_type,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom.get(address as usize).copied().unwrap_or(0xFF),
            0x4000..=0x7FFF => {
                let bank_offset = self.rom_bank * 0x4000;
                let addr = bank_offset + (address as usize - 0x4000);
                self.rom.get(addr).copied().unwrap_or(0xFF)
            }
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match self.mbc_type {
            MbcType::NoMbc => {}
            MbcType::Mbc1 => self.handle_mbc1_write(address, value),
            MbcType::Mbc3 => self.handle_mbc3_write(address, value),
            MbcType::Mbc5 => self.handle_mbc5_write(address, value),
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled || self.ram.is_empty() {
            return 0xFF;
        }

        let addr = self.ram_bank * 0x2000 + address as usize;
        self.ram.get(addr).copied().unwrap_or(0xFF)
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled || self.ram.is_empty() {
            return;
        }

        let addr = self.ram_bank * 0x2000 + address as usize;
        if let Some(byte) = self.ram.get_mut(addr) {
            *byte = value;
        }
    }

    fn handle_mbc1_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = (value & 0x0F) == 0x0A,
            0x2000..=0x3FFF => {
                let bank = (value & 0x1F) as usize;
                self.rom_bank = if bank == 0 { 1 } else { bank };
            }
            0x4000..=0x5FFF => {
                let bank = (value & 0x03) as usize;
                self.rom_bank = (self.rom_bank & 0x1F) | (bank << 5);
            }
            _ => {}
        }
    }

    fn handle_mbc3_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = (value & 0x0F) == 0x0A,
            0x2000..=0x3FFF => {
                let bank = (value & 0x7F) as usize;
                self.rom_bank = if bank == 0 { 1 } else { bank };
            }
            0x4000..=0x5FFF => self.ram_bank = (value & 0x03) as usize,
            _ => {}
        }
    }

    fn handle_mbc5_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = (value & 0x0F) == 0x0A,
            0x2000..=0x2FFF => self.rom_bank = (self.rom_bank & 0x100) | (value as usize),
            0x3000..=0x3FFF => {
                self.rom_bank = (self.rom_bank & 0xFF) | (((value & 0x01) as usize) << 8);
            }
            0x4000..=0x5FFF => self.ram_bank = (value & 0x0F) as usize,
            _ => {}
        }
    }
    
    // Save state methods
    pub fn get_ram_data(&self) -> Vec<u8> {
        self.ram.clone()
    }
    
    pub fn load_ram_data(&mut self, data: &[u8]) {
        if data.len() == self.ram.len() {
            self.ram.copy_from_slice(data);
        }
    }
    
    pub fn get_mbc_state(&self) -> MbcSaveState {
        MbcSaveState {
            rom_bank: self.rom_bank,
            ram_bank: self.ram_bank,
            ram_enabled: self.ram_enabled,
        }
    }
    
    pub fn set_mbc_state(&mut self, state: &MbcSaveState) {
        self.rom_bank = state.rom_bank;
        self.ram_bank = state.ram_bank;
        self.ram_enabled = state.ram_enabled;
    }
}