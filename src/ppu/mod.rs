use crate::memory::Memory;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
const VBLANK_INTERRUPT: u8 = 0x01;
const LCDC_INTERRUPT: u8 = 0x02;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

pub struct Ppu {
    mode: Mode,
    cycles: u32,
    line: u8,
    screen_buffer: Vec<u8>,
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            mode: Mode::OamScan,
            cycles: 0,
            line: 0,
            screen_buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
            lcdc: 0x91,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
        }
    }

    pub fn update(&mut self, cycles: u8, memory: &mut Memory) {
        if !self.is_lcd_enabled() {
            return;
        }

        self.cycles += cycles as u32;

        match self.mode {
            Mode::OamScan => {
                if self.cycles >= 80 {
                    self.cycles -= 80;
                    self.mode = Mode::Drawing;
                }
            }
            Mode::Drawing => {
                if self.cycles >= 172 {
                    self.cycles -= 172;
                    self.mode = Mode::HBlank;
                    self.render_scanline();
                }
            }
            Mode::HBlank => {
                if self.cycles >= 204 {
                    self.cycles -= 204;
                    self.line += 1;

                    if self.line == 144 {
                        self.mode = Mode::VBlank;
                        memory.request_interrupt(VBLANK_INTERRUPT);
                    } else {
                        self.mode = Mode::OamScan;
                    }
                }
            }
            Mode::VBlank => {
                if self.cycles >= 456 {
                    self.cycles -= 456;
                    self.line += 1;

                    if self.line > 153 {
                        self.line = 0;
                        self.mode = Mode::OamScan;
                    }
                }
            }
        }

        self.update_stat(memory);
        self.sync_registers(memory);
    }

    pub fn get_screen_buffer(&self) -> Vec<u8> {
        self.screen_buffer.clone()
    }

    fn is_lcd_enabled(&self) -> bool {
        (self.lcdc & 0x80) != 0
    }

    fn render_scanline(&mut self) {
        if self.line >= SCREEN_HEIGHT as u8 {
            return;
        }

        for x in 0..SCREEN_WIDTH {
            let pixel_offset = (self.line as usize * SCREEN_WIDTH + x) * 4;
            
            // Simple gradient pattern for testing
            let shade = ((x as f32 / SCREEN_WIDTH as f32) * 255.0) as u8;
            let color = match self.line % 4 {
                0 => [shade, 0, 0],
                1 => [0, shade, 0],
                2 => [0, 0, shade],
                _ => [shade, shade, shade],
            };
            
            self.screen_buffer[pixel_offset] = color[0];
            self.screen_buffer[pixel_offset + 1] = color[1];
            self.screen_buffer[pixel_offset + 2] = color[2];
            self.screen_buffer[pixel_offset + 3] = 255;
        }
    }

    fn update_stat(&mut self, memory: &mut Memory) {
        let mut stat_interrupt = false;

        self.stat = (self.stat & 0xFC) | (self.mode as u8);

        if self.ly == self.lyc {
            self.stat |= 0x04;
            if (self.stat & 0x40) != 0 {
                stat_interrupt = true;
            }
        } else {
            self.stat &= !0x04;
        }

        match self.mode {
            Mode::HBlank if (self.stat & 0x08) != 0 => stat_interrupt = true,
            Mode::VBlank if (self.stat & 0x10) != 0 => stat_interrupt = true,
            Mode::OamScan if (self.stat & 0x20) != 0 => stat_interrupt = true,
            _ => {}
        }

        if stat_interrupt {
            memory.request_interrupt(LCDC_INTERRUPT);
        }
    }

    fn sync_registers(&mut self, memory: &mut Memory) {
        self.lcdc = memory.read_byte(0xFF40);
        self.stat = memory.read_byte(0xFF41);
        self.scy = memory.read_byte(0xFF42);
        self.scx = memory.read_byte(0xFF43);
        self.ly = self.line;
        self.lyc = memory.read_byte(0xFF45);
        self.bgp = memory.read_byte(0xFF47);
        self.obp0 = memory.read_byte(0xFF48);
        self.obp1 = memory.read_byte(0xFF49);
        self.wy = memory.read_byte(0xFF4A);
        self.wx = memory.read_byte(0xFF4B);

        memory.write_byte(0xFF41, self.stat);
        memory.write_byte(0xFF44, self.ly);
    }
}