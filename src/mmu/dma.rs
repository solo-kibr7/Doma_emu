pub use crate::mmu::MMU;

#[derive(Default)]
pub struct OamDma {
    pub(super) address_byte: u8,
    pub(super) value: u8,
    pub(super) start_delay: u8,
    pub(super) in_transfer: bool,
}

impl OamDma {
    pub fn dma_start(&mut self, start: u8) {
        //println!("start");
        self.address_byte = 0;
        self.start_delay = 2;
        self.in_transfer = true;
        self.value = start;
    }


    pub fn read_register(&self) -> u8 {
        self.address_byte
    }
}