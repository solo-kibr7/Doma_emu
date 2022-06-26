pub use crate::mmu::{MMU, interrupts};
pub use crate::instruction::Instruction;
pub use crate::dbg::DBG;

use std::fmt;

pub mod execute;
pub mod opcodes;
//mod executor;


pub struct CPU {
    a: u8, // accumulaate /arguments
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8, // Flags
    h: u8, // addr
    l: u8, // addr
    sp: u16, // Stack Pointer
    pc: u16, // Program Counter
    halted: bool,
    ime: bool, // Interrupt Master Enable Flag
    ei: bool, // enable_interrupt_next
    ticks: usize,
}
impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{A:{:#X}, B:{:#X}, C:{:#X}, D:{:#X}, E:{:#X}, H:{:#X}, L:{:#X}}} {{ei:{} ime:{}}} \nflags: {{{}{}{}{}}} {{pc: {:#X}, sp: {:#X}, ticks:{:#X}}}",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.ei,
            self.ime,
            if self.get_flag_z() {"Z"} else {"-"},
            if self.get_flag_n() {"N"} else {"-"},
            if self.get_flag_h() {"H"} else {"-"},
            if self.get_flag_c() {"C"} else {"-"},
            self.pc,
            self.sp,
            self.ticks,
        )
    }
}
impl Default for CPU {
    fn default() -> Self {
        let mut cpu = CPU {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0, // Flags
            h: 0x01, // addr
            l: 0x4D, // addr
            sp: 0xFFFE, // Stack Pointer
            pc: 0x0100, // Program Counter
            halted: false,
            ime: false,
            ei: false,
            ticks: 0,
        };
        cpu
    }
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0, // Flags
            h: 0, // addr
            l: 0, // addr
            sp: 0, // Stack Pointer
            pc: 0, // Program Counter
            halted: false,
            ime: false,
            ei: false,
            ticks: 0,
        };
        cpu
    }

    // get flags from register f
    pub fn get_flag_z(&self) -> bool {
        self.f & 0b10000000 != 0
    }
    pub fn get_flag_n(&self) -> bool {
        self.f & 0b01000000 != 0
    }
    pub fn get_flag_h(&self) -> bool {
        self.f & 0b00100000 != 0
    }
    pub fn get_flag_c(&self) -> bool {
        self.f & 0b00010000 != 0
    }

    // if bit is false, then the flag resets
    pub fn set_flag_z(&mut self, bit: bool) {
        self.f = if bit {self.f | 0b10000000} else {self.f & 0b01111111};
    }
    pub fn set_flag_n(&mut self, bit: bool) {
        self.f = if bit {self.f | 0b01000000} else {self.f & 0b10111111};
    }
    pub fn set_flag_h(&mut self, bit: bool) {
        self.f = if bit {self.f | 0b00100000} else {self.f & 0b11011111};
    }
    pub fn set_flag_c(&mut self, bit: bool) {
        self.f = if bit {self.f | 0b00010000} else {self.f & 0b11101111};
    }

    pub fn get_a(&mut self) -> u8 {
        self.a
    }
    pub fn get_b(&mut self) -> u8 {
        self.b
    }
    pub fn get_c(&mut self) -> u8 {
        self.c
    }
    pub fn get_d(&mut self) -> u8 {
        self.d
    }
    pub fn get_e(&mut self) -> u8 {
        self.e
    }
    pub fn get_h(&mut self) -> u8 {
        self.h
    }
    pub fn get_l(&mut self) -> u8 {
        self.l
    }

    pub fn get_pc(&mut self) -> u16 {
        self.pc
    }
    pub fn get_sp(&mut self) -> u16 {
        self.sp
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }
    pub fn set_sp(&mut self, sp: u16) {
        self.sp = sp;
    }

    pub fn get_af(&mut self) -> u16 {
        let a: u16 = (self.a as u16) << 8;
        a | (self.f as u16)
    }
    pub fn get_bc(&mut self) -> u16 {
        let b: u16 = (self.b as u16) << 8;
        b | (self.c as u16)
    }
    pub fn get_de(&mut self) -> u16 {
        let d: u16 = (self.d as u16) << 8;
        d | (self.e as u16)
    }
    pub fn get_hl(&mut self) -> u16 {
        let h: u16 = (self.h as u16) << 8;
        h | (self.l as u16)
    }

    pub fn set_hl(&mut self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = (hl & 0x00FF) as u8;
    }

    pub fn get_halted(&mut self) -> bool {
        self.halted
    }
    pub fn set_halted(&mut self, b: bool){
        self.halted = b;
    }

    pub fn set_ime(&mut self, b: bool){
        self.ime = b;
    }

    pub fn get_ticks(&mut self) -> usize {
        self.ticks
    }

    pub fn alu_add(&mut self, n: &u8, carry: bool) -> u8 {
        let c = if carry && self.get_flag_c() {1} else {0};
        let result = self.a.wrapping_add(*n).wrapping_add(c);
        self.set_flag_z(result == 0);
        self.set_flag_h((self.a & 0x0F) + (*n & 0x0F) + c > 0xF);
        self.set_flag_n(false);
        self.set_flag_c((self.a as u16) + (*n as u16) + (c as u16) > 0xFF);
        result
    }
    pub fn alu_sub(&mut self, n: &u8, carry: bool) -> u8 {
        let c = if carry && self.get_flag_c() {1} else {0};
        let result = self.a.wrapping_sub(*n).wrapping_sub(c);
        self.set_flag_z(result == 0);
        self.set_flag_h((self.a & 0x0F) < (*n & 0x0F) + c);
        self.set_flag_n(true);
        self.set_flag_c((self.a as u16) < (*n as u16) + (c as u16));
        //println!("Print conditions: {:?} and result: {:#X}", carry && self.get_flag_c(), result);
        result
    }
    pub fn swap(&mut self, n: &u8) -> u8 {
        self.set_flag_z(*n == 0);
        self.set_flag_h(false);
        self.set_flag_n(false);
        self.set_flag_c(false);
        (n << 4) | (n >> 4)
    }

    pub fn push_stack(&mut self, mmu: &mut MMU) {
        let addr1: u8 = ((self.pc & 0xFF00) >> 8) as u8;
        let addr2: u8 = (self.pc & 0x00FF) as u8;
        self.sp = self.sp.wrapping_sub(1);
        mmu.write_byte(self.sp, addr1);
        self.sp = self.sp.wrapping_sub(1);
        mmu.write_byte(self.sp, addr2);
    }

    pub fn pop_stack(&mut self, mmu:&mut MMU) -> u16 {
        let low = mmu.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let high = mmu.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        (high << 8) | low
    }

    // decode instructions
    pub fn decode(&mut self, byte: u8, mmu: &MMU) -> Instruction {
        let n1 = mmu.read_byte(self.pc + 1) as u16;
        let n2 = mmu.read_byte(self.pc + 2) as u16;
        // d16 is nn
        let d16: u16 = (n2 << 8) | n1;
        //println!("d16: {:#X} or n1 {:#X} and n2 {:#X}", d16, n1, n2);

        opcodes::decoder(self, mmu, byte, n1, d16)
    }

    pub fn cb_decode(&mut self, byte: u8, mmu: &MMU) -> Instruction {
        let n1 = mmu.read_byte(self.pc + 2) as u16;
        let n2 = mmu.read_byte(self.pc + 3) as u16;
        let d16: u16 = (n2 << 8) | n1;

        opcodes::cb_decoder(self, byte, mmu)
    }

    pub fn excute(&mut self, instruction: &Instruction, mmu: &mut MMU) -> u8{
        execute::executer(self, instruction, mmu)
    }

    pub fn run_instruction(&mut self, mmu: &mut MMU) -> u8 {
        let byte = mmu.read_byte(self.pc);

        

        let instruction = self.decode(byte, mmu);
        //190000
        /* if self.ticks > 0x1D4000{

            println!("{:?} {:?} ({:#X} {:#X}) \n", self, instruction, byte, mmu.read_byte(self.pc + 1));
            /* println!(
                "Counter:{:#X} IE:{:#X} IF:{:#X} Interrupt:{} \n", 
                mmu.read_byte(0xFF04), mmu.read_byte(0xFFFF), mmu.read_byte(0xFF0F), 
                mmu.interrupts.peek_highest_interrupt().is_some()); */
        } */

        /*println!("Current byte {:#x} | pc: {:#X}", byte as u8, self.pc);

        println!("Current instruction {:?} \n ", instruction);

        println!("Current instruction {:?} \n ", self);*/

        self.excute(&instruction, mmu)
    }

    pub fn do_cycle(&mut self, mmu: &mut MMU, dbg: &mut DBG) {
        if self.halted {
            self.cycle(mmu, 1);
            if mmu.interrupts.read_requested() != 0 {
                self.halted = false;
            }
        } else {
            dbg.dbg_update(mmu);
            dbg.dbg_print();
            let m = self.run_instruction(mmu);
            self.cycle(mmu, m);
        }
        if self.ime && mmu.interrupts.peek_highest_interrupt().is_some() {
            self.interrupt_handle(mmu);
        }
        if self.ei {
            self.ime = true;
            self.ei = false;
        }
        
    }
    pub fn cycle(&mut self, mmu: &mut MMU, cycles: u8) {
        let n = cycles * 4;
        for x in 0..n {
            self.ticks += 1;
            mmu.timer.ticks(&mut mmu.interrupts);
        }
    }
    pub fn interrupt_handle(&mut self, mmu:&mut MMU) {
        // Push PC part 1
        // trigger write oam bug because of the increment
        
        //bus.trigger_write_oam_bug(self.reg_sp);
        let pc = self.pc;

        self.sp = self.sp.wrapping_sub(1);
        //cpu.set_sp(sp);
        mmu.write_byte(self.sp, (pc >> 8) as u8);


        if mmu.interrupts.peek_highest_interrupt().is_some() {
            let highest_interrupt = mmu.interrupts.get_highest_interrupt(self).unwrap();
            self.pc = mmu.interrupts.interrupt_addresses(highest_interrupt);
        } else {
            // Interrupt cancelled. Why would this happen??
            self.pc = 0;
        }
        
        // Push PC part 2
        self.sp = self.sp.wrapping_sub(1);
        mmu.write_byte(self.sp, pc as u8);
    }
}