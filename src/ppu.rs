use crate::mmu::MMU;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

pub struct PPU {
    mode: u8,
    mode_clock: usize,
    background_buffer: Vec<u32>,
    viewport: Vec<u32>,
}