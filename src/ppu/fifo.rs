pub use fixed_vec_deque::FixedVecDeque;

pub enum FetchState {
    TileNum,
    DataLow,
    DataHigh,
    Sleep,
    Push,
}

#[derive(Default, Copy, Clone)]
pub struct BgFifoPixel {
    color: u32,
    bg_priority: bool,
}

impl BgFifoPixel {
    pub fn get_color(&self) -> u32 {
        self.color
    }
    pub fn bg_priority(&self) -> bool {
        self.bg_priority
    }
}

pub struct BgFifo {
    pixels: FixedVecDeque<[BgFifoPixel; 16]>,
}

impl Default for BgFifo {
    fn default() -> Self {
        Self {
            pixels: FixedVecDeque::new(),
        }
    }
}

impl BgFifo {
    pub fn push(&mut self, color: u32, bg_priority: bool) {
        *self.pixels.push_back() = BgFifoPixel {
            color,
            bg_priority,
        }
    }
    pub fn pop(&mut self) -> BgFifoPixel {
        *self.pixels.pop_front().unwrap()
    }
    pub fn length(&self) -> usize {
        self.pixels.len()
    }
}

#[derive(Default, Copy, Clone)]
pub struct SpFifoPixel {
    color: u32,
    bg_priority: bool,
    palette: bool,
}
impl SpFifoPixel {
    pub fn get_color(&self) -> u32 {
        self.color
    }
    pub fn get_palette(&self) -> bool {
        self.palette
    }
}

pub struct SpFifo {
    pixels: FixedVecDeque<[SpFifoPixel; 16]>,
}

impl Default for SpFifo {
    fn default() -> Self {
        Self {
            pixels: FixedVecDeque::new(),
        }
    }
}

impl SpFifo {
    pub fn push(&mut self, color: u32, bg_priority: bool, palette: bool) {
        *self.pixels.push_back() = SpFifoPixel {
            color,
            bg_priority,
            palette,
        }
    }
    pub fn pop(&mut self) -> SpFifoPixel {
        *self.pixels.pop_front().unwrap()
    }
    pub fn length(&self) -> usize {
        self.pixels.len()
    }
}

pub struct PixelFifo {
    pub(super) bgfifo: BgFifo,
    pub(super) spfifo: SpFifo,
    pub(super) current_state: FetchState,
    pub(super) line_x: u8,
    pub(super) push_x: u8,
    pub(super) fetch_x: u8,
    pub(super) bgw_fetch_data: [u8; 3],
    pub(super) fetch_sprite_data: [u8; 6], //oam data..
    pub(super) sprite_count: u8,
    pub(super) map_x: u8,
    pub(super) map_y: u8,
    pub(super) tile_x: u8,
    pub(super) tile_y: u8,
    pub(super) fifo_x: u8,
}

impl Default for PixelFifo {
    fn default() -> Self {
        Self {
            bgfifo: BgFifo::default(),
            spfifo: SpFifo::default(),
            current_state: FetchState::TileNum,
            line_x: 0,
            push_x: 0,
            fetch_x: 0,
            bgw_fetch_data: [0; 3],
            fetch_sprite_data: [0; 6], //oam data..
            sprite_count: 0,
            map_x: 0,
            map_y: 0,
            tile_x: 0,
            tile_y: 0,
            fifo_x: 0,
        }
    }
}

