use bitflags::bitflags;

bitflags! {
    struct SerialControl: u8 {
        const SHIFT_CLOCK = 1 << 0;
        const CLOCK_SPEED = 1 << 1;
        //Transfer Start Flag
        const START_FLAG = 1 << 7;
    }
}

pub struct Serial {
    data: u8,
    control: SerialControl,
}

impl Serial {
    pub fn new() -> Serial {
        Serial { 
            data: 0, 
            control: SerialControl::from_bits_truncate(0),
        }
    }
    pub fn get_data(&self) -> u8 {
        self.data
    }
    pub fn get_control(&self) -> u8 {
        self.control.bits()
    }
    pub fn write_data(&mut self, value: u8) {
        self.data = value;
    }
    pub fn write_control(&mut self, value: u8) {
        self.control = SerialControl::from_bits_truncate(value);
    }
}