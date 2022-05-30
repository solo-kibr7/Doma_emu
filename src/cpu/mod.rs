pub use crate::mmu::MMU;
pub use crate::instruction::Instruction;

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
}
impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU {{ A: {:#X}, B: {:#X}, C: {:#X}, D: {:#X}, E: {:#X}, H: {:#X}, L: {:#X} }} \nflags: {{ Z: {:?}, N: {:?}, H: {:?}, C: {:?} }}\n{{ pc: {:#X}, sp: {:#X} }}",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.get_flag_z(),
            self.get_flag_n(),
            self.get_flag_h(),
            self.get_flag_c(),
            self.pc,
            self.sp,
        )
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
        result
    }
    pub fn swap(&mut self, n: &u8) -> u8 {
        self.set_flag_z(*n == 0);
        self.set_flag_h(false);
        self.set_flag_n(false);
        self.set_flag_c(false);
        (n << 4) | (n >> 4)
    }

    // decode instructions
    pub fn decode(&mut self, byte: u8, mmu: &MMU) -> Instruction {
        let n1 = mmu.read_byte(self.pc + 1) as u16;
        let n2 = mmu.read_byte(self.pc + 2) as u16;
        // d16 is nn
        let d16: u16 = (n2 << 8) | n1;

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

    pub fn run_instruction(&mut self, mmu: &mut MMU) {
        let byte = mmu.read_byte(self.pc);

        println!("Current byte {:#x} | pc: {:#X}", byte as u8, self.pc);

        let instruction = self.decode(byte, mmu);

        println!("Current instruction {:?} \n ", instruction);

        self.excute(&instruction, mmu);
    }

    pub fn do_cycle(&mut self, mmu: &mut MMU) {

    }
}