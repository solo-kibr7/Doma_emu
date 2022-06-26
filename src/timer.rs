use bitflags::bitflags;
pub use crate::mmu::interrupts::{Interrupts, InterruptType}; 

bitflags! {
    struct TimerControl: u8 {
        const TIMER_ENABLE = 1 << 2;
        const FREQ_DIVIDER = 0b11;
        const UNUSED = 0b111 << 5;
    }
}
impl TimerControl {
    fn timer_enabled(&self) -> bool {
        self.intersects(Self::TIMER_ENABLE)
    }

    fn freq_divider_bit(&self) -> u16 {
        match self.bits() & Self::FREQ_DIVIDER.bits {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => unreachable!(),
        }
    }
}

pub struct Timer {
    divider: u16,
    counter: u8,
    modulo: u8,
    control: TimerControl,
    interrupt_next: bool,
    during_interrupt: bool,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            // why???
            // 0x0008
            divider: 0xABCC, // divider value if boot_rom is present
            counter: 0,
            modulo: 0,
            control: TimerControl::from_bits_truncate(0),
            interrupt_next: false,
            during_interrupt: false,
        }
    }
}

impl Timer {
    pub fn timer_read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.divider >> 8) as u8,
            0xFF05 => self.counter,
            0xFF06 => self.modulo,
            0xFF07 => self.control.bits(),
            _ => panic!("Unsupported address {:#X}. How did this happen lol!", address)
        }
    }
    /*pub fn timer_write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.divider = 0,
            0xFF05 => self.counter = value,
            0xFF06 => self.modulo = value,
            0xFF07 => self.control = TimerControl::from_bits_truncate(value),
            _ => panic!("Unsupported address {:#X}. How did this happen lol!", address)
        };
    }*/
    pub fn write_div(&mut self) {
        let old_divider_bit = self.divider_bit();
        self.divider = 0; // reset
        let new_divider_bit = self.divider_bit();

        if old_divider_bit && !new_divider_bit {
            self.increment_timer();
        }
    }
    pub fn write_counter(&mut self, data: u8) {
        // ignore timer reload and interrupt if there is an interrupt_next
        self.interrupt_next = false;

        // in the case this is the timer counter(TIMA) is reloaded
        // (and interrupt is triggered), then reload from the (TMA)
        // and ignore `data`
        self.counter = if self.during_interrupt {
            self.modulo
        } else {
            data
        };
    }
    pub fn write_modulo(&mut self, data: u8) {
        self.modulo = data;

        // if TMA is written during the same cycle it is reloaded into
        // the timer counter (TIMA), then reload TIMA as well
        if self.during_interrupt {
            self.counter = self.modulo;
        }
    }
    pub fn write_control(&mut self, data: u8) {
        let old_enable = self.control.timer_enabled();
        let old_divider_bit = old_enable && self.divider_bit();

        self.control
            .clone_from(&TimerControl::from_bits_truncate(data));

        let new_enable = self.control.timer_enabled();
        let new_divider_bit = new_enable && self.divider_bit();

        if old_divider_bit && !new_divider_bit {
            self.increment_timer();
        }
    }
    pub fn ticks(&mut self, interrupt: &mut Interrupts) {
        self.during_interrupt = false;

        if self.interrupt_next {
            interrupt.request_interrupt(InterruptType::Timer);
            self.interrupt_next = false;
            self.counter = self.modulo;
            self.during_interrupt = true;
        }

        let old_divider_bit = self.divider_bit();

        // because each CPU M-cycle is 4 T-cycles
        self.divider = self.divider.wrapping_add(1);

        let new_divider_bit = self.divider_bit();

        if self.control.timer_enabled() && old_divider_bit && !new_divider_bit {
            self.increment_timer();
        }
    }
    fn increment_timer(&mut self) {
        self.counter = self.counter.wrapping_add(1);
        self.interrupt_next = self.counter == 0;
    }
    fn divider_bit(&self) -> bool {
        let bit = self.control.freq_divider_bit();
        (self.divider >> bit) & 1 == 1
    }
}