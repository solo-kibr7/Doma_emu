pub mod sprite;
pub mod lcd;
pub mod fifo;

use sprite::Sprite;
pub use crate::mmu::interrupts::{Interrupts, InterruptType};

pub struct PPU {
    /*mode: u8,
    mode_clock: usize,
    background_buffer: Vec<u32>,
    viewport: Vec<u32>,*/
    vram: [u8; 0x2000],
    oam: [Sprite; 40],
    pub current_frame: u32,
    scanline: u8,
    line_ticks: u32,
    pub(super) lcd: lcd::Lcd,
    fps: lcd::state_machine::Fps_manager,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [Sprite::default(); 40],
            current_frame: 0,
            scanline: 153,
            line_ticks: 400,
            lcd: lcd::Lcd::default(),
            fps: lcd::state_machine::Fps_manager::new(),
            //add more
        }
    }
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            vram: [0; 0x2000],
            oam: [Sprite::default(); 40],
            current_frame: 0,
            scanline: 0,
            line_ticks: 0,
            lcd: lcd::Lcd::default(),
            fps: lcd::state_machine::Fps_manager::new(),
        }
    }

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

    pub fn ppu_ticks(&mut self, interrupt: &mut Interrupts) {
        self.line_ticks += 1;
        match self.lcd.lcd_status.current_mode() {
            0 => lcd::state_machine::mode_hblank(self, interrupt),
            1 => lcd::state_machine::mode_vblank(self, interrupt),
            2 => lcd::state_machine::mode_oam(self),
            3 => lcd::state_machine::mode_transfer(self),
            _ => unreachable!("ppu tick: unreachable mode error. like how did this happen lol"),
        }
    }
}