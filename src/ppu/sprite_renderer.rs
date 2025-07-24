use crate::memory::Memory;

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub y: u8,
    pub x: u8,
    pub tile_index: u8,
    pub attributes: u8,
}

impl Sprite {
    pub fn from_oam(memory: &Memory, index: usize) -> Self {
        let base_addr = 0xFE00 + (index * 4) as u16;
        Self {
            y: memory.read_byte(base_addr),
            x: memory.read_byte(base_addr + 1),
            tile_index: memory.read_byte(base_addr + 2),
            attributes: memory.read_byte(base_addr + 3),
        }
    }
    
    pub fn is_visible(&self) -> bool {
        self.x > 0 && self.x < 168 && self.y > 0 && self.y < 160
    }
    
    pub fn get_palette_number(&self) -> bool {
        (self.attributes & 0x10) != 0
    }
    
    pub fn is_x_flipped(&self) -> bool {
        (self.attributes & 0x20) != 0
    }
    
    pub fn is_y_flipped(&self) -> bool {
        (self.attributes & 0x40) != 0
    }
    
    pub fn has_priority(&self) -> bool {
        (self.attributes & 0x80) == 0
    }
}

pub struct SpriteRenderer;

impl SpriteRenderer {
    pub fn get_sprites_on_line(memory: &Memory, line: u8, sprite_size: bool) -> Vec<Sprite> {
        let mut sprites = Vec::new();
        let sprite_height = if sprite_size { 16 } else { 8 };
        
        for i in 0..40 {
            let sprite = Sprite::from_oam(memory, i);
            
            if sprite.is_visible() {
                let sprite_y = sprite.y.wrapping_sub(16);
                if line >= sprite_y && line < sprite_y + sprite_height {
                    sprites.push(sprite);
                    if sprites.len() >= 10 {
                        break;
                    }
                }
            }
        }
        
        // Sort by X coordinate (lower X has higher priority)
        sprites.sort_by_key(|s| s.x);
        sprites
    }
    
    pub fn get_sprite_pixel(
        sprite: &Sprite,
        x: u8,
        y: u8,
        memory: &Memory,
        sprite_size: bool,
    ) -> Option<u8> {
        let sprite_x = sprite.x.wrapping_sub(8);
        let sprite_y = sprite.y.wrapping_sub(16);
        
        // Check if pixel is within sprite bounds
        if x < sprite_x || x >= sprite_x + 8 {
            return None;
        }
        if y < sprite_y || y >= sprite_y + if sprite_size { 16 } else { 8 } {
            return None;
        }
        
        let mut pixel_x = x - sprite_x;
        let mut pixel_y = y - sprite_y;
        
        // Handle flipping
        if sprite.is_x_flipped() {
            pixel_x = 7 - pixel_x;
        }
        if sprite.is_y_flipped() {
            pixel_y = (if sprite_size { 15 } else { 7 }) - pixel_y;
        }
        
        // Calculate tile index for 8x16 sprites
        let tile_index = if sprite_size {
            if pixel_y < 8 {
                sprite.tile_index & 0xFE
            } else {
                sprite.tile_index | 0x01
            }
        } else {
            sprite.tile_index
        };
        
        // Get pixel from tile data
        let tile_y = pixel_y % 8;
        let base_address = 0x8000 + (tile_index as u16 * 16);
        let line_address = base_address + (tile_y * 2) as u16;
        
        let byte1 = memory.read_byte(line_address);
        let byte2 = memory.read_byte(line_address + 1);
        
        let bit_position = 7 - pixel_x;
        let bit1 = (byte1 >> bit_position) & 1;
        let bit2 = (byte2 >> bit_position) & 1;
        let color_id = (bit2 << 1) | bit1;
        
        // Transparent pixel
        if color_id == 0 {
            None
        } else {
            Some(color_id)
        }
    }
}