pub use crate::cpu::CPU;
pub use crate::mmu::MMU;
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
            // why 1 and not 0???
            requested: InterruptFlags::from_bits_truncate(0),
        }
    }
}

impl Interrupts {
    pub fn write_enabled(&mut self, value: u8) {
        self.enabled.clone_from(&InterruptFlags::from_bits_truncate(value));
    }
    pub fn write_requested(&mut self, value: u8) {
        self.requested.clone_from(&InterruptFlags::from_bits_truncate(value));
    }

    pub fn read_enable(&self) -> u8 {
        self.enabled.bits()
    }
    pub fn read_requested(&self) -> u8 {
        self.requested.bits()
    }

    pub fn int_check_remove(&mut self, interrupt: InterruptType, cpu:&mut CPU) -> bool {
        let it:InterruptFlags = interrupt.into();
        if self.requested.bits() & it.bits() != 0 && self.enabled.bits() & it.bits() != 0 {
            self.write_requested(self.requested.bits() & !it.bits());
            cpu.set_halted(false);
            cpu.set_ime(false);
            return true;
        }
        false
    }
    pub fn int_check(&mut self, interrupt: InterruptType) -> bool {
        let it:InterruptFlags = interrupt.into();
        //println!("if:{:#X}, ie:{:#X}, it:{:#X} b:{}", self.requested.bits(), self.enabled.bits(), it.bits(), self.requested.bits() & it.bits() != 0);
        if self.requested.bits() & it.bits() != 0 && self.enabled.bits() & it.bits() != 0 {
            return true;
        }
        false
    }

    pub fn get_highest_interrupt(&mut self, cpu:&mut CPU) -> Option<InterruptType> {
        let mut count = 0;
        while count < 5 {
            if self.int_check_remove(InterruptType::try_from(count).unwrap(), cpu) {
                return Some(InterruptType::try_from(count).unwrap());
            }
            count += 1;
        }
        None
    }

    pub fn peek_highest_interrupt(&mut self) -> Option<InterruptType> {
        let mut count = 0;
        while count < 5 {
            //println!("int check: {}", self.int_check(InterruptType::try_from(count).unwrap()));
            if self.int_check(InterruptType::try_from(count).unwrap()) {
                return Some(InterruptType::try_from(count).unwrap());
            }
            count += 1;
        }
        None
    }

    pub fn interrupt_addresses(&mut self, int_type: InterruptType) -> u16 {
        match int_type {
            InterruptType::Vblank => 0x40,
            InterruptType::LcdStat => 0x48,
            InterruptType::Timer => 0x50,
            InterruptType::Serial => 0x58,
            InterruptType::Joypad => 0x60,
        }
    }
    /*
    pub fn interrupt_handle(&mut self, cpu:&mut CPU, mmu:&mut MMU) {
            let pc = cpu.get_pc();
            let mut sp = cpu.get_sp();

            // Push PC part 1
            // trigger write oam bug because of the increment
            
            //bus.trigger_write_oam_bug(self.reg_sp);

            sp = sp.wrapping_sub(1);
            cpu.set_sp(sp);
            mmu.write_byte(cpu.get_sp(), (pc >> 8) as u8);

            /*if let Some(int_type) = bus.take_next_interrupt() {
                cpu_state = CpuState::RunningInterrupt(int_type);
                self.reg_pc = INTERRUPTS_VECTOR[int_type as usize];
            } else {
                // Interrupt cancelled
                self.reg_pc = 0;
            }*/
            if self.get_highest_interrupt(cpu).is_some() {
                let highest_interrupt = self.get_highest_interrupt(cpu).unwrap();
                cpu.set_pc(self.interrupt_addresses(highest_interrupt));
            } else {
                // why would this happen???
                cpu.set_pc(0);
            }
            
            // Push PC part 2
            sp = sp.wrapping_sub(1);
            cpu.set_sp(sp);
            mmu.write_byte(cpu.get_sp(), pc as u8);
    }*/
    pub fn request_interrupt(&mut self, interrupt: InterruptType) {
        self.requested.insert(interrupt.into());
    }
}