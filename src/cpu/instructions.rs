use super::Cpu;

impl Cpu {
    #[allow(dead_code)]
    pub fn nop(&mut self) -> u8 {
        4
    }

    #[allow(dead_code)]
    pub fn halt(&mut self) -> u8 {
        self.halt = true;
        4
    }
}