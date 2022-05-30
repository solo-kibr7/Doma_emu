pub use crate::cpu::CPU;
use bitflags::bitflags;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InterruptType {
    Vblank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}
impl TryFrom<u8> for InterruptType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Vblank),
            1 => Ok(Self::LcdStat),
            2 => Ok(Self::Timer),
            3 => Ok(Self::Serial),
            4 => Ok(Self::Joypad),
            _ => Err(()),
        }
    }
}

bitflags! {
    struct InterruptFlags: u8 {
        const VBLANK   = 1 << 0;
        const LCD_STAT = 1 << 1;
        const TIMER    = 1 << 2;
        const SERIAL   = 1 << 3;
        const JOYPAD   = 1 << 4;
        const UNUSED   = 0b111 << 5;
    }
}

impl From<InterruptType> for InterruptFlags {
    fn from(interrupt: InterruptType) -> Self {
        match interrupt {
            InterruptType::Vblank => Self::VBLANK,
            InterruptType::LcdStat => Self::LCD_STAT,
            InterruptType::Timer => Self::TIMER,
            InterruptType::Serial => Self::SERIAL,
            InterruptType::Joypad => Self::JOYPAD,
        }
    }
}

pub struct Interrupts {
    enabled: InterruptFlags,
    requested: InterruptFlags,
}
impl Default for Interrupts {
    fn default() -> Self {
        Self {
            enabled: InterruptFlags::from_bits_truncate(0),
            requested: InterruptFlags::from_bits_truncate(0),
        }
    }
}

impl Interrupts {
    pub fn int_check(&mut self, address: u16, interrupt: InterruptType, cpu:&mut CPU) {
        let it: InterruptFlags = interrupt.into();
        if self.requested.bits() & it.bits() != 0 && self.enabled.bits() & it.bits() != 0 {
            self.write_requested(self.requested.bits() & !it.bits());
            cpu.set_halted(false);
            cpu.set_ime(false);
    
        }
    }
    pub fn write_enabled(&mut self, value: u8) {
        self.enabled = InterruptFlags::from_bits_truncate(value);
    }
    pub fn write_requested(&mut self, value: u8) {
        self.requested = InterruptFlags::from_bits_truncate(value);
    }

    pub fn read_interrupt_enable(&self) -> u8 {
        self.enabled.bits()
    }
    pub fn read_interrupt_requested(&self) -> u8 {
        self.requested.bits()
    }
    
}