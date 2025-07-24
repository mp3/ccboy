use super::Cpu;
use crate::memory::Memory;

impl Cpu {
    pub fn nop(&mut self) -> u8 {
        4
    }

    pub fn halt(&mut self) -> u8 {
        self.halt = true;
        4
    }
}