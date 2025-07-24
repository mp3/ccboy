mod mmu;
mod cartridge;

pub use mmu::Memory;

#[allow(dead_code)]
pub const ROM_BANK_SIZE: usize = 0x4000;
#[allow(dead_code)]
pub const RAM_BANK_SIZE: usize = 0x2000;
pub const VRAM_SIZE: usize = 0x2000;
pub const WRAM_SIZE: usize = 0x2000;
pub const OAM_SIZE: usize = 0xA0;
pub const IO_SIZE: usize = 0x80;
pub const HRAM_SIZE: usize = 0x7F;