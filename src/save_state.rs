use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SaveState {
    pub cpu: CpuSaveState,
    pub memory: MemorySaveState,
    pub cycles: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CpuSaveState {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub ime: bool,
    pub halt: bool,
    pub cycles: u32,
}

#[derive(Serialize, Deserialize)]
pub struct MemorySaveState {
    pub vram: Vec<u8>,
    pub wram: Vec<u8>,
    pub oam: Vec<u8>,
    pub io: Vec<u8>,
    pub hram: Vec<u8>,
    pub interrupt_enable: u8,
    pub interrupt_flag: u8,
    pub boot_rom_enabled: bool,
    pub cartridge_ram: Option<Vec<u8>>,
    pub mbc_state: MbcSaveState,
}

#[derive(Serialize, Deserialize)]
pub struct MbcSaveState {
    pub rom_bank: usize,
    pub ram_bank: usize,
    pub ram_enabled: bool,
}