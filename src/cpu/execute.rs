
pub use crate::cpu::CPU;
pub use crate::instruction::Instruction;
pub use crate::mmu::MMU;
pub fn executer(cpu: &mut CPU, instruction: &Instruction, mmu: &mut MMU) -> u8 {
    match instruction {
        Instruction::LdN(n, v) => {
            match n {
                0 => {cpu.b = *v;},
                1 => {cpu.c = *v;},
                2 => {cpu.d = *v;},
                3 => {cpu.e = *v;},
                4 => {cpu.h = *v;},
                5 => {cpu.l = *v;},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
            cpu.pc += 2;
            2
        },

        Instruction::LdAR2(n, r2) => {
            cpu.a = *r2 as u8;
            cpu.pc += 1;
            match n {
                0 => {1},
                1 => {2},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
        },
        Instruction::LdAnn(r2) => {
            cpu.a = *r2 as u8;
            cpu.pc += 3;
            4
        },
        Instruction::LdAd8(r2) => {
            cpu.a = *r2 as u8;
            cpu.pc += 2;
            2
        },
        Instruction::LdBR2(r2, b) => {
            cpu.b = *r2 as u8;
            cpu.pc += 1;
            if *b {2} else {1}
        },
        Instruction::LdCR2(r2, b) => {
            cpu.c = *r2 as u8;
            cpu.pc += 1;
            if *b {2} else {1}
        },
        Instruction::LdDR2(r2, b) => {
            cpu.d = *r2 as u8;
            cpu.pc += 1;
            if *b {2} else {1}
        },
        Instruction::LdER2(r2, b) => {
            cpu.e = *r2 as u8;
            cpu.pc += 1;
            if *b {2} else {1}
        },
        Instruction::LdHR2(r2, b) => {
            cpu.h = *r2 as u8;
            cpu.pc += 1;
            if *b {2} else {1}
        },
        Instruction::LdLR2(r2, b) => {
            cpu.l = *r2 as u8;
            cpu.pc += 1;
            if *b {2} else {1}
        },
        Instruction::LdHlR2(r2) => {
            mmu.write_byte(cpu.get_hl(), *r2);
            cpu.pc += 1;
            2
        },

        Instruction::LdnA(n, a) => {
            match n {
                0 => {cpu.b = *a;},
                1 => {cpu.c = *a;},
                2 => {cpu.d = *a;},
                3 => {cpu.e = *a;},
                4 => {cpu.h = *a;},
                5 => {cpu.l = *a;},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
            cpu.pc += 1;
            1
        },
        Instruction::Ldn16A(n, a) => {
            mmu.write_byte(*n, *a);
            cpu.pc += 1;
            2
        },
        Instruction::Lda16A(n, a) => {
            mmu.write_byte(*n, *a);
            cpu.pc += 3;
            4
        },
        Instruction::LdAc(c) => {
            cpu.a = mmu.read_byte(0xFF00 | *c as u16);
            cpu.pc += 2;
            2
        },
        Instruction::LdCa(a) => {
            cpu.c = mmu.read_byte(0xFF00 | *a as u16);
            cpu.pc += 2;
            2
        },
        
        // put LddAhl
        

        Instruction::LddAHl(hl16) => {
            cpu.a = mmu.read_byte(*hl16);
            let mut hl = *hl16 as u16;
            hl = hl.wrapping_sub(1);
            cpu.set_hl(hl);
            cpu.pc += 1;
            2
        },
        Instruction::LddHlA(hl16) => {
            let mut hl = *hl16 as u16;
            mmu.write_byte(hl, cpu.a);
            hl = hl.wrapping_sub(1);
            cpu.set_hl(hl);
            cpu.pc += 1;
            // maybe implement DECHl here

            //println!("CPU STATE: {:?}", cpu);
            2
        },
        Instruction::LdIAHl(hl16) => {
            cpu.a = mmu.read_byte(*hl16);
            let mut hl = *hl16 as u16;
            hl = hl.wrapping_add(1);
            cpu.set_hl(hl);
            cpu.pc += 1;
            2
        },
        Instruction::LdIHlA(hl16) => {
            let mut hl = *hl16 as u16;
            mmu.write_byte(hl, cpu.a);
            hl = hl.wrapping_add(1);
            cpu.set_hl(hl);
            cpu.pc += 1;
            2
        },



        Instruction::LdHnA(n) => {
            mmu.write_byte(0xFF00 | *n as u16, cpu.a);
            cpu.pc += 2;
            3
        },
        Instruction::LdHAn(n) => {
            cpu.a = mmu.read_byte(0xFF00 | *n as u16);
            cpu.pc += 2;
            3
        },


        Instruction::LdBc(d16) => {
            cpu.b = (d16 >> 8) as u8;
            cpu.c = (d16 & 0x00FF) as u8;
            cpu.pc += 3;
            3
        },
        Instruction::LdDe(d16) => {
            cpu.d = (d16 >> 8) as u8;
            cpu.e = (d16 & 0x00FF) as u8;
            cpu.pc += 3;
            3
        },
        Instruction::LdHl(d16) => {
            cpu.h = (d16 >> 8) as u8;
            cpu.l = (d16 & 0x00FF) as u8;
            cpu.pc += 3;
            3
        },
        Instruction::LdSp(d16) => {
            cpu.sp = *d16;
            cpu.pc += 3;
            3
        },
        Instruction::LdSpHl(hl) => {
            cpu.sp = *hl;
            cpu.pc += 1;
            3
        },
        Instruction::LdHlSp(s8) => {
            let sp = cpu.sp as i16;
            let result = if *s8 >= 0 {sp.wrapping_add(*s8 as i16)} else {sp.wrapping_sub(s8.abs() as i16)};
            if *s8 >= 0 {
                cpu.set_flag_c(((cpu.sp & 0xFF) as i32 + *s8 as i32) as u16 > 0xFF);
                cpu.set_flag_h((cpu.sp & 0xF) + (*s8 as u8 & 0xF) as u16 > 0xF);
            } else {
                cpu.set_flag_c((result as u16 & 0xFF) <= cpu.sp & 0xFF);
                cpu.set_flag_h((result as u16 & 0xF) <= cpu.sp & 0xF);
            }
            cpu.set_hl(result as u16);
            cpu.pc += 2;
            3
        },
        Instruction::LdnnSp(d16) => {
            mmu.write_byte(*d16, (cpu.sp & 0x00FF) as u8);
            mmu.write_byte(d16.wrapping_add(1), (cpu.sp >> 8) as u8);
            5
        },

        Instruction::Pushnn(n16) => {
            let addr1: u8 = ((n16 & 0xFF00) >> 8) as u8;
            let addr2: u8 = (n16 & 0x00FF) as u8;
            cpu.sp = cpu.sp.wrapping_sub(1);
            mmu.write_byte(cpu.sp, addr1);
            cpu.sp = cpu.sp.wrapping_sub(1);
            mmu.write_byte(cpu.sp, addr2);
            cpu.pc += 1;
            4
        },
        Instruction::Popnn(n) => {
            let value1 = mmu.read_byte(cpu.sp);
            cpu.sp = cpu.sp.wrapping_add(1);
            let value2 = mmu.read_byte(cpu.sp);
            cpu.sp = cpu.sp.wrapping_add(1);
            match n {
                0 => {cpu.a = value2; cpu.f = value1;},
                1 => {cpu.b = value2; cpu.c = value1;},
                2 => {cpu.d = value2; cpu.e = value1;},
                3 => {cpu.h = value2; cpu.l = value1;},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
            cpu.pc += 1;
            3
        },

        // 8bit alu
        Instruction::AddN(n) => {
            cpu.a = cpu.alu_add(n, false);
            cpu.pc += 1;
            1
        },
        Instruction::AddHl(n) => {
            cpu.a = cpu.alu_add(n, false);
            cpu.pc += 1;
            2
        },
        Instruction::AddD8(n) => {
            cpu.a = cpu.alu_add(n, false);
            cpu.pc += 2;
            2
        },
        Instruction::AdcN(n) => {
            cpu.a = cpu.alu_add(n, true);
            cpu.pc += 1;
            1
        },
        Instruction::AdcHl(n) => {
            cpu.a = cpu.alu_add(n, true);
            cpu.pc += 1;
            2
        },
        Instruction::AdcD8(n) => {
            cpu.a = cpu.alu_add(n, true);
            cpu.pc += 2;
            2
        },

        Instruction::SubN(n) => {
            cpu.a = cpu.alu_sub(n, false);
            cpu.pc += 1;
            1
        },
        Instruction::SubHl(n) => {
            cpu.a = cpu.alu_sub(n, false);
            cpu.pc += 1;
            2
        },
        Instruction::SubD8(n) => {
            cpu.a = cpu.alu_sub(n, false);
            cpu.pc += 2;
            2
        },

        Instruction::SbcN(n) => {
            cpu.a = cpu.alu_sub(n, true);
            cpu.pc += 1;
            1
        },
        Instruction::SbcHl(n) => {
            cpu.a = cpu.alu_sub(n, true);
            cpu.pc += 1;
            2
        },
        Instruction::SbcD8(n) => {
            cpu.a = cpu.alu_sub(n, true);
            cpu.pc += 2;
            2
        },
        Instruction::Andn(n) => {
            let result = cpu.a & *n;
            cpu.a = result;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(true);
            cpu.set_flag_c(false);
            cpu.pc += 1;
            1
        },
        Instruction::AndHl(n) => {
            let result = cpu.a & *n;
            cpu.a = result;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(true);
            cpu.set_flag_c(false);
            cpu.pc += 1;
            2
        },
        Instruction::AndD8(n) => {
            let result = cpu.a & *n;
            cpu.a = result;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(true);
            cpu.set_flag_c(false);
            cpu.pc += 2;
            2
        },
        Instruction::OrN(n) => {
            let result = cpu.a | *n;
            cpu.a = result;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(false);
            // hl has more cycles
            cpu.pc += 1;
            1
        },
        Instruction::OrHl(n) => {
            let result = cpu.a | *n;
            cpu.a = result;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(false);
            // hl has more cycles
            cpu.pc += 1;
            2
        },
        Instruction::OrD8(n) => {
            let result = cpu.a | *n;
            cpu.a = result;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(false);
            // hl has more cycles
            cpu.pc += 2;
            2
        },

        Instruction::Xor(r) => {
            let result = cpu.a ^ *r;
            cpu.a = result;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(false);
            cpu.pc += 1;
            1
        },
        Instruction::XorHl(r) => {
            let result = cpu.a ^ *r;
            cpu.a = result;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(false);
            cpu.pc += 1;
            2
        },
        Instruction::XorD8(n) => {
            let result = cpu.a ^ *n;
            cpu.a = result;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(false);
            cpu.pc += 2;
            2
        },

        Instruction::Cp(n) => {
            cpu.a = cpu.alu_sub(n, false);
            cpu.pc += 1;
            1
        },
        Instruction::CpHl(n) => {
            cpu.a = cpu.alu_sub(n, false);
            cpu.pc += 1;
            2
        },
        Instruction::CpD8(n) => {
            cpu.a = cpu.alu_sub(n, false);
            cpu.pc += 2;
            2
        },

        Instruction::IncN(n) => {
            let r: u8;
            match n {
                0 => {r = cpu.a.wrapping_add(1); cpu.a = r;},
                1 => {r = cpu.b.wrapping_add(1); cpu.b = r;},
                2 => {r = cpu.c.wrapping_add(1); cpu.c = r;},
                3 => {r = cpu.d.wrapping_add(1); cpu.d = r;},
                4 => {r = cpu.e.wrapping_add(1); cpu.e = r;},
                5 => {r = cpu.h.wrapping_add(1); cpu.h = r;},
                6 => {r = cpu.l.wrapping_add(1); cpu.l = r;},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h((n & 0x0F) + 1 > 0x0F);
            //H - Set if carry from bit 3.
            cpu.pc += 1;
            1
        },
        Instruction::IncHl(hl) => {
            let n = mmu.read_byte(*hl);
            let r = n.wrapping_add(1);
            mmu.write_byte(*hl, r);
            
            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h((n & 0x0F) + 1 > 0x0F);

            cpu.pc += 1;
            3
        },
        Instruction::DecN(v, n) => {
            let r: u8;
            match v {
                0 => {r = cpu.a.wrapping_sub(1); cpu.a = r;},
                1 => {r = cpu.b.wrapping_sub(1); cpu.b = r;},
                2 => {r = cpu.c.wrapping_sub(1); cpu.c = r;},
                3 => {r = cpu.d.wrapping_sub(1); cpu.d = r;},
                4 => {r = cpu.e.wrapping_sub(1); cpu.e = r;},
                5 => {r = cpu.h.wrapping_sub(1); cpu.h = r;},
                6 => {r = cpu.l.wrapping_sub(1); cpu.l = r;},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(true);
            cpu.set_flag_h((n & 0x0F) == 0);

            cpu.pc += 1;
            1
        },
        // DEC Hl
        Instruction::DecHl(hl) => {
            let n = mmu.read_byte(*hl);
            let r = n.wrapping_sub(1);
            mmu.write_byte(*hl, r);
            
            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(true);
            cpu.set_flag_h((n & 0x0F) == 0);

            cpu.pc += 1;
            3
        },

        // 16bit arithmetic

        Instruction::AddHlN(nn) => {
            let hl = cpu.get_hl();
            let r = hl.wrapping_add(*nn);
            cpu.set_flag_h((hl & 0x07FF) + (hl & 0x07FF) > 0x07FF);
            cpu.set_flag_n(false);
            cpu.set_flag_c(hl > 0xFFFF - nn);

            cpu.set_hl(r);
            cpu.pc += 1;
            2
        },

        Instruction::IncNN(n, nn) => {
            let r = nn.wrapping_add(1);
            match n {
                0 => {cpu.b = (r >> 8) as u8; cpu.c = (r & 0x00FF) as u8;},
                1 => {cpu.d = (r >> 8) as u8; cpu.e = (r & 0x00FF) as u8;},
                2 => {cpu.h = (r >> 8) as u8; cpu.l = (r & 0x00FF) as u8;},
                3 => {cpu.sp = r;},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
            cpu.pc += 1;
            2
        },
        Instruction::DecNN(n, nn) => {
            let r = nn.wrapping_sub(1);
            match n {
                0 => {cpu.b = (r >> 8) as u8; cpu.c = (r & 0x00FF) as u8;},
                1 => {cpu.d = (r >> 8) as u8; cpu.e = (r & 0x00FF) as u8;},
                2 => {cpu.h = (r >> 8) as u8; cpu.l = (r & 0x00FF) as u8;},
                3 => {cpu.sp = r;},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
            cpu.pc += 1;
            2
        },

        // miscellaneous
        Instruction::SwapN(n, v) => {
            match n {
                0 => {cpu.a = cpu.swap(v);},
                1 => {cpu.b = cpu.swap(v);},
                2 => {cpu.c = cpu.swap(v);},
                3 => {cpu.d = cpu.swap(v);},
                4 => {cpu.e = cpu.swap(v);},
                5 => {cpu.h = cpu.swap(v);},
                6 => {cpu.l = cpu.swap(v);},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
            cpu.pc += 2;
            2
        },
        Instruction::SwapHl(v) => {
            mmu.write_byte(cpu.get_hl(), cpu.swap(v));
            cpu.pc += 2;
            4
        },

        Instruction::Daa => {
            let mut a = cpu.a;
            let mut adjust = if cpu.get_flag_c() {0x60} else {0x00};
            
            if cpu.get_flag_h() {adjust |= 0x06;};

            if cpu.get_flag_n() {
                a = a.wrapping_sub(adjust);
            } else {
                if a & 0x0F > 0x09 {adjust |= 0x06;};
                if a > 0x99 {adjust |= 0x60;};
                a = a.wrapping_add(adjust);
            }

            cpu.set_flag_c(adjust >= 0x60);
            cpu.set_flag_h(false);
            cpu.set_flag_z(a == 0);
            cpu.a = a;
            1
        },

        Instruction::Cpl(a) => {
            cpu.a = !a; 
            cpu.set_flag_n(true); 
            cpu.set_flag_h(true); 
            cpu.pc += 1;
            1
        },
        Instruction::Ccf => {
            cpu.set_flag_c(!cpu.get_flag_c());
            cpu.set_flag_h(false);
            cpu.set_flag_n(false);
            cpu.pc += 1;
            1
        },
        Instruction::Scf => {
            cpu.set_flag_n(false); 
            cpu.set_flag_h(false); 
            cpu.set_flag_c(true); 
            cpu.pc += 1;
            1
        },

        Instruction::Nop => {cpu.pc += 1; 1},

        Instruction::Halt => {
            // When halt instruction is executed several outcomes might occur:
                // - When IME = 1:
                //      In this case, the halt instruction works normally. It will
                //      stop exection and wait until an interrupt occure (`IF & IE & 0x1F != 0`),
                //      then it will exit halt mode and execute the interrupt normally.
                // - When IME = 0:
                //      - If an interrupt is waiting (`IF & IE & 0x1F != 0`), it
                //        will enter a `Halt bug` state, in this state, the halt
                //        mode is not entered and the PC register is not incremented
                //        on the next instruction.
                //      - If an interrupt is not waiting (`IF & IE & 0x1F == 0`),
                //        the cpu will enter halt mode normally and wait for an interrupt
                //        to occur like in *IME = 1* case but if an interrupt is
                //        requested it will not just to the interrupt vector
                //        and it will continue executing normally, we can think
                //        of it as being stuck in a large array of NOP instructions
                //        until an interrupt is requested.
            cpu.set_halted(true);
            cpu.pc += 1;
            1
        },
        Instruction::Stop => {
            cpu.pc += 2;
            1
        },

        Instruction::Di => {cpu.ime = false; cpu.pc += 1; 1},

        Instruction::Ei => {cpu.ei = true; cpu.pc += 1; 1},


        // rotate and shift
        Instruction::Rlca(a) => {
            let c = a & 0x80 == 0x80;
            
            cpu.a = (a << 1) | (if c {1} else {0});

            cpu.set_flag_z(false);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 1;
            1
        },
        Instruction::Rla(a) => {
            let c = a & 0x80 == 0x80;
            
            cpu.a = (a << 1) | (if cpu.get_flag_c() {1} else {0});

            cpu.set_flag_z(false);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 1;
            1
        },
        Instruction::Rrca(a) => {
            let c = a & 0x01 == 0x01;
            
            cpu.a = (a >> 1) | (if c {0x80} else {0});

            cpu.set_flag_z(false);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 1;
            1
        },
        Instruction::Rra(a) => {
            let c = a & 0x01 == 0x01;
            
            cpu.a = (a >> 1) | (if cpu.get_flag_c() {0x80} else {0});

            cpu.set_flag_z(false);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 1;
            1
        },

        Instruction::RlcN(n, v) => {
            let c = v & 0x80 == 0x80;
            let r = (v << 1) | (if c {1} else {0});

            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 2;
            match n {
                0 => {cpu.a = r; 2},
                1 => {cpu.b = r; 2},
                2 => {cpu.c = r; 2},
                3 => {cpu.d = r; 2},
                4 => {cpu.e = r; 2},
                5 => {cpu.h = r; 2},
                6 => {cpu.l = r; 2},
                7 => {mmu.write_byte(cpu.get_hl(), r); 4},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
        },

        Instruction::RlN(n, v) => {
            let c = v & 0x80 == 0x80;
            let r = (v << 1) | (if cpu.get_flag_c() {1} else {0});

            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 2;
            match n {
                0 => {cpu.a = r; 2},
                1 => {cpu.b = r; 2},
                2 => {cpu.c = r; 2},
                3 => {cpu.d = r; 2},
                4 => {cpu.e = r; 2},
                5 => {cpu.h = r; 2},
                6 => {cpu.l = r; 2},
                7 => {mmu.write_byte(cpu.get_hl(), r); 4},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
        },

        Instruction::RrcN(n, v) => {
            let c = v & 0x01 == 0x01;
            
            let r = (v >> 1) | (if c {0x80} else {0});

            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 2;
            match n {
                0 => {cpu.a = r; 2},
                1 => {cpu.b = r; 2},
                2 => {cpu.c = r; 2},
                3 => {cpu.d = r; 2},
                4 => {cpu.e = r; 2},
                5 => {cpu.h = r; 2},
                6 => {cpu.l = r; 2},
                7 => {mmu.write_byte(cpu.get_hl(), r); 4},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
        },

        Instruction::RrN(n, v) => {
            let c = v & 0x01 == 0x01;
            
            let r = (v >> 1) | (if cpu.get_flag_c() {0x80} else {0});

            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 2;
            match n {
                0 => {cpu.a = r; 2},
                1 => {cpu.b = r; 2},
                2 => {cpu.c = r; 2},
                3 => {cpu.d = r; 2},
                4 => {cpu.e = r; 2},
                5 => {cpu.h = r; 2},
                6 => {cpu.l = r; 2},
                7 => {mmu.write_byte(cpu.get_hl(), r); 4},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
        },
        
        Instruction::SlaN(n, v) => {
            let c = v & 0x80 == 0x80;
            let r = v << 1;
            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 2;
            match n {
                0 => {cpu.a = r; 2},
                1 => {cpu.b = r; 2},
                2 => {cpu.c = r; 2},
                3 => {cpu.d = r; 2},
                4 => {cpu.e = r; 2},
                5 => {cpu.h = r; 2},
                6 => {cpu.l = r; 2},
                7 => {mmu.write_byte(cpu.get_hl(), r); 4},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
        },
        Instruction::SraN(n, v) => {
            let c = v & 0x01 == 0x01;
            let r = v >> 1 | (v & 0x80);
            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 2;
            match n {
                0 => {cpu.a = r; 2},
                1 => {cpu.b = r; 2},
                2 => {cpu.c = r; 2},
                3 => {cpu.d = r; 2},
                4 => {cpu.e = r; 2},
                5 => {cpu.h = r; 2},
                6 => {cpu.l = r; 2},
                7 => {mmu.write_byte(cpu.get_hl(), r); 4},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
        },

        Instruction::SrlN(n, v) => {
            let c = v & 0x01 == 0x01;
            let r = v >> 1;
            cpu.set_flag_z(r == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(false);
            cpu.set_flag_c(c);
            cpu.pc += 2;
            match n {
                0 => {cpu.a = r; 2},
                1 => {cpu.b = r; 2},
                2 => {cpu.c = r; 2},
                3 => {cpu.d = r; 2},
                4 => {cpu.e = r; 2},
                5 => {cpu.h = r; 2},
                6 => {cpu.l = r; 2},
                7 => {mmu.write_byte(cpu.get_hl(), r); 4},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
        },

        Instruction::BitbR(pos, r) => {
            let result: u8 = r & pos;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(true);
            cpu.pc += 2;
            2
        },
        Instruction::BitbHl(pos, r) => {
            let result: u8 = r & pos;
            cpu.set_flag_z(result == 0);
            cpu.set_flag_n(false);
            cpu.set_flag_h(true);
            cpu.pc += 2;
            4
        },

        Instruction::SetbR(pos, r, n) => {
            cpu.pc += 2;
            match n {
                0 => {cpu.a = r | pos; 2},
                1 => {cpu.b = r | pos; 2},
                2 => {cpu.c = r | pos; 2},
                3 => {cpu.d = r | pos; 2},
                4 => {cpu.e = r | pos; 2},
                5 => {cpu.h = r | pos; 2},
                6 => {cpu.l = r | pos; 2},
                7 => {mmu.write_byte(cpu.get_hl(), r | pos); 4},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
        },

        Instruction::ResbR(pos, r, n) => {
            cpu.pc += 2;
            match n {
                0 => {cpu.a = r & !pos; 2},
                1 => {cpu.b = r & ! pos; 2},
                2 => {cpu.c = r & ! pos; 2},
                3 => {cpu.d = r & ! pos; 2},
                4 => {cpu.e = r & ! pos; 2},
                5 => {cpu.h = r & ! pos; 2},
                6 => {cpu.l = r & ! pos; 2},
                7 => {mmu.write_byte(cpu.get_hl(), r | pos); 4},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
        },

        Instruction::Jpnn(nn) => {
            cpu.pc = *nn;
            //cpu.pc += 3;
            4
        },

        Instruction::Jpcc(nn, m) => {
            //cpu.pc += 3;
            match m {
                0 => {if !cpu.get_flag_z() {cpu.pc = *nn; 4} else {cpu.pc += 3; 3}},
                1 => {if cpu.get_flag_z() {cpu.pc = *nn; 4} else {cpu.pc += 3; 3}},
                2 => {if !cpu.get_flag_c() {cpu.pc = *nn; 4} else {cpu.pc += 3; 3}},
                3 => {if cpu.get_flag_c() {cpu.pc = *nn; 4} else {cpu.pc += 3; 3}},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    m, instruction,
                ),
            }
        },

        Instruction::JpHl(hl) => {
            cpu.pc = *hl;
            cpu.pc += 1;
            1
        },


        Instruction::JrN(n) => {
            cpu.pc = cpu.pc.wrapping_add(*n as u16);
            cpu.pc += 2;
            3
        },

        Instruction::Jrcc(n, m) => {
            cpu.pc += 2;
            match m {
                0 => {if !cpu.get_flag_z() {cpu.pc = cpu.pc.wrapping_add(*n as u16); 3} else {2}},
                1 => {if cpu.get_flag_z() {cpu.pc = cpu.pc.wrapping_add(*n as u16); 3} else {2}},
                2 => {if !cpu.get_flag_c() {cpu.pc = cpu.pc.wrapping_add(*n as u16); 3} else {2}},
                3 => {if cpu.get_flag_c() {cpu.pc = cpu.pc.wrapping_add(*n as u16); 3} else {2}},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    m, instruction,
                ),
            }
        },

        Instruction::Callnn(d16) => {
            cpu.pc += 3;
            cpu.push_stack(mmu);
            cpu.pc = *d16;

            6
        },

        Instruction::Callcc(d16, n) => {
            cpu.pc += 3;
            let mut v = 0;
            match n {
                0 => {if !cpu.get_flag_z() {cpu.push_stack(mmu); cpu.pc = *d16; v = 6;} else {v = 3;}},
                1 => {if cpu.get_flag_z() {cpu.push_stack(mmu); cpu.pc = *d16; v = 6;} else {v = 3;}},
                2 => {if !cpu.get_flag_c() {cpu.push_stack(mmu); cpu.pc = *d16; v = 6;} else {v = 3;}},
                3 => {if cpu.get_flag_c() {cpu.push_stack(mmu); cpu.pc = *d16; v = 6;} else {v = 3;}},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
            v
        },

        Instruction::Rst(n) => {
            cpu.push_stack(mmu);
            cpu.pc = *n as u16;
            cpu.pc += 1;
            4
        },

        Instruction::Ret => {
            //println!("CPU STATE: {:?}", cpu);
            //println!("pop: {:#X}", cpu.pop_stack(mmu));
            cpu.pc = cpu.pop_stack(mmu);
            //cpu.pc += 1;
            4
        },

        Instruction::Retcc(n) => {
            let mut v = 0;
            match n {
                0 => {if !cpu.get_flag_z() {cpu.pc = cpu.pop_stack(mmu); v = 5;} else {v = 2;}},
                1 => {if cpu.get_flag_z() {cpu.pc = cpu.pop_stack(mmu); v = 5;} else {v = 2;}},
                2 => {if !cpu.get_flag_c() {cpu.pc = cpu.pop_stack(mmu); v = 5;} else {v = 2;}},
                3 => {if cpu.get_flag_c() {cpu.pc = cpu.pop_stack(mmu); v = 5;} else {v = 2;}},
                _ => panic!(
                    "Can't find {:?} for instruction {:#?}",
                    n, instruction,
                ),
            }
            cpu.pc += 1;
            v
        },

        Instruction::Reti => {
            cpu.pc = cpu.pop_stack(mmu);
            cpu.pc += 1;
            cpu.ime = true;
            4
        },

        _ => panic!(
            "Unreconized instruction {:?} on pc {:#X}\n CPU STATE: {:?}",
            instruction, cpu.pc, cpu
        ),
    }
}

