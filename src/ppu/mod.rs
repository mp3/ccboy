mod tile_renderer;
mod sprite_renderer;

use crate::memory::Memory;
use tile_renderer::TileRenderer;
use sprite_renderer::SpriteRenderer;

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
        // Read LCDC from memory
        self.lcdc = memory.read_byte(0xFF40);
        
        if !self.is_lcd_enabled() {
            self.line = 0;
            self.cycles = 0;
            self.mode = Mode::OamScan;
            memory.write_byte(0xFF44, 0); // Reset LY
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
                    self.render_scanline(memory);
                }
            }
            Mode::HBlank => {
                if self.cycles >= 204 {
                    self.cycles -= 204;
                    self.line += 1;
                    memory.write_byte(0xFF44, self.line); // Update LY register

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
                    
                    if self.line <= 153 {
                        memory.write_byte(0xFF44, self.line); // Update LY register
                    } else {
                        self.line = 0;
                        memory.write_byte(0xFF44, 0); // Reset LY
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

    fn render_scanline(&mut self, memory: &Memory) {
        if self.line >= SCREEN_HEIGHT as u8 {
            return;
        }

        // LCDC register bits
        let bg_enabled = (self.lcdc & 0x01) != 0;
        let sprites_enabled = (self.lcdc & 0x02) != 0;
        let sprite_size = (self.lcdc & 0x04) != 0;
        let bg_tile_map = (self.lcdc & 0x08) != 0;
        let bg_tile_data = (self.lcdc & 0x10) == 0;
        let window_enabled = (self.lcdc & 0x20) != 0;
        let window_tile_map = (self.lcdc & 0x40) != 0;
        
        // Get sprites for this line
        let sprites = if sprites_enabled {
            SpriteRenderer::get_sprites_on_line(memory, self.line, sprite_size)
        } else {
            Vec::new()
        };
        
        for x in 0..SCREEN_WIDTH {
            let mut color_id = 0u8;
            let mut final_palette = self.bgp;
            let mut _bg_priority = true;
            
            // Render background
            if bg_enabled {
                let bg_x = (x as u8).wrapping_add(self.scx);
                let bg_y = self.line.wrapping_add(self.scy);
                
                let tile_index = TileRenderer::get_background_tile_index(
                    memory,
                    bg_x,
                    bg_y,
                    bg_tile_map
                );
                
                let tile_data = TileRenderer::get_tile_data(
                    memory,
                    tile_index,
                    bg_tile_data
                );
                
                let pixel_x = bg_x % 8;
                let pixel_y = bg_y % 8;
                color_id = tile_data[pixel_y as usize][pixel_x as usize];
            }
            
            // Render window
            if window_enabled && self.line >= self.wy && x as u8 >= self.wx.saturating_sub(7) {
                let window_x = (x as u8).saturating_sub(self.wx.saturating_sub(7));
                let window_y = self.line.saturating_sub(self.wy);
                
                let tile_index = TileRenderer::get_window_tile_index(
                    memory,
                    window_x,
                    window_y,
                    window_tile_map
                );
                
                let tile_data = TileRenderer::get_tile_data(
                    memory,
                    tile_index,
                    bg_tile_data
                );
                
                let pixel_x = window_x % 8;
                let pixel_y = window_y % 8;
                color_id = tile_data[pixel_y as usize][pixel_x as usize];
            }
            
            // Check for sprite pixels
            for sprite in &sprites {
                if let Some(sprite_color) = SpriteRenderer::get_sprite_pixel(
                    sprite,
                    x as u8,
                    self.line,
                    memory,
                    sprite_size
                ) {
                    if sprite.has_priority() || color_id == 0 {
                        color_id = sprite_color;
                        final_palette = if sprite.get_palette_number() {
                            self.obp1
                        } else {
                            self.obp0
                        };
                        _bg_priority = false;
                    }
                    break;
                }
            }
            
            // Apply palette and convert to RGB
            let shade = TileRenderer::apply_palette(color_id, final_palette);
            let rgb = TileRenderer::get_rgb_color(shade);
            
            let pixel_offset = (self.line as usize * SCREEN_WIDTH + x) * 4;
            self.screen_buffer[pixel_offset] = rgb[0];
            self.screen_buffer[pixel_offset + 1] = rgb[1];
            self.screen_buffer[pixel_offset + 2] = rgb[2];
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