pub mod sprite;
pub mod lcd;

use sprite::Sprite;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

pub struct PPU {
    /*mode: u8,
    mode_clock: usize,
    background_buffer: Vec<u32>,
    viewport: Vec<u32>,*/
    vram: [u8; 0x2000],
    oam: [Sprite; 40],
    pub(super) lcd: lcd::Lcd,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [Sprite::default(); 40],
            lcd: lcd::Lcd::default(),
        }
    }
}

impl PPU {
    pub fn read_vram(&self, address: u16) -> u8 {
        self.vram[(address - 0x8000) as usize]
    }
    pub fn write_vram(&mut self, address: u16, value:u8) {
        self.vram[(address - 0x8000) as usize] = value;
    }

    pub fn read_oam(&self, address: u16) -> u8 {
        let index = (address & 0xFF) as usize;
        //println!("read, index:{}, i2:{}, i3:{}", index, index/4, index%4);
        self.oam[index / 4].get_at_offset((index % 4) as u8)
    }
    pub fn write_oam(&mut self, address: u16, data:u8) {
        let index = (address & 0xFF) as usize;
        //println!("write, index:{}, i2:{}, i3:{}", index, index/4, index%4);
        self.oam[index / 4].set_at_offset((index % 4) as u8, data);
    }
}