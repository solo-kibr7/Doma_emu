pub use crate::mmu::MMU;
pub use crate::instruction::Instruction;
pub use crate::decoder::Decoder;
use std::fmt;

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
    t: usize,
    m: usize,
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
            t: 0,
            m: 0,
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

        Decoder::decoder(self, mmu, byte, n1, d16)
    }

    pub fn cb_decode(&mut self, byte: u8, mmu: &MMU) -> Instruction {
        let n1 = mmu.read_byte(self.pc + 2) as u16;
        let n2 = mmu.read_byte(self.pc + 3) as u16;
        let d16: u16 = (n2 << 8) | n1;

        Decoder::cb_decoder(self, byte, mmu)
    }

    pub fn excute(&mut self, instruction: &Instruction, mmu: &mut MMU) {
        match instruction {
            Instruction::LdB(n) => {
                self.b = *n;
                self.pc += 2;
            },
            Instruction::LdC(n) => {
                self.c = *n;
                self.pc += 2;
            },
            Instruction::LdD(n) => {
                self.d = *n;
                self.pc += 2;
            },
            Instruction::LdE(n) => {
                self.e = *n;
                self.pc += 2;
            },
            Instruction::LdH(n) => {
                self.h = *n;
                self.pc += 2;
            },
            Instruction::LdL(n) => {
                self.l = *n;
                self.pc += 2;
            },

            Instruction::LdAR2(r2) => {
                self.a = *r2 as u8;
                self.pc += 1;
            },
            Instruction::LdAnn(r2) => {
                self.a = *r2 as u8;
                self.pc += 3;
            },
            Instruction::LdAd8(r2) => {
                self.a = *r2 as u8;
                self.pc += 2;
            },
            Instruction::LdBR2(r2) => {
                self.b = *r2 as u8;
                self.pc += 1;
            },
            Instruction::LdCR2(r2) => {
                self.c = *r2 as u8;
                self.pc += 1;
            },
            Instruction::LdDR2(r2) => {
                self.d = *r2 as u8;
                self.pc += 1;
            },
            Instruction::LdER2(r2) => {
                self.e = *r2 as u8;
                self.pc += 1;
            },
            Instruction::LdHR2(r2) => {
                self.h = *r2 as u8;
                self.pc += 1;
            },
            Instruction::LdLR2(r2) => {
                self.l = *r2 as u8;
                self.pc += 1;
            },

            Instruction::LdnA(n, a) => {
                match n {
                    0 => {self.b = *a;},
                    1 => {self.c = *a;},
                    2 => {self.d = *a;},
                    3 => {self.e = *a;},
                    4 => {self.h = *a;},
                    5 => {self.l = *a;},
                    _ => panic!(
                        "Can't find {:?} for instruction {:#?}",
                        n, instruction,
                    ),
                }
                self.pc += 1;
            },
            Instruction::Ldn16A(n, a) => {
                mmu.write_byte(*n, *a);
                self.pc += 1;
            },
            Instruction::Lda16A(n, a) => {
                mmu.write_byte(*n, *a);
                self.pc += 3;
            },
            Instruction::LdAc(c) => {
                self.a = mmu.read_byte(0xFF00 | *c as u16);
                self.pc += 2;
            },
            Instruction::LdCa(a) => {
                self.c = mmu.read_byte(0xFF00 | *a as u16);
                self.pc += 2;
            },
            
            // put LddAhl
            

            Instruction::LddAHl(hl16) => {
                self.a = mmu.read_byte(*hl16);
                let mut hl = *hl16 as u16;
                hl = hl.wrapping_sub(1);
                self.set_hl(hl);
                self.pc += 1;
            },
            Instruction::LddHlA(hl16) => {
                let mut hl = *hl16 as u16;
                mmu.write_byte(hl, self.a);
                hl = hl.wrapping_sub(1);
                self.set_hl(hl);
                self.pc += 1;
                // maybe implement DECHl here

                //println!("CPU STATE: {:?}", self);
            },
            Instruction::LdIAHl(hl16) => {
                self.a = mmu.read_byte(*hl16);
                let mut hl = *hl16 as u16;
                hl = hl.wrapping_add(1);
                self.set_hl(hl);
                self.pc += 1;
            },
            Instruction::LdIHlA(hl16) => {
                let mut hl = *hl16 as u16;
                mmu.write_byte(hl, self.a);
                hl = hl.wrapping_add(1);
                self.set_hl(hl);
                self.pc += 1;
            },



            Instruction::LdHnA(n) => {
                mmu.write_byte(0xFF00 + *n as u16, self.a);
                self.pc += 2;
            },
            Instruction::LdHAn(n) => {
                self.a = mmu.read_byte(0xFF00 + *n as u16);
                self.pc += 2;
            },


            Instruction::LdBc(d16) => {
                self.b = (d16 >> 8) as u8;
                self.c = (d16 & 0x00FF) as u8;
                self.pc += 3;
            },
            Instruction::LdDe(d16) => {
                self.d = (d16 >> 8) as u8;
                self.e = (d16 & 0x00FF) as u8;
                self.pc += 3;
            },
            Instruction::LdHl(d16) => {
                self.h = (d16 >> 8) as u8;
                self.l = (d16 & 0x00FF) as u8;
                self.pc += 3;
            },
            Instruction::LdSp(d16) => {
                self.sp = *d16;
                self.pc += 3;
            },
            Instruction::LdSpHl(hl) => {
                self.sp = *hl;
                self.pc += 1;
            },
            Instruction::LdHlSp(s8) => {

                let sp = self.sp as i16;
                let result = if *s8 >= 0 {sp.wrapping_add(*s8 as i16)} else {sp.wrapping_sub(s8.abs() as i16)};
                if *s8 >= 0 {
                    self.set_flag_c(((self.sp & 0xFF) as i32 + *s8 as i32) as u16 > 0xFF);
                    self.set_flag_h((self.sp & 0xF) + (*s8 as u8 & 0xF) as u16 > 0xF);
                } else {
                    self.set_flag_c((result as u16 & 0xFF) <= self.sp & 0xFF);
                    self.set_flag_h((result as u16 & 0xF) <= self.sp & 0xF);
                }
                self.set_hl(result as u16);
                self.pc += 2;
            },
            Instruction::LdnnSp(d16) => {
                mmu.write_byte(*d16, (self.sp & 0x00FF) as u8);
                mmu.write_byte(d16.wrapping_add(1), (self.sp >> 8) as u8);
            },

            Instruction::Pushnn(n16) => {
                let addr1: u8 = ((n16 & 0xFF00) >> 8) as u8;
                let addr2: u8 = (n16 & 0x00FF) as u8;
                self.sp = self.sp.wrapping_sub(1);
                mmu.write_byte(self.sp, addr1);
                self.sp = self.sp.wrapping_sub(1);
                mmu.write_byte(self.sp, addr2);
                self.pc += 1;
            },
            Instruction::Popnn(n) => {
                let value1 = mmu.read_byte(self.sp);
                self.sp = self.sp.wrapping_add(1);
                let value2 = mmu.read_byte(self.sp);
                self.sp = self.sp.wrapping_add(1);
                match n {
                    0 => {self.a = value2; self.f = value1;},
                    1 => {self.b = value2; self.c = value1;},
                    2 => {self.d = value2; self.e = value1;},
                    3 => {self.h = value2; self.l = value1;},
                    _ => panic!(
                        "Can't find {:?} for instruction {:#?}",
                        n, instruction,
                    ),
                }
                self.pc += 1;
            },

            // 8bit alu
            Instruction::AddN(n) => {
                self.a = self.alu_add(n, false);
                self.pc += 1;
            },
            Instruction::AddHl(n) => {
                self.a = self.alu_add(n, false);
                self.pc += 1;
            },
            Instruction::AddD8(n) => {
                self.a = self.alu_add(n, false);
                self.pc += 2;
            },
            Instruction::AdcN(n) => {
                self.a = self.alu_add(n, true);
                self.pc += 1;
            },
            Instruction::AdcHl(n) => {
                self.a = self.alu_add(n, true);
                self.pc += 1;
            },
            Instruction::AdcD8(n) => {
                self.a = self.alu_add(n, true);
                self.pc += 2;
            },

            Instruction::SubN(n) => {
                self.a = self.alu_sub(n, false);
                self.pc += 1;
            },
            Instruction::SubHl(n) => {
                self.a = self.alu_sub(n, false);
                self.pc += 1;
            },
            Instruction::SubD8(n) => {
                self.a = self.alu_sub(n, false);
                self.pc += 2;
            },

            Instruction::SbcN(n) => {
                self.a = self.alu_sub(n, true);
                self.pc += 1;
            },
            Instruction::SbcHl(n) => {
                self.a = self.alu_sub(n, true);
                self.pc += 1;
            },
            Instruction::SbcD8(n) => {
                self.a = self.alu_sub(n, true);
                self.pc += 2;
            },
            Instruction::Andn(n) => {
                let result = self.a & *n;
                self.a = result;
                self.set_flag_z(result == 0);
                self.set_flag_n(false);
                self.set_flag_h(true);
                self.set_flag_c(false);
                self.pc += 1;
            },
            Instruction::AndHl(n) => {
                let result = self.a & *n;
                self.a = result;
                self.set_flag_z(result == 0);
                self.set_flag_n(false);
                self.set_flag_h(true);
                self.set_flag_c(false);
                self.pc += 1;
            },
            Instruction::AndD8(n) => {
                let result = self.a & *n;
                self.a = result;
                self.set_flag_z(result == 0);
                self.set_flag_n(false);
                self.set_flag_h(true);
                self.set_flag_c(false);
                self.pc += 2;
            },
            Instruction::OrN(n) => {
                let result = self.a | *n;
                self.a = result;
                self.set_flag_z(result == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(false);
                // hl has more cycles
                self.pc += 1;
            },
            Instruction::OrHl(n) => {
                let result = self.a | *n;
                self.a = result;
                self.set_flag_z(result == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(false);
                // hl has more cycles
                self.pc += 1;
            },
            Instruction::OrD8(n) => {
                let result = self.a | *n;
                self.a = result;
                self.set_flag_z(result == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(false);
                // hl has more cycles
                self.pc += 2;
            },

            Instruction::Xor(r) => {
                let result = self.a ^ *r;
                self.a = result;
                self.set_flag_z(result == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(false);
                self.pc += 1;
            },
            Instruction::XorHl(r) => {
                let result = self.a ^ *r;
                self.a = result;
                self.set_flag_z(result == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(false);
                self.pc += 1;
            },
            Instruction::XorD8(n) => {
                let result = self.a ^ *n;
                self.a = result;
                self.set_flag_z(result == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(false);
                self.pc += 2;
            },

            Instruction::Cp(n) => {
                self.alu_sub(n, false);
                self.pc += 1;
            },
            Instruction::CpHl(n) => {
                self.alu_sub(n, false);
                self.pc += 1;
            },
            Instruction::CpD8(n) => {
                self.alu_sub(n, false);
                self.pc += 2;
            },

            Instruction::IncN(n) => {
                let r: u8;
                match n {
                    0 => {r = self.a.wrapping_add(1); self.a = r;},
                    1 => {r = self.b.wrapping_add(1); self.b = r;},
                    2 => {r = self.c.wrapping_add(1); self.c = r;},
                    3 => {r = self.d.wrapping_add(1); self.d = r;},
                    4 => {r = self.e.wrapping_add(1); self.e = r;},
                    5 => {r = self.h.wrapping_add(1); self.h = r;},
                    6 => {r = self.l.wrapping_add(1); self.l = r;},
                    _ => panic!(
                        "Can't find {:?} for instruction {:#?}",
                        n, instruction,
                    ),
                }
                self.set_flag_z(r == 0);
                self.set_flag_n(false);
                self.set_flag_h((n & 0x0F) + 1 > 0x0F);
                //H - Set if carry from bit 3.
                self.pc += 1;
            },
            Instruction::IncHl(hl) => {
                let n = mmu.read_byte(*hl);
                let r = n.wrapping_add(1);
                mmu.write_byte(*hl, r);
                
                self.set_flag_z(r == 0);
                self.set_flag_n(false);
                self.set_flag_h((n & 0x0F) + 1 > 0x0F);

                self.pc += 1;
            },
            Instruction::DecN(v, n) => {
                let r: u8;
                match v {
                    0 => {r = self.a.wrapping_sub(1); self.a = r;},
                    1 => {r = self.b.wrapping_sub(1); self.b = r;},
                    2 => {r = self.c.wrapping_sub(1); self.c = r;},
                    3 => {r = self.d.wrapping_sub(1); self.d = r;},
                    4 => {r = self.e.wrapping_sub(1); self.e = r;},
                    5 => {r = self.h.wrapping_sub(1); self.h = r;},
                    6 => {r = self.l.wrapping_sub(1); self.l = r;},
                    _ => panic!(
                        "Can't find {:?} for instruction {:#?}",
                        n, instruction,
                    ),
                }
                self.set_flag_z(r == 0);
                self.set_flag_n(true);
                self.set_flag_h((n & 0x0F) == 0);

                self.pc += 1;
            },
            // DEC Hl
            Instruction::DecHl(hl) => {
                let n = mmu.read_byte(*hl);
                let r = n.wrapping_sub(1);
                mmu.write_byte(*hl, r);
                
                self.set_flag_z(r == 0);
                self.set_flag_n(true);
                self.set_flag_h((n & 0x0F) == 0);

                self.pc += 1;
            },

            // 16bit arithmetic

            Instruction::AddHlN(nn) => {
                let hl = self.get_hl();
                let r = hl.wrapping_add(*nn);
                self.set_flag_h((hl & 0x07FF) + (hl & 0x07FF) > 0x07FF);
                self.set_flag_n(false);
                self.set_flag_c(hl > 0xFFFF - nn);

                self.set_hl(r);
                self.pc += 1;
            },

            Instruction::IncNN(n, nn) => {
                let r = nn.wrapping_add(1);
                match n {
                    0 => {self.b = (r >> 8) as u8; self.c = (r & 0x00FF) as u8;},
                    1 => {self.d = (r >> 8) as u8; self.e = (r & 0x00FF) as u8;},
                    2 => {self.h = (r >> 8) as u8; self.l = (r & 0x00FF) as u8;},
                    3 => {self.sp = r;},
                    _ => panic!(
                        "Can't find {:?} for instruction {:#?}",
                        n, instruction,
                    ),
                }
                self.pc += 1;
            },
            Instruction::DecNN(n, nn) => {
                let r = nn.wrapping_sub(1);
                match n {
                    0 => {self.b = (r >> 8) as u8; self.c = (r & 0x00FF) as u8;},
                    1 => {self.d = (r >> 8) as u8; self.e = (r & 0x00FF) as u8;},
                    2 => {self.h = (r >> 8) as u8; self.l = (r & 0x00FF) as u8;},
                    3 => {self.sp = r;},
                    _ => panic!(
                        "Can't find {:?} for instruction {:#?}",
                        n, instruction,
                    ),
                }
                self.pc += 1;
            },

            // miscellaneous
            Instruction::SwapN(n, v) => {
                match n {
                    0 => {self.a = self.swap(v);},
                    1 => {self.b = self.swap(v);},
                    2 => {self.c = self.swap(v);},
                    3 => {self.d = self.swap(v);},
                    4 => {self.e = self.swap(v);},
                    5 => {self.h = self.swap(v);},
                    6 => {self.l = self.swap(v);},
                    _ => panic!(
                        "Can't find {:?} for instruction {:#?}",
                        n, instruction,
                    ),
                }
                self.pc += 2;
            },
            Instruction::SwapHl(v) => {
                mmu.write_byte(self.get_hl(), self.swap(v));
                self.pc += 2;
            },

            Instruction::Cpl(a) => {
                self.a = !a; 
                self.set_flag_n(true); 
                self.set_flag_h(true); 
                self.pc += 1;
            },
            Instruction::Scf => {
                self.set_flag_n(false); 
                self.set_flag_h(false); 
                self.set_flag_c(true); 
                self.pc += 1;
            },

            Instruction::Nop => {self.pc += 1;},


            // rotate and shift

            Instruction::Rlca(a) => {
                let c = a & 0x80 == 0x80;
                
                self.a = (a << 1) | (if c {1} else {0});

                self.set_flag_z(false);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(c);
                self.pc += 1;
            },
            Instruction::Rla(a) => {
                let c = a & 0x80 == 0x80;
                
                self.a = (a << 1) | (if self.get_flag_c() {1} else {0});

                self.set_flag_z(false);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(c);
                self.pc += 1;
            },

            Instruction::RlN(n, v) => {
                let c = v & 0x80 == 0x80;
                let r = (v << 1) | (if self.get_flag_c() {1} else {0});
                match n {
                    0 => {self.a = r;},
                    1 => {self.b = r;},
                    2 => {self.c = r;},
                    3 => {self.d = r;},
                    4 => {self.e = r;},
                    5 => {self.h = r;},
                    6 => {self.l = r;},
                    _ => panic!(
                        "Can't find {:?} for instruction {:#?}",
                        n, instruction,
                    ),
                }
                self.set_flag_z(r == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(c);
                self.pc += 2;
            },
            Instruction::RlHl(v) => {
                let c = v & 0x80 == 0x80;
                let r = (v << 1) | (if self.get_flag_c() {1} else {0});
                mmu.write_byte(self.get_hl(), r);
                self.set_flag_z(r == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(c);
                self.pc += 2;
            },
            Instruction::SlaN(n, v) => {
                let c = v & 0x80 == 0x80;
                let r = v << 1;
                match n {
                    0 => {self.a = r;},
                    1 => {self.b = r;},
                    2 => {self.c = r;},
                    3 => {self.d = r;},
                    4 => {self.e = r;},
                    5 => {self.h = r;},
                    6 => {self.l = r;},
                    7 => {mmu.write_byte(self.get_hl(), r);},
                    _ => panic!(
                        "Can't find {:?} for instruction {:#?}",
                        n, instruction,
                    ),
                }
                self.set_flag_z(r == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(c);
                self.pc += 2;
            },
            Instruction::SraN(n, v) => {
                let c = v & 0x01 == 0x01;
                let r = v >> 1 | (v & 0x80);
                match n {
                    0 => {self.a = r;},
                    1 => {self.b = r;},
                    2 => {self.c = r;},
                    3 => {self.d = r;},
                    4 => {self.e = r;},
                    5 => {self.h = r;},
                    6 => {self.l = r;},
                    7 => {mmu.write_byte(self.get_hl(), r);},
                    _ => panic!(
                        "Can't find {:?} for instruction {:#?}",
                        n, instruction,
                    ),
                }
                self.set_flag_z(r == 0);
                self.set_flag_n(false);
                self.set_flag_h(false);
                self.set_flag_c(c);
                self.pc += 2;
            },

            Instruction::BitbR(pos, r) => {
                let result: u8 = r & pos;
                if result == 0 {
                    self.set_flag_z(true);
                } else {
                    self.set_flag_z(false);
                }
                self.set_flag_n(false);
                self.set_flag_h(true);
                self.pc += 2;
            },
            Instruction::JrN(n) => {
                self.pc = self.pc.wrapping_add(*n as u16);
                self.pc += 2;

            }
            // cover both JrNz and JrZ
            Instruction::JrZ(n, b) => {
                // if b is false, then Z needs to be reset to jump
                // if b is true, then Z needs to be set to jump
                if self.get_flag_z() == *b {
                    //println!("sum: {:?}", self.pc.wrapping_add(*n as u16));
                    self.pc = self.pc.wrapping_add(*n as u16);
                }
                self.pc += 2;
                
            },
            // cover both JrNc and JrC
            Instruction::JrC(n, b) => {
                // if b is false, then C needs to be reset to jump
                // if b is true, then C needs to be set to jump
                if self.get_flag_n() == *b {
                    println!("sum: {:#x}", self.pc.wrapping_add(*n as u16));
                    self.pc = self.pc.wrapping_add(*n as u16);
                }
                self.pc += 2;
            },

            Instruction::Callnn(d16) => {
                self.pc += 3;
                let addr1: u8 = ((self.pc & 0xFF00) >> 8) as u8;
                let addr2: u8 = (self.pc & 0x00FF) as u8;
                self.sp = self.sp.wrapping_sub(1);
                mmu.write_byte(self.sp, addr1);
                self.sp = self.sp.wrapping_sub(1);
                mmu.write_byte(self.sp, addr2);
                self.pc = *d16;
            },

            Instruction::Ret(sp) => {
                //println!("CPU STATE: {:?}", self);
                let low = mmu.read_byte(*sp) as u16;
                self.sp = sp.wrapping_add(1);
                let high = mmu.read_byte(self.sp) as u16;
                self.sp = sp.wrapping_add(1);
                self.pc = (high << 8) | low;
            },

            _ => panic!(
                "Unreconized instruction {:?} on pc {:#X}\n CPU STATE: {:?}",
                instruction, self.pc, self
            ),
        }
    }

    pub fn run_instruction(&mut self, mmu: &mut MMU) {
        let byte = mmu.read_byte(self.pc);

        println!("Current byte {:#x} | pc: {:#X}", byte as u8, self.pc);

        let instruction = self.decode(byte, mmu);

        println!("Current instruction {:?} \n ", instruction);

        self.excute(&instruction, mmu)
    }
}