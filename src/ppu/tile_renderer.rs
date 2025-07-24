use crate::memory::Memory;

#[allow(dead_code)]
const TILE_SIZE: usize = 16;  // 16 bytes per tile (8x8 pixels, 2 bits per pixel)
#[allow(dead_code)]
const TILE_WIDTH: usize = 8;
#[allow(dead_code)]
const TILE_HEIGHT: usize = 8;

pub struct TileRenderer;

impl TileRenderer {
    pub fn get_tile_data(memory: &Memory, tile_index: u8, tile_data_select: bool) -> [[u8; 8]; 8] {
        let mut tile_data = [[0u8; 8]; 8];
        
        let base_address = if tile_data_select {
            // Signed addressing mode (0x8800 - 0x97FF)
            let signed_index = tile_index as i8;
            0x9000u16.wrapping_add((signed_index as i16 * 16) as u16)
        } else {
            // Unsigned addressing mode (0x8000 - 0x8FFF)
            0x8000 + (tile_index as u16 * 16)
        };
        
        for y in 0..8 {
            let line_address = base_address + (y * 2) as u16;
            let byte1 = memory.read_byte(line_address);
            let byte2 = memory.read_byte(line_address + 1);
            
            for x in 0..8 {
                let bit_position = 7 - x;
                let bit1 = (byte1 >> bit_position) & 1;
                let bit2 = (byte2 >> bit_position) & 1;
                tile_data[y][x] = (bit2 << 1) | bit1;
            }
        }
        
        tile_data
    }
    
    pub fn get_background_tile_index(memory: &Memory, x: u8, y: u8, tile_map_select: bool) -> u8 {
        let tile_map_base = if tile_map_select { 0x9C00 } else { 0x9800 };
        let tile_x = x / 8;
        let tile_y = y / 8;
        let tile_offset = (tile_y as u16 * 32) + tile_x as u16;
        
        memory.read_byte(tile_map_base + tile_offset)
    }
    
    pub fn get_window_tile_index(memory: &Memory, x: u8, y: u8, tile_map_select: bool) -> u8 {
        let tile_map_base = if tile_map_select { 0x9C00 } else { 0x9800 };
        let tile_x = x / 8;
        let tile_y = y / 8;
        let tile_offset = (tile_y as u16 * 32) + tile_x as u16;
        
        memory.read_byte(tile_map_base + tile_offset)
    }
    
    pub fn apply_palette(color_id: u8, palette: u8) -> u8 {
        let shift = color_id * 2;
        (palette >> shift) & 0x03
    }
    
    pub fn get_rgb_color(shade: u8) -> [u8; 3] {
        match shade {
            0 => [0xE4, 0xE4, 0xE4],  // White
            1 => [0xA8, 0xA8, 0xA8],  // Light gray
            2 => [0x54, 0x54, 0x54],  // Dark gray
            3 => [0x00, 0x00, 0x00],  // Black
            _ => [0xFF, 0x00, 0xFF],  // Error color (magenta)
        }
    }
}