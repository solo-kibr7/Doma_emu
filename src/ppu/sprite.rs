use bitflags::bitflags;

/*
 Bit7   BG and Window over OBJ (0=No, 1=BG and Window colors 1-3 over the OBJ)
 Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
 Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
 Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
 Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
 Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)
 */

bitflags! {
    #[derive(Default)]
    struct SpriteFlags: u8 {
        const PRIORITY = 1 << 7;
        const Y_FLIP = 1 << 6;
        const X_FLIP = 1 << 5;
        const DMG_PALETTE = 1 << 4;
        const VRAM_BANK = 1 << 3;
        const CGB_PALETTE = 0b111;
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Sprite {
    y: u8,
    x: u8,
    tile: u8,
    flags: SpriteFlags,
}

impl Sprite {
    pub fn get_at_offset(&self, offset:u8) -> u8 {
        match offset {
            0 => self.y,
            1 => self.x,
            2 => self.tile,
            3 => self.flags.bits(),
            _ => unreachable!(),
        }
    }
    pub fn set_at_offset(&mut self, offset: u8, data: u8) {
        match offset {
            0 => self.y = data,
            1 => self.x = data,
            2 => self.tile = data,
            3 => self
                .flags
                .clone_from(&SpriteFlags::from_bits_truncate(data)),
            _ => unreachable!(),
        }
    }

    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn set_y(&mut self, data:u8) {
        self.y = data;
    }
    pub fn set_x(&mut self, data:u8) {
        self.x = data;
    }

    pub fn tile(&self) -> u8 {
        self.tile
    }
    pub fn set_tile(&mut self, data:u8) {
        self.tile = data;
    }

    pub fn flags(&self) -> u8 {
        self.flags.bits()
    }
    pub fn set_flags(&mut self, data:u8) {
        self.flags = SpriteFlags::from_bits_truncate(data);
    }
}