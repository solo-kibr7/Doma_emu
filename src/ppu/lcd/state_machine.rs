pub use crate::ppu::{PPU, lcd::{Lcd, Mode}, fifo::{PixelFifo, FetchState}};
pub use crate::mmu::{MMU, interrupts::{Interrupts, InterruptType}};

pub const LINES_PER_FRAME: usize = 154;
pub const TICKS_PER_LINE: usize = 456;
pub const XRES: usize = 160;
pub const YRES: usize = 144;


pub fn increment_ly(ppu: &mut PPU, interrupt: &mut Interrupts) {
    if ppu.window_visible() && (ppu.lcd.ly >= ppu.lcd.window_y)
        && (ppu.lcd.ly < ppu.lcd.window_y + YRES as u8) {
        ppu.window_line += 1;
    }

    ppu.lcd.ly += 1;
    if ppu.lcd.ly == ppu.lcd.ly_compare {
        ppu.lcd.lcd_status.equals_flag_set(true);
        if ppu.lcd.lcd_status.stat_interrupt() {
            interrupt.request_interrupt(InterruptType::LcdStat);
        }
    } else {
        ppu.lcd.lcd_status.equals_flag_set(false);
    }
}

pub fn mode_oam(ppu: &mut PPU) {
    if ppu.line_ticks >= 80 {
        let val = ppu.lcd.lcd_status.mode_value(&Mode::TRANSFER);
        ppu.lcd.lcd_status.current_mode_set(val);

        ppu.pixel_fifo.current_state = FetchState::TileNum;
        ppu.pixel_fifo.line_x = 0;
        ppu.pixel_fifo.fetch_x = 0;
        ppu.pixel_fifo.push_x = 0;
        ppu.pixel_fifo.fifo_x = 0;
    }

    if ppu.line_ticks == 1 {
        //println!("tick");
        ppu.selected_oam_reset();
        ppu.sprite_count = 0;
        ppu.load_sprites();
    }
}

pub fn mode_transfer(ppu: &mut PPU, interrupt: &mut Interrupts) {
    ppu.pipeline_process();

    if ppu.pixel_fifo.push_x >= XRES as u8 {
        ppu.pipeline_reset();

        let val = ppu.lcd.lcd_status.mode_value(&Mode::HBLANK);
        ppu.lcd.lcd_status.current_mode_set(val);

        if ppu.lcd.lcd_status.hblank_interrupt() {
            interrupt.request_interrupt(InterruptType::LcdStat);
        }
    }
}

pub fn mode_vblank(ppu: &mut PPU, interrupt: &mut Interrupts) {
    if ppu.line_ticks >= TICKS_PER_LINE as u32 {
        increment_ly(ppu, interrupt);
        if ppu.lcd.ly >=  LINES_PER_FRAME as u8 {
            let val = ppu.lcd.lcd_status.mode_value(&Mode::OAM);
            ppu.lcd.lcd_status.current_mode_set(val);
            ppu.lcd.ly = 0;
            ppu.window_line = 0;
        }
        ppu.line_ticks = 0;
    }
}

pub fn mode_hblank(ppu: &mut PPU, interrupt: &mut Interrupts) {
    // worry about frame rate later
    if ppu.line_ticks >= TICKS_PER_LINE as u32 {
        increment_ly(ppu, interrupt);

        if ppu.lcd.ly >=  YRES as u8 {
            let val = ppu.lcd.lcd_status.mode_value(&Mode::VBLANK);
            ppu.lcd.lcd_status.current_mode_set(val);
            interrupt.request_interrupt(InterruptType::Vblank);

            if ppu.lcd.lcd_status.vblank_interrupt() {
                interrupt.request_interrupt(InterruptType::LcdStat);
            }
            ppu.current_frame += 1;

        } else {
            let val = ppu.lcd.lcd_status.mode_value(&Mode::OAM);
            ppu.lcd.lcd_status.current_mode_set(val);
        }
        ppu.line_ticks = 0;
    }
}
