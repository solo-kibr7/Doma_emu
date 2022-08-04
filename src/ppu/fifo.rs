pub use fixed_vec_deque::FixedVecDeque;

pub enum FetchState {
    Tile,
    DataLow,
    DataHigh,
    Sleep,
    Push,
}

#[derive(Default, Copy, Clone)]
pub struct BgFifoPixel {
    color: u8,
    bg_priority: bool,
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
    pub fn push(&mut self, color: u8, bg_priority: bool) {
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

#[derive(Default)]
pub struct SpFifoPixel {
    color: u8,
    bg_priority: bool,
}

pub struct SpFifo {
    pixels: FixedVecDeque<[BgFifoPixel; 16]>,
}

impl Default for SpFifo {
    fn default() -> Self {
        Self {
            pixels: FixedVecDeque::new(),
        }
    }
}

pub struct PixelFifo {
    bgfifo: BgFifo,
    spfifo: SpFifo,
    fetch_state: FetchState,
    line_x: u8,
    push_x: u8,
    fetch_x: u8,
    /* u8 bgw_fetch_data[3];
    u8 fetch_entry_data[6]; //oam data.. */
    map_y: u8,
    map_x: u8,
    tile_y: u8,
    fifo_x: u8,
}

impl Default for PixelFifo {
    fn default() -> Self {
        Self {
            bgfifo: BgFifo::default(),
            spfifo: SpFifo::default(),
            fetch_state: FetchState::Tile,
            line_x: 0,
            push_x: 0,
            fetch_x: 0,
            /* u8 bgw_fetch_data[3];
            u8 fetch_entry_data[6]; //oam data.. */
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
        }
    }
}

