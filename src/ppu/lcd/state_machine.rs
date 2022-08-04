pub use crate::ppu::{PPU, lcd::{Lcd, Mode}};
pub use crate::mmu::{MMU, interrupts::{Interrupts, InterruptType}};

use std::{thread, time::{Duration, Instant}};


pub const LINES_PER_FRAME: usize = 154;
pub const TICKS_PER_LINE: usize = 456;
pub const XRES: usize = 144;
pub const YRES: usize = 160;

pub struct Fps_manager {
    current_time: u128,
    prev_time: u128,
}
impl Fps_manager {
    pub const fn new() -> Fps_manager {
        Fps_manager {
            current_time: 0,
            prev_time: 0,
        }
    }
    pub fn write_prev_time(&mut self, time: u128) {
        self.prev_time = time;
    }
    pub const fn read_prev_time(&self) -> u128 {
        self.prev_time
    }

    pub fn write_current_time(&mut self, time: u128) {
        self.current_time = time;
    }
    pub const fn read_current_time(&self) -> u128 {
        self.current_time
    }
}

pub fn increment_ly(lcd: &mut Lcd, interrupt: &mut Interrupts) {
    lcd.ly += 1;
    if lcd.ly == lcd.ly_compare {
        lcd.lcd_status.equals_flag_set(true);
        if lcd.lcd_status.stat_interrupt() {
            interrupt.request_interrupt(InterruptType::LcdStat);
        }
    } else {
        lcd.lcd_status.equals_flag_set(false);
    }
}

pub fn mode_oam(ppu: &mut PPU) {
    if ppu.line_ticks >= 80 {
        let val = ppu.lcd.lcd_status.mode_value(&Mode::TRANSFER);
        ppu.lcd.lcd_status.current_mode_set(val);
    }
}

pub fn mode_transfer(ppu: &mut PPU) {
    if ppu.line_ticks >= 80 + 172 {
        let val = ppu.lcd.lcd_status.mode_value(&Mode::HBLANK);
        ppu.lcd.lcd_status.current_mode_set(val);
    }
}

pub fn mode_vblank(ppu: &mut PPU, interrupt: &mut Interrupts) {
    if ppu.line_ticks >= TICKS_PER_LINE as u32 {
        increment_ly(&mut ppu.lcd, interrupt);
        if ppu.lcd.ly >=  LINES_PER_FRAME as u8 {
            let val = ppu.lcd.lcd_status.mode_value(&Mode::OAM);
            ppu.lcd.lcd_status.current_mode_set(val);
            ppu.lcd.ly = 0;
        }
        ppu.line_ticks = 0;
    }
}

const target_frame_time:u32 = 1000/60;
static prev_frame_time: u128 = 0;

pub fn mode_hblank(ppu: &mut PPU, interrupt: &mut Interrupts) {
    // worry about frame rate later
    if ppu.line_ticks >= TICKS_PER_LINE as u32 {
        increment_ly(&mut ppu.lcd, interrupt);

        if ppu.lcd.ly >=  YRES as u8 {
            let val = ppu.lcd.lcd_status.mode_value(&Mode::VBLANK);
            ppu.lcd.lcd_status.current_mode_set(val);
            interrupt.request_interrupt(InterruptType::Vblank);

            if ppu.lcd.lcd_status.vblank_interrupt() {
                interrupt.request_interrupt(InterruptType::LcdStat);
            }
            ppu.current_frame += 1;

            /* let end:u128 = u128::from(crate::now.elapsed().as_millis());

            let frame_time:u128 = end - ppu.fps.read_prev_frame_time();

            if (frame_time) < target_frame_time as u128 {
                thread::sleep(Duration::from_millis((target_frame_time as u128 - frame_time) as u64));
            }

            ppu.fps.new_prev_frame_time(end); */
        } else {
            let val = ppu.lcd.lcd_status.mode_value(&Mode::OAM);
            ppu.lcd.lcd_status.current_mode_set(val);
        }
        ppu.line_ticks = 0;
    }
}

pub fn current_time(time: u64) -> u64 {
    time
}