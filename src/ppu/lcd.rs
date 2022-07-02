use bitflags::bitflags;

pub enum Mode {
    HBLANK,
    VBLANK,
    OAM,
    XFER,
}

bitflags! {
    struct LcdControl: u8 {
        const DISPLAY_ENABLE      = 1 << 7;
        const WINDOW_TILEMAP      = 1 << 6;
        const WINDOW_ENABLE       = 1 << 5;
        const BG_WINDOW_TILE_DATA = 1 << 4;
        const BG_TILEMAP          = 1 << 3;
        const OBJ_SIZE            = 1 << 2;
        const OBJ_ENABLE          = 1 << 1;
        const BG_WINDOW_PRIORITY  = 1 << 0;
    }
}

impl LcdControl {
    fn display_enable(&self) -> bool {
        self.intersects(Self::DISPLAY_ENABLE)
    }

    fn window_tilemap(&self) -> u16 {
        if self.intersects(Self::WINDOW_TILEMAP) {
            0x9C00
        } else {
            0x9800
        }
    }

    fn window_enable(&self) -> bool {
        self.intersects(Self::WINDOW_ENABLE)
    }

    fn bg_window_tile_data(&self) -> u16 {
        if self.intersects(Self::BG_WINDOW_TILE_DATA) {
            0x8800
        } else {
            0x8000
        }
    }

    fn bg_tilemap(&self) -> u16 {
        if self.intersects(Self::BG_TILEMAP) {
            0x9C00
        } else {
            0x9800
        }
    }

    fn sprite_size(&self) -> u8 {
        if self.intersects(Self::OBJ_SIZE) {
            16
        } else {
            8
        }
    }

    fn sprite_enable(&self) -> bool {
        self.intersects(Self::OBJ_ENABLE)
    }

    fn bg_window_priority(&self) -> bool {
        self.intersects(Self::BG_WINDOW_PRIORITY)
    }
}

bitflags! {
    struct LcdStatus: u8 {
        const STAT_INTERRUPT   = 1 << 6;
        const OAM_INTERRUPT    = 1 << 5;
        const VBLANK_INTERRUPT = 1 << 4;
        const HBLANK_INTERRUPT = 1 << 3;
        const EQUALS_FLAG      = 1 << 2;
        const MODE_FLAG        = 0b11;
    }
}

impl LcdStatus {
    fn stat_interrupt(&self) -> bool {
        self.intersects(Self::STAT_INTERRUPT)
    }

    fn oam_interrupt(&self) -> bool {
        self.intersects(Self::OAM_INTERRUPT)
    }

    fn vblank_interrupt(&self) -> bool {
        self.intersects(Self::VBLANK_INTERRUPT)
    }

    fn hblank_interrupt(&self) -> bool {
        self.intersects(Self::HBLANK_INTERRUPT)
    }

    fn equals_flag_set(&mut self, value: bool) {
        self.set(Self::EQUALS_FLAG, value);
    }

    fn current_mode(&self) -> u8 {
        self.bits() & Self::MODE_FLAG.bits
    }

    fn current_mode_set(&mut self, data: u8) {
        self.clone_from(&Self::from_bits_truncate(
            (self.bits() & !0b11) | data & 0b11,
        ));
        assert!(self.current_mode() == data & 0b11);
    }
}

pub const COLORS_DEFAULT: [u32; 4] = [0xFFFFFFFF, 0xFFAAAAAA, 0xFF555555, 0xFF000000]; 

pub struct Lcd {
    lcd_control: LcdControl,
    lcd_status: LcdStatus,
    scroll_y: u8,
    scroll_x: u8,
    ly: u8, // current scanline
    ly_compare: u8,
    dmg_bg_palette: u8,
    dmg_sprite_palette: [u8; 2],
    window_y: u8,
    window_x: u8,
    bg_colors: [u32; 4],
    sp1_colors: [u32; 4],
    sp2_colors: [u32; 4],

}

impl Default for Lcd {
    fn default() -> Self {
        Self {
            lcd_control: LcdControl::from_bits_truncate(0x91),
            lcd_status: LcdStatus::from_bits_truncate(4),
            scroll_y: 0,
            scroll_x: 0,
            ly: 0, // current scanline
            ly_compare: 0,
            dmg_bg_palette: 0xFC,
            dmg_sprite_palette: [0xFF; 2],
            window_y: 0,
            window_x: 0,
            bg_colors:[0xFFFFFF, 0x555555, 0xAAAAAA, 0x000000],
            sp1_colors: [0xFFFFFF, 0x555555, 0xAAAAAA, 0x000000],
            sp2_colors: [0xFFFFFF, 0x555555, 0xAAAAAA, 0x000000],
        }
    }
}

impl Lcd {
    pub fn lcd_read(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcd_control.bits(),
            0xFF41 => self.lcd_status.bits(),
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.ly,
            0xFF45 => self.ly_compare,
            0xFF47 => self.dmg_bg_palette,
            0xFF48 => self.dmg_sprite_palette[0],
            0xFF49 => self.dmg_sprite_palette[1],
            0xFF4A => self.window_y,
            0xFF4B => self.window_x,
            _ => unreachable!("Unsupported address {:#X}. How did this happen lol!", address),
        }
    }

    pub fn lcd_write(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => self.lcd_control.clone_from(&LcdControl::from_bits_truncate(value)),
            0xFF41 => self.lcd_status.clone_from(&LcdStatus::from_bits_truncate(value)),
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF44 => self.ly = value,
            0xFF45 => self.ly_compare = value,
            0xFF47 => {self.dmg_bg_palette = value; self.update_palette(value, false);},
            0xFF48 => {self.dmg_sprite_palette[0] = value; self.update_palette(value & 0b11111100, true);},
            0xFF49 => {self.dmg_sprite_palette[1] = value; self.update_palette(value & 0b11111100, true);},
            0xFF4A => self.window_y = value,
            0xFF4B => self.window_x = value,
            _ => unreachable!("Unsupported address {:#X}. How did this happen lol!", address),
        }
    }

    pub fn update_palette(&mut self, palette_data: u8, pal_1: bool) {
        if pal_1 {
            for i in 0..4 {
                self.sp1_colors[i] = COLORS_DEFAULT[((palette_data >> 2 * i) & 0b11) as usize];
            }
        } else {
            for i in 0..4 {
                self.sp2_colors[i] = COLORS_DEFAULT[((palette_data >> 2 * i) & 0b11) as usize];
            }
        }
    }
}