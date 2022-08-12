pub mod sprite;
pub mod lcd;
pub mod fifo;

use sprite::{Sprite, SelectedSprite};
use fifo::{PixelFifo, FetchState};

pub use crate::mmu::interrupts::{Interrupts, InterruptType};

use arrayvec::ArrayVec;

pub const XRES: usize = 160; // already in state_machine. FIX THIS!!!
pub const YRES: usize = 144;
pub const SCALE: usize = 1;
pub const TILE_COLORS: [u32; 4] = [0xFFFFFF, 0xAAAAAA, 0x555555, 0x000000];


pub struct PPU {
    /*mode: u8,
    mode_clock: usize,
    background_buffer: Vec<u32>,
    viewport: Vec<u32>,*/
    vram: [u8; 0x2000],
    oam: [Sprite; 40],
    selected_oam: ArrayVec<SelectedSprite, 10>,
    sprite_count: u8,
    fetched_sprites: [Sprite; 3],
    fetched_sprite_count: u8,
    pub current_frame: u32,
    window_line: u8,
    line_ticks: u32,
    pub(super) pixel_fifo: PixelFifo,
    pub(super) lcd: lcd::Lcd,
    pub(super) video_buffer: Vec<u32>,
}

impl Default for PPU {
    fn default() -> Self {
        let mut ppu = Self {
            vram: [0; 0x2000],
            oam: [Sprite::default(); 40],
            selected_oam: ArrayVec::<_, 10>::new(),
            sprite_count: 0,
            fetched_sprites: [Sprite::default(); 3],
            fetched_sprite_count: 0,
            current_frame: 0,
            window_line: 0,
            line_ticks: 400,
            pixel_fifo: PixelFifo::default(),
            lcd: lcd::Lcd::default(),
            video_buffer: vec![0; XRES * YRES],
            //add more
        };
        ppu.lcd.lcd_write(0xFF47, 0xFC);
        ppu.lcd.lcd_write(0xFF48, 0xFF);
        ppu.lcd.lcd_write(0xFF49, 0xFF);
        ppu
    }
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            vram: [0; 0x2000],
            oam: [Sprite::default(); 40],
            selected_oam: ArrayVec::<_, 10>::new(),
            sprite_count: 0,
            fetched_sprites: [Sprite::default(); 3],
            fetched_sprite_count: 0,
            current_frame: 0,
            window_line: 0,
            line_ticks: 0,
            pixel_fifo: PixelFifo::default(),
            lcd: lcd::Lcd::new(),
            video_buffer: vec![0; XRES * YRES],
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
        println!("write, index:{:#X}, i2:{}, i3:{}, data:{}", index, index/4, index%4, data);
        self.oam[index / 4].set_at_offset((index % 4) as u8, data);
    }

    pub fn ppu_ticks(&mut self, interrupt: &mut Interrupts) {
        self.line_ticks += 1;
        match self.lcd.lcd_status.current_mode() {
            0 => lcd::state_machine::mode_hblank(self, interrupt),
            1 => lcd::state_machine::mode_vblank(self, interrupt),
            2 => lcd::state_machine::mode_oam(self),
            3 => lcd::state_machine::mode_transfer(self, interrupt),
            _ => unreachable!("ppu tick: unreachable mode error. like how did this happen lol"),
        }
    }

    pub fn pipeline_process(&mut self) {
        self.pixel_fifo.map_y = self.lcd.ly.wrapping_add(self.lcd.scroll_y);
        self.pixel_fifo.map_x = self.pixel_fifo.fetch_x.wrapping_add(self.lcd.scroll_x);
        self.pixel_fifo.tile_y = ((self.lcd.ly.wrapping_add(self.lcd.scroll_y)) % 8) * 2;

        if self.line_ticks & 0x1 == 0 {
            self.pipeline_fetch();
        }

        self.pipeline_push_pixel();
    }

    pub fn pipeline_fetch(&mut self) {
        match self.pixel_fifo.current_state {
            FetchState::TileNum => self.fetch_tile(),
            FetchState::DataLow => self.fetch_data(0),
            FetchState::DataHigh => self.fetch_data(1),
            FetchState::Sleep => self.pixel_fifo.current_state = FetchState::Push,
            FetchState::Push => self.fetch_push(),
        };
    }

    pub fn fetch_tile(&mut self) {
        self.fetched_sprite_count = 0;

        if self.lcd.lcd_control.bg_window_priority() {
            /* self.pixel_fifo.tile_x = ((self.lcd.scroll_x / 8) + self.pixel_fifo.fetch_x) & 0x1F;
            self.pixel_fifo.tile_y = self.lcd.ly.wrapping_add(self.lcd.scroll_y);

            let tile_index = (self.pixel_fifo.tile_x as u16).wrapping_add(32 * ((self.pixel_fifo.tile_y / 8) as u16)); */

            self.pixel_fifo.bgw_fetch_data[0] = self.read_vram(self.lcd.lcd_control.bg_tilemap()
                + (self.pixel_fifo.map_x / 8) as u16 + ((self.pixel_fifo.map_y / 8) as u16 * 32) as u16);
            /* self.pixel_fifo.bgw_fetch_data[0] = self.read_vram(self.lcd.lcd_control.bg_tilemap()
                + tile_index); */
            
            if self.lcd.lcd_control.bg_window_tile_data() == 0x8800 {
                self.pixel_fifo.bgw_fetch_data[0] = self.pixel_fifo.bgw_fetch_data[0].wrapping_add(0x80);
            }

            self.pipeline_load_window_tile();
        }
        //println!("selected_oam:{}", self.selected_oam.len());
        if self.lcd.lcd_control.sprite_enable() && !self.selected_oam.is_empty() {
            //println!("load");
            self.pipeline_load_sprite_tile();
        }

        self.pixel_fifo.current_state = FetchState::DataLow;
        self.pixel_fifo.fetch_x += 8;
    }

    pub fn fetch_data(&mut self, row: u8) {
        self.pixel_fifo.bgw_fetch_data[row as usize + 1] = self.read_vram(
            self.lcd.lcd_control.bg_window_tile_data() + self.pixel_fifo.bgw_fetch_data[0] as u16 * 16
            + (self.pixel_fifo.tile_y + row) as u16);

        self.pipeline_load_sprite_data(row);
        
        match row {
            0 => self.pixel_fifo.current_state = FetchState::DataHigh,
            1 => self.pixel_fifo.current_state = FetchState::Sleep,
            _ => unreachable!("fetch_data: unreachable"),
        };
    }

    pub fn fetch_push(&mut self) {
        if self.pixel_fifo.bgfifo.length() <= 8 {
            let x  = self.pixel_fifo.fetch_x as i16 - (8 - self.lcd.scroll_x % 8) as i16;

            for i in 0..8 {
                let bit = 7 - i;
                let hi: bool = self.pixel_fifo.bgw_fetch_data[2] & (1 << bit) != 0;
                let low: bool = self.pixel_fifo.bgw_fetch_data[1] & (1 << bit) != 0;

                let mut color:u8 = 0x0;
                if hi {color |= 0b10;}
                if low {color |= 0b01;}

                if !self.lcd.lcd_control.bg_window_priority() {
                    color = 0x0;
                }

                let mut bg_color:u32 = self.lcd.bg_colors[color as usize];

                let sprite_enable = self.lcd.lcd_control.sprite_enable();

                let display_color: u32;

                if x >= 0 {
                    display_color = if sprite_enable {
                        let (priority, sp_color) = self.fetch_sprite_pixels(color, &mut bg_color);
                        sp_color
                    } else {
                        bg_color
                    };
                    self.pixel_fifo.bgfifo.push(display_color, true);
                    
                    self.pixel_fifo.fifo_x += 1;
                }
            }
            self.pixel_fifo.current_state = FetchState::TileNum;
        }
    }

    pub fn pipeline_push_pixel(&mut self) {
        if self.pixel_fifo.bgfifo.length() > 8 {
            let bg_pixel = self.pixel_fifo.bgfifo.pop();

            /* let color: u32 = if !bg_pixel.bg_priority() {
                self.pixel_fifo.spfifo.pop().get_color()
            } else {
                bg_pixel.get_color()
            }; */

            let color: u32 = bg_pixel.get_color();

            if self.pixel_fifo.line_x >= self.lcd.scroll_x % 8 {
                let buffer_index = self.pixel_fifo.push_x as usize + self.lcd.ly as usize * XRES;
                
                self.video_buffer[buffer_index] = color;
                self.pixel_fifo.push_x = self.pixel_fifo.push_x.wrapping_add(1);
            }
            self.pixel_fifo.line_x = self.pixel_fifo.line_x.wrapping_add(1);
        }
    }

    pub fn pipeline_reset(&mut self) {
        for i in 0..self.pixel_fifo.bgfifo.length() {
            self.pixel_fifo.bgfifo.pop();
        }
    }

    pub fn load_sprites(&mut self) {
        let cur_y = self.lcd.ly;
        let size  = self.lcd.lcd_control.sprite_size();
        //let mut count = 0;
    
        for (i, &sprite) in self.oam.iter().enumerate() {
            //if sprite.x() > 0 {println!("x:{}", sprite.x());}
            if sprite.x() == 0 {
                continue;
            }
            
            if self.selected_oam.len() >= 10 {
                break;
            }
            
    
            if (sprite.y() <= cur_y + 16) && (sprite.y() + size > cur_y + 16) {
                //println!("selected:{}, spritex:{}, ly:{}, i:{}", self.selected_oam.len(), sprite.x(), cur_y, i);
                if self.selected_oam.is_empty() || self.selected_oam[self.selected_oam.len() - 1].sprite().x() > sprite.x() {
                    self.selected_oam.insert(0, SelectedSprite::new(sprite, i as u8));
                    continue;
                }

                let mut c = 0;
                while c < self.selected_oam.len() {
                    //println!("c:{}", c);
                    if c + 1 == self.selected_oam.len() {
                        self.selected_oam.push(SelectedSprite::new(sprite, i as u8));
                        break;
                    }
                    if self.selected_oam[c as usize].sprite().x() > sprite.x() {
                        self.selected_oam.insert(c as usize, SelectedSprite::new(sprite, i as u8));
                        break;
                    }
                    
                    c += 1;
                }
            }
        }
        self.sprite_count = self.selected_oam.len() as u8;
    }

    pub fn pipeline_load_sprite_tile(&mut self) {
        for (i, &selected_sprite) in self.selected_oam.iter().enumerate() {
            let sprite_x = selected_sprite.sprite().x();
            let sp_x = (sprite_x - 8) + (self.lcd.scroll_x % 8);

            let inbound: bool = (sp_x >= self.pixel_fifo.fetch_x) && (sp_x < (self.pixel_fifo.fetch_x + 8));
            let inbound2: bool = ((sp_x + 8) >= self.pixel_fifo.fetch_x) && ((sp_x + 8) < self.pixel_fifo.fetch_x + 8);

            if inbound || inbound2 {
                self.fetched_sprites[self.fetched_sprite_count as usize] = selected_sprite.sprite();
                self.fetched_sprite_count += 1;
            }

            if self.fetched_sprite_count >= 3 {
                break;
            }
        }
    }

    pub fn pipeline_load_sprite_data(&mut self, offset: u8) {
        let cur_y = self.lcd.ly;
        let size  = self.lcd.lcd_control.sprite_size();

        for i in 0..self.fetched_sprite_count {
            let t_y = ((cur_y + 16) - self.fetched_sprites[i as usize].y()) * 2;
            let tile_y: u8 = if self.fetched_sprites[i as usize].y_flip() {(size * 2) - 2 - t_y} else {t_y};

            let tile = self.fetched_sprites[i as usize].tile();
            let tile_index = if size == 16 {tile & !(0x01)} else {tile};

            //println!("index:{}, ly:{}, offset:{}", tile_index, cur_y, offset);

            let data = self.read_vram(0x8000 + (tile_index as u16 * 16) + tile_y as u16 + offset as u16);
            self.pixel_fifo.fetch_sprite_data[((i * 2) + offset) as usize] = data;
            
        }
    }

    pub fn fetch_sprite_pixels(&mut self, bg_color: u8, color: &mut u32) -> (bool, u32) {
        for (i, &sprite) in self.fetched_sprites.iter().enumerate() {
            if self.fetched_sprite_count == i as u8 {
                break;
            }
            
            let sp_x = sprite.x() - 8 + (self.lcd.scroll_x % 8);

            if sp_x + 8 < self.pixel_fifo.fifo_x {
                continue;
            }

            let offset: i16 = self.pixel_fifo.fifo_x as i16 - sp_x as i16;
            if (offset < 0) || (offset > 7) {
                continue;
            }

            let bit = if sprite.x_flip() {offset} else {7 - offset};

            let hi: bool = self.pixel_fifo.fetch_sprite_data[i * 2 + 1] & (1 << bit) != 0;
            let lo: bool = self.pixel_fifo.fetch_sprite_data[i * 2] & (1 << bit) != 0;

            let mut sp_color:u8 = 0x0;
            if hi {sp_color |= 0b10;}
            if lo {sp_color |= 0b01;}

            let bg_priority = sprite.priority();

            if sp_color == 0 {
                // transparent
                continue;
            }

            if !bg_priority || bg_color == 0 {
                // false is sp2 and true is sp1
                //let palette = if sprite.dmg_palette() {false} else {true};

                let new_color = if sprite.dmg_palette() {
                    self.lcd.sp2_colors[sp_color as usize]
                } else {
                    self.lcd.sp1_colors[sp_color as usize]
                };

                if sp_color > 0 {
                    return (false, new_color);
                }
            }
        }
       (true, *color)
    }

    pub fn selected_oam_reset(&mut self) {
        self.selected_oam = ArrayVec::<_, 10>::new();
    }

    pub fn window_visible(&mut self) -> bool {
        self.lcd.lcd_control.window_enable() && (self.lcd.window_x >= 0) && (self.lcd.window_x <= 166)
            && (self.lcd.window_y >= 0) && (self.lcd.window_y < YRES as u8)
    }

    pub fn pipeline_load_window_tile(&mut self) {
        if self.window_visible() {
            let win_x = self.lcd.window_x;
            let win_y = self.lcd.window_y;

            if (self.pixel_fifo.fetch_x + 7 >= win_x)
                && (self.pixel_fifo.fetch_x + 7 < win_x + YRES as u8 + 14) && (self.lcd.ly >= win_y)
                && (self.lcd.ly < win_y + XRES as u8) {
                    
                    let win_tile_y: u8  = self.window_line / 8;
                    
                    self.pixel_fifo.bgw_fetch_data[0] = self.read_vram(self.lcd.lcd_control.window_tilemap()
                        + (self.pixel_fifo.fetch_x as u16 + 7 - win_x as u16)/8 + (win_tile_y as u16 * 32));
                    
                        if self.lcd.lcd_control.bg_window_tile_data() == 0x8800 {
                        self.pixel_fifo.bgw_fetch_data[0] = self.pixel_fifo.bgw_fetch_data[0].wrapping_add(0x80);
                    }
            } 
        }
    }

}