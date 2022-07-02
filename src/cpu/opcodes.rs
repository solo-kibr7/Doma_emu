pub use crate::cpu::CPU;
    pub use crate::instruction::Instruction;
    pub use crate::mmu::MMU;

    //CPU::get_a();
    pub fn decoder(cpu: &mut CPU, mmu: &MMU, byte: u8, n1: u16, d16: u16) -> Instruction {
        match byte {
    
            // 8 bit loads
            // 1. LD nn,n
            // Description: Put value nn into n.
            0x06 => Instruction::LdN(0, n1 as u8),
            0x0E => Instruction::LdN(1, n1 as u8),
            0x16 => Instruction::LdN(2, n1 as u8),
            0x1E => Instruction::LdN(3, n1 as u8),
            0x26 => Instruction::LdN(4, n1 as u8),
            0x2E => Instruction::LdN(5, n1 as u8),
    
    
            // 2. LD r1,r2
            // Description: Put value r2 into r1.
    
            // r1 = a is the same as 3. LD A,n
    
            0x7F => Instruction::LdAR2(0, cpu.get_a()),
            0x78 => Instruction::LdAR2(0, cpu.get_b()),
            0x79 => Instruction::LdAR2(0, cpu.get_c()),
            0x7A => Instruction::LdAR2(0, cpu.get_d()),
            0x7B => Instruction::LdAR2(0, cpu.get_e()),
            0x7C => Instruction::LdAR2(0, cpu.get_h()),
            0x7D => Instruction::LdAR2(0, cpu.get_l()),
            0x7E => Instruction::LdAR2(1, mmu.read_byte(cpu.get_hl())),
            0x0A => Instruction::LdAR2(1, mmu.read_byte(cpu.get_bc())),
            0x1A => Instruction::LdAR2(1, mmu.read_byte(cpu.get_de())),
            0xFA => Instruction::LdAnn(mmu.read_byte(d16)),
            0x3E => Instruction::LdAd8(n1 as u8),
    
            0x40 => Instruction::LdBR2(cpu.get_b(), false),
            0x41 => Instruction::LdBR2(cpu.get_c(), false),
            0x42 => Instruction::LdBR2(cpu.get_d(), false),
            0x43 => Instruction::LdBR2(cpu.get_e(), false),
            0x44 => Instruction::LdBR2(cpu.get_h(), false),
            0x45 => Instruction::LdBR2(cpu.get_l(), false),
            0x46 => Instruction::LdBR2(mmu.read_byte(cpu.get_hl()), true),
    
            0x48 => Instruction::LdCR2(cpu.get_b(), false),
            0x49 => Instruction::LdCR2(cpu.get_c(), false),
            0x4A => Instruction::LdCR2(cpu.get_d(), false),
            0x4B => Instruction::LdCR2(cpu.get_e(), false),
            0x4C => Instruction::LdCR2(cpu.get_h(), false),
            0x4D => Instruction::LdCR2(cpu.get_l(), false),
            0x4E => Instruction::LdCR2(mmu.read_byte(cpu.get_hl()), true),
    
            0x50 => Instruction::LdDR2(cpu.get_b(), false),
            0x51 => Instruction::LdDR2(cpu.get_c(), false),
            0x52 => Instruction::LdDR2(cpu.get_d(), false),
            0x53 => Instruction::LdDR2(cpu.get_e(), false),
            0x54 => Instruction::LdDR2(cpu.get_h(), false),
            0x55 => Instruction::LdDR2(cpu.get_l(), false),
            0x56 => Instruction::LdDR2(mmu.read_byte(cpu.get_hl()), true),
    
            0x58 => Instruction::LdER2(cpu.get_b(), false),
            0x59 => Instruction::LdER2(cpu.get_c(), false),
            0x5A => Instruction::LdER2(cpu.get_d(), false),
            0x5B => Instruction::LdER2(cpu.get_e(), false),
            0x5C => Instruction::LdER2(cpu.get_h(), false),
            0x5D => Instruction::LdER2(cpu.get_l(), false),
            0x5E => Instruction::LdER2(mmu.read_byte(cpu.get_hl()), true),
    
            0x60 => Instruction::LdHR2(cpu.get_b(), false),
            0x61 => Instruction::LdHR2(cpu.get_c(), false),
            0x62 => Instruction::LdHR2(cpu.get_d(), false),
            0x63 => Instruction::LdHR2(cpu.get_e(), false),
            0x64 => Instruction::LdHR2(cpu.get_h(), false),
            0x65 => Instruction::LdHR2(cpu.get_l(), false),
            0x66 => Instruction::LdHR2(mmu.read_byte(cpu.get_hl()), true),
    
            0x68 => Instruction::LdLR2(cpu.get_b(), false),
            0x69 => Instruction::LdLR2(cpu.get_c(), false),
            0x6A => Instruction::LdLR2(cpu.get_d(), false),
            0x6B => Instruction::LdLR2(cpu.get_e(), false),
            0x6C => Instruction::LdLR2(cpu.get_h(), false),
            0x6D => Instruction::LdLR2(cpu.get_l(), false),
            0x6E => Instruction::LdLR2(mmu.read_byte(cpu.get_hl()), true),

            0x77 => Instruction::LdHlR2(cpu.get_a(), false),
            0x70 => Instruction::LdHlR2(cpu.get_b(), false),
            0x71 => Instruction::LdHlR2(cpu.get_c(), false),
            0x72 => Instruction::LdHlR2(cpu.get_d(), false),
            0x73 => Instruction::LdHlR2(cpu.get_e(), false),
            0x74 => Instruction::LdHlR2(cpu.get_h(), false),
            0x75 => Instruction::LdHlR2(cpu.get_l(), false),
            0x36 => Instruction::LdHlR2(n1 as u8, true),
    
            // 4. LD n,A
            // Description:  Put value A into n.
            // the first parameter is so I can easily match it to a register for code reuse
            0x47 => Instruction::LdnA(0, cpu.get_a()),
            0x4F => Instruction::LdnA(1, cpu.get_a()),
            0x57 => Instruction::LdnA(2, cpu.get_a()),
            0x5F => Instruction::LdnA(3, cpu.get_a()),
            0x67 => Instruction::LdnA(4, cpu.get_a()),
            0x6F => Instruction::LdnA(5, cpu.get_a()),
            0x02 => Instruction::Ldn16A(cpu.get_bc(), cpu.get_a()),
            0x12 => Instruction::Ldn16A(cpu.get_de(), cpu.get_a()),
            0xEA => Instruction::Lda16A(d16, cpu.get_a()),
    
            // 5. LD A,(C)
            // Description: Put value at address $FF00 + register C into A. Same as: LD A,($FF00+C)
            0xF2 => Instruction::LdAc(cpu.get_c()),
    
            // 6. LD (C),A
            // Description: Put A into address $FF00 + register C
            0xE2 => Instruction::LdCa(cpu.get_a()),
    
            // 7-9. LDD A,(HL)
            // Description: Put value at address HL into A. Decrement HL.
            0x3A => Instruction::LddAHl(cpu.get_hl()),
    
            // 10-12. LDD (HL),A
            // Description: Put A into memory address HL. Decrement HL
            0x32 => Instruction::LddHlA(cpu.get_hl()),

            // 13-15. LDI A,(HL)
            // Description: Put value at address HL into A. Increment HL. Same as: LD A,(HL) - INC HL
            0x2A => Instruction::LdIAHl(cpu.get_hl()),

            // 17-18. LDI (HL),A
            // Description:  Put A into memory address HL. Increment HL.
            0x22 => Instruction::LdIHlA(cpu.get_hl()),
    
            // 19. LDH (n),A
            // Description:  Put A into memory address $FF00+n
            0xE0 => Instruction::LdHnA(n1 as u8),
    
            // 20. LDH A,(n)
            // Description:  Put memory address $FF00+n into A.
            0xF0 => Instruction::LdHAn(n1 as u8),
    
            // 16bit loads
            // 1. LD (n, nn)
            // Put value nn into n
            0x01 => Instruction::LdBc(d16),
            0x11 => Instruction::LdDe(d16),
            0x21 => Instruction::LdHl(d16),
            0x31 => Instruction::LdSp(d16),

            // 2. LD SP,HL
            // Description: Put HL into Stack Pointer (SP).
            0xF9 => Instruction::LdSpHl(cpu.get_hl()),


            // 3-4. LDHL SP,n
            // Description:  Put SP + n effective address into HL.
            0xF8 => Instruction::LdHlSp(n1 as i8),

            // 5. LD (nn),SP
            // Description: Put Stack Pointer (SP) at address n.
            0x08 => Instruction::LdnnSp(d16),

            // 6. PUSH nn
            // Description: Push register pair nn onto stack.  Decrement Stack Pointer (SP) twice.
            0xF5 => Instruction::Pushnn(cpu.get_af()),
            0xC5 => Instruction::Pushnn(cpu.get_bc()),
            0xD5 => Instruction::Pushnn(cpu.get_de()),
            0xE5 => Instruction::Pushnn(cpu.get_hl()),

            // 7. POP nn
            // Description: Pop two bytes off stack into register pair nn. Increment Stack Pointer twice.
            0xF1 => Instruction::Popnn(0),
            0xC1 => Instruction::Popnn(1),
            0xD1 => Instruction::Popnn(2),
            0xE1 => Instruction::Popnn(3),

            // 8bit alu
            // 1. ADD A,n
            // Description: Add n to A.
            0x87 => Instruction::AddN(cpu.get_a()),
            0x80 => Instruction::AddN(cpu.get_b()),
            0x81 => Instruction::AddN(cpu.get_c()),
            0x82 => Instruction::AddN(cpu.get_d()),
            0x83 => Instruction::AddN(cpu.get_e()),
            0x84 => Instruction::AddN(cpu.get_h()),
            0x85 => Instruction::AddN(cpu.get_l()),
            0x86 => Instruction::AddHl(mmu.read_byte(cpu.get_hl())),
            0xC6 => Instruction::AddD8(n1 as u8),

            // 2. ADC A,n
            // Description: Add n + Carry flag to A.
            0x8F => Instruction::AdcN(cpu.get_a()),
            0x88 => Instruction::AdcN(cpu.get_b()),
            0x89 => Instruction::AdcN(cpu.get_c()),
            0x8A => Instruction::AdcN(cpu.get_d()),
            0x8B => Instruction::AdcN(cpu.get_e()),
            0x8C => Instruction::AdcN(cpu.get_h()),
            0x8D => Instruction::AdcN(cpu.get_l()),
            0x8E => Instruction::AdcHl(mmu.read_byte(cpu.get_hl())),
            0xCE => Instruction::AdcD8(n1 as u8),

            // 3. SUB n
            // Description:  Subtract n from A.
            0x97 => Instruction::SubN(cpu.get_a()),
            0x90 => Instruction::SubN(cpu.get_b()),
            0x91 => Instruction::SubN(cpu.get_c()),
            0x92 => Instruction::SubN(cpu.get_d()),
            0x93 => Instruction::SubN(cpu.get_e()),
            0x94 => Instruction::SubN(cpu.get_h()),
            0x95 => Instruction::SubN(cpu.get_l()),
            0x96 => Instruction::SubHl(mmu.read_byte(cpu.get_hl())),
            0xD6 => Instruction::SubD8(n1 as u8),

            // 4. SBC A,n
            // Description:  Subtract n + Carry flag from A.
            0x9F => Instruction::SbcN(cpu.get_a()),
            0x98 => Instruction::SbcN(cpu.get_b()),
            0x99 => Instruction::SbcN(cpu.get_c()),
            0x9A => Instruction::SbcN(cpu.get_d()),
            0x9B => Instruction::SbcN(cpu.get_e()),
            0x9C => Instruction::SbcN(cpu.get_h()),
            0x9D => Instruction::SbcN(cpu.get_l()),
            0x9E => Instruction::SbcHl(mmu.read_byte(cpu.get_hl())),
            0xDE => Instruction::SbcD8(n1 as u8),

            // 5. AND nDescription: 
            // Logically AND n with A, result in A.
            0xA7 => Instruction::Andn(cpu.get_a()),
            0xA0 => Instruction::Andn(cpu.get_b()),
            0xA1 => Instruction::Andn(cpu.get_c()),
            0xA2 => Instruction::Andn(cpu.get_d()),
            0xA3 => Instruction::Andn(cpu.get_e()),
            0xA4 => Instruction::Andn(cpu.get_h()),
            0xA5 => Instruction::Andn(cpu.get_l()),
            0xA6 => Instruction::AndHl(mmu.read_byte(cpu.get_hl())),
            0xE6 => Instruction::AndD8(n1 as u8),

            // 6. OR n
            // Description:  Logical OR n with register A, result in A.
            0xB7 => Instruction::OrN(cpu.get_a()),
            0xB0 => Instruction::OrN(cpu.get_b()),
            0xB1 => Instruction::OrN(cpu.get_c()),
            0xB2 => Instruction::OrN(cpu.get_d()),
            0xB3 => Instruction::OrN(cpu.get_e()),
            0xB4 => Instruction::OrN(cpu.get_h()),
            0xB5 => Instruction::OrN(cpu.get_l()),
            0xB6 => Instruction::OrHl(mmu.read_byte(cpu.get_hl())),
            0xF6 => Instruction::OrD8(n1 as u8),
            
            // 7. XOR (n)
            // Logical exclusive OR n with register A, result in A
            0xAF => Instruction::Xor(cpu.get_a()),
            0xA8 => Instruction::Xor(cpu.get_b()),
            0xA9 => Instruction::Xor(cpu.get_c()),
            0xAA => Instruction::Xor(cpu.get_d()),
            0xAB => Instruction::Xor(cpu.get_e()),
            0xAC => Instruction::Xor(cpu.get_h()),
            0xAD => Instruction::Xor(cpu.get_l()),
            0xAE => Instruction::XorHl(mmu.read_byte(cpu.get_hl())),
            0xEE => Instruction::XorD8(n1 as u8),

            // 8. CP n
            // Description: Compare A with n. Does't store result into a
            0xBF => Instruction::Cp(cpu.get_a()),
            0xB8 => Instruction::Cp(cpu.get_b()),
            0xB9 => Instruction::Cp(cpu.get_c()),
            0xBA => Instruction::Cp(cpu.get_d()),
            0xBB => Instruction::Cp(cpu.get_e()),
            0xBC => Instruction::Cp(cpu.get_h()),
            0xBD => Instruction::Cp(cpu.get_l()),
            0xBE => Instruction::CpHl(mmu.read_byte(cpu.get_hl())),
            0xFE => Instruction::CpD8(n1 as u8),
            
            // 9. INC n
            // Description:  Increment register n.
            0x3C => Instruction::IncN(0, cpu.get_a()),
            0x04 => Instruction::IncN(1, cpu.get_b()),
            0x0C => Instruction::IncN(2, cpu.get_c()),
            0x14 => Instruction::IncN(3, cpu.get_d()),
            0x1C => Instruction::IncN(4, cpu.get_e()),
            0x24 => Instruction::IncN(5, cpu.get_h()),
            0x2C => Instruction::IncN(6, cpu.get_l()),
            0x34 => Instruction::IncHl(cpu.get_hl()),
    
            // 10. DEC n
            // Description:  Decrement register n.
            0x3D => Instruction::DecN(0, cpu.get_a()),
            0x05 => Instruction::DecN(1, cpu.get_b()),
            0x0D => Instruction::DecN(2, cpu.get_c()),
            0x15 => Instruction::DecN(3, cpu.get_d()),
            0x1D => Instruction::DecN(4, cpu.get_e()),
            0x25 => Instruction::DecN(5, cpu.get_h()),
            0x2D => Instruction::DecN(6, cpu.get_l()),
            0x35 => Instruction::DecHl(cpu.get_hl()),
    

            // 16-Bit Arithmetic
            // 1. ADD HL,n
            // Description: Add n to HL
            0x09 => Instruction::AddHlN(cpu.get_bc()),
            0x19 => Instruction::AddHlN(cpu.get_de()),
            0x29 => Instruction::AddHlN(cpu.get_hl()),
            0x39 => Instruction::AddHlN(cpu.get_sp()),

            // 2. ADD SP,n
            // Description:  Add n to Stack Pointer (SP).
            0xE8 => Instruction::AddSpN(n1 as i8),

            // 3. INC nn
            // Description: Increment register nn
            0x03 => Instruction::IncNN(0, cpu.get_bc()),
            0x13 => Instruction::IncNN(1, cpu.get_de()),
            0x23 => Instruction::IncNN(2, cpu.get_hl()),
            0x33 => Instruction::IncNN(3, cpu.get_sp()),

            // 4. DEC nn
            // Description:  Decrement register nn
            0x0B => Instruction::DecNN(0, cpu.get_bc()),
            0x1B => Instruction::DecNN(1, cpu.get_de()),
            0x2B => Instruction::DecNN(2, cpu.get_hl()),
            0x3B => Instruction::DecNN(3, cpu.get_sp()),

            // Miscellaneous

            //2. DAA
            // Description: Decimal adjust register A. 
            // This instruction adjusts register A so that the correct
            // representation of Binary Coded Decimal (BCD) is obtained.
            0x27 => Instruction::Daa,

            // 3. CPL
            // Description:  Complement A register. (Flip all bits.)
            0x2F => Instruction::Cpl(cpu.get_a()),

            // 4. CCF
            // Description:  Complement carry flag.
            0x3F => Instruction::Ccf,

            // 5. SCF
            // Description:  Set Carry flag
            0x37 => Instruction::Scf,

            // 6. NOP
            // Description: No operation.
            0x00 => Instruction::Nop,

            // 7. HALT 
            // Description: Power down CPU until an interrupt occurs. 
            // Use this when ever possible to reduce energy consumption
            0x76 => Instruction::Halt,

            // 8. STOP
            // Description: Halt CPU & LCD display until button pressed
            0x10 => Instruction::Stop,

            // 9. DI
            // Description: This instruction disables interrupts but not immediately. 
            // Interrupts are disabled after  instruction after DI is executed.
            0xF3 => Instruction::Di,

            // 10. EI
            // Description:  Enable interrupts. This intruction enables interrupts but not immediately.
            // Interrupts are enabled after  instruction after EI is executed.
            0xFB => Instruction::Ei,

    
            // rotate and shift
            // 1. RLCADescription:
            // Rotate A left. Old bit 7 to Carry flag.
            0x07 => Instruction::Rlca(cpu.get_a()),

            // 2. RLA
            // Description: Rotate A left through Carry flag.
            0x17 => Instruction::Rla(cpu.get_a()),

            // 3. RRCA
            // Description: Rotate A right. Old bit 0 to Carry flag.
            0x0F => Instruction::Rrca(cpu.get_a()),

            // 4. RRA
            // Description: Rotate A right through Carry flag
            0x1F => Instruction::Rra(cpu.get_a()),
    
            // jumps
            // 1. JP nn
            // Description:  Jump to address nn
            0xC3 => Instruction::Jpnn(d16),

            // 2. JP cc,nn
            // Description: Jump to address n if following condition is true:
            0xC2 => Instruction::Jpcc(d16, 0),
            0xCA => Instruction::Jpcc(d16, 1),
            0xD2 => Instruction::Jpcc(d16, 2),
            0xDA => Instruction::Jpcc(d16, 3),

            // 3. JP (HL)
            // Description:  Jump to address contained in HL.
            0xE9 => Instruction::JpHl(cpu.get_hl()),

            // 4. JR n
            // Description: Add n to current address and jump to it
            0x18 => Instruction::JrN(n1 as i8),

            //5. JR cc,n
            // Description: If following condition is true then add n to current address and jump to it:
            0x20 => Instruction::Jrcc(n1 as i8, 0),
            0x28 => Instruction::Jrcc(n1 as i8, 1),
            0x30 => Instruction::Jrcc(n1 as i8, 2),
            0x38 => Instruction::Jrcc(n1 as i8, 3),

            // calls
            // 1. CALL nn
            // Description: Push address of next instruction onto stack and then jump to address nn
            0xCD => Instruction::Callnn(d16),

            // 2. CALL cc,nn
            // Description:  Call address n if following condition is true:
            0xC4 => Instruction::Callcc(d16, 0),
            0xCC => Instruction::Callcc(d16, 1),
            0xD4 => Instruction::Callcc(d16, 2),
            0xDC => Instruction::Callcc(d16, 3),

            //restarts
            // 1. RST n
            // Description:  Push present address onto stack. Jump to address $0000 + n.
            0xC7 => Instruction::Rst(0x00),
            0xCF => Instruction::Rst(0x08),
            0xD7 => Instruction::Rst(0x10),
            0xDF => Instruction::Rst(0x18),
            0xE7 => Instruction::Rst(0x20),
            0xEF => Instruction::Rst(0x28),
            0xF7 => Instruction::Rst(0x30),
            0xFF => Instruction::Rst(0x38),

            //returns
            // 1. RET
            // Description: Pop two bytes from stack & jump to that address
            0xC9 => Instruction::Ret,

            // 2. RET cc
            // Description: Return if following condition is true:
            0xC0 => Instruction::Retcc(0),
            0xC8 => Instruction::Retcc(1),
            0xD0 => Instruction::Retcc(2),
            0xD8 => Instruction::Retcc(3),

            // 3. RETI
            // Description: Pop two bytes from stack & jump to that address then  enable interrupts.
            0xD9 => Instruction::Reti,
    
            0xCB => {
                //cpu.pc += 1;
                //println!("Current byte {:#X}", n1 as u8);
                //println!("Current Instruction {:?} \n", cpu.cb_decode(n1 as u8, mmu));
                cpu.cb_decode(n1 as u8, mmu)
            },
            
    
            _ => panic!(
                "Decode: Unreconized byte {:#X} on pc {:#X}\n cpu STATE: {:?}",
                byte, cpu.get_pc(), cpu
            ),
        }
    }
    pub fn cb_decoder(cpu: &mut CPU, byte: u8, mmu: &MMU) -> Instruction {

        match byte {
            // Miscellaneous
            // 1. SWAP n
            // Description:  Swap upper & lower nibles of n
            0x37 => Instruction::SwapN(0, cpu.get_a()),
            0x30 => Instruction::SwapN(1, cpu.get_b()),
            0x31 => Instruction::SwapN(2, cpu.get_c()),
            0x32 => Instruction::SwapN(3, cpu.get_d()),
            0x33 => Instruction::SwapN(4, cpu.get_e()),
            0x34 => Instruction::SwapN(5, cpu.get_h()),
            0x35 => Instruction::SwapN(6, cpu.get_l()),
            0x36 => Instruction::SwapHl(mmu.read_byte(cpu.get_hl())),

            // rotates and shifts
            // 5. RLC n
            // Description: Rotate n left. Old bit 7 to Carry flag.
            0x07 => Instruction::RlcN(0, cpu.get_a()),
            0x00 => Instruction::RlcN(1, cpu.get_b()),
            0x01 => Instruction::RlcN(2, cpu.get_c()),
            0x02 => Instruction::RlcN(3, cpu.get_d()),
            0x03 => Instruction::RlcN(4, cpu.get_e()),
            0x04 => Instruction::RlcN(5, cpu.get_h()),
            0x05 => Instruction::RlcN(6, cpu.get_l()),
            0x06 => Instruction::RlcN(7, mmu.read_byte(cpu.get_hl())),

            // 6. RL n
            // Description: Rotate n left through Carry flag.
            0x17 => Instruction::RlN(0, cpu.get_a()),
            0x10 => Instruction::RlN(1, cpu.get_b()),
            0x11 => Instruction::RlN(2, cpu.get_c()),
            0x12 => Instruction::RlN(3, cpu.get_d()),
            0x13 => Instruction::RlN(4, cpu.get_e()),
            0x14 => Instruction::RlN(5, cpu.get_h()),
            0x15 => Instruction::RlN(6, cpu.get_l()),
            0x16 => Instruction::RlN(7, mmu.read_byte(cpu.get_hl())),


            // 7. RRC n
            // Description: Rotate n right. Old bit 0 to Carry flag.
            0x0F => Instruction::RrcN(0, cpu.get_a()),
            0x08 => Instruction::RrcN(1, cpu.get_b()),
            0x09 => Instruction::RrcN(2, cpu.get_c()),
            0x0A => Instruction::RrcN(3, cpu.get_d()),
            0x0B => Instruction::RrcN(4, cpu.get_e()),
            0x0C => Instruction::RrcN(5, cpu.get_h()),
            0x0D => Instruction::RrcN(6, cpu.get_l()),
            0x0E => Instruction::RrcN(7, mmu.read_byte(cpu.get_hl())),

            // 8. RR n
            // Description: Rotate n right through Carry flag.
            0x1F => Instruction::RrN(0, cpu.get_a()),
            0x18 => Instruction::RrN(1, cpu.get_b()),
            0x19 => Instruction::RrN(2, cpu.get_c()),
            0x1A => Instruction::RrN(3, cpu.get_d()),
            0x1B => Instruction::RrN(4, cpu.get_e()),
            0x1C => Instruction::RrN(5, cpu.get_h()),
            0x1D => Instruction::RrN(6, cpu.get_l()),
            0x1E => Instruction::RrN(7, mmu.read_byte(cpu.get_hl())),

            // 9. SLA n
            // Description:  Shift n left into Carry. LSB of n set to 0
            0x27 => Instruction::SlaN(0, cpu.get_a()),
            0x20 => Instruction::SlaN(1, cpu.get_b()),
            0x21 => Instruction::SlaN(2, cpu.get_c()),
            0x22 => Instruction::SlaN(3, cpu.get_d()),
            0x23 => Instruction::SlaN(4, cpu.get_e()),
            0x24 => Instruction::SlaN(5, cpu.get_h()),
            0x25 => Instruction::SlaN(6, cpu.get_l()),
            0x26 => Instruction::SlaN(7, mmu.read_byte(cpu.get_hl())),

            // 10. SRA n
            // Description:  Shift n right into Carry. MSB doesn't change
            0x2F => Instruction::SraN(0, cpu.get_a()),
            0x28 => Instruction::SraN(1, cpu.get_b()),
            0x29 => Instruction::SraN(2, cpu.get_c()),
            0x2A => Instruction::SraN(3, cpu.get_d()),
            0x2B => Instruction::SraN(4, cpu.get_e()),
            0x2C => Instruction::SraN(5, cpu.get_h()),
            0x2D => Instruction::SraN(6, cpu.get_l()),
            0x2E => Instruction::SraN(7, mmu.read_byte(cpu.get_hl())),

            // 11. SRL n
            // Description:  Shift n right into Carry. MSB set to 0.
            0x3F => Instruction::SrlN(0, cpu.get_a()),
            0x38 => Instruction::SrlN(1, cpu.get_b()),
            0x39 => Instruction::SrlN(2, cpu.get_c()),
            0x3A => Instruction::SrlN(3, cpu.get_d()),
            0x3B => Instruction::SrlN(4, cpu.get_e()),
            0x3C => Instruction::SrlN(5, cpu.get_h()),
            0x3D => Instruction::SrlN(6, cpu.get_l()),
            0x3E => Instruction::SrlN(7, mmu.read_byte(cpu.get_hl())),

            // 1. BIT b,rDescription:
            // Test bit b in register r.

            // bit 0
            0x47 => Instruction::BitbR(0b00000001, cpu.get_a()),
            0x40 => Instruction::BitbR(0b00000001, cpu.get_b()),
            0x41 => Instruction::BitbR(0b00000001, cpu.get_c()),
            0x42 => Instruction::BitbR(0b00000001, cpu.get_d()),
            0x43 => Instruction::BitbR(0b00000001, cpu.get_e()),
            0x44 => Instruction::BitbR(0b00000001, cpu.get_h()),
            0x45 => Instruction::BitbR(0b00000001, cpu.get_l()),
            0x46 => Instruction::BitbHl(0b00000001, mmu.read_byte(cpu.get_hl())),

            // bit 1
            0x4F => Instruction::BitbR(0b00000010, cpu.get_a()),
            0x48 => Instruction::BitbR(0b00000010, cpu.get_b()),
            0x49 => Instruction::BitbR(0b00000010, cpu.get_c()),
            0x4A => Instruction::BitbR(0b00000010, cpu.get_d()),
            0x4B => Instruction::BitbR(0b00000010, cpu.get_e()),
            0x4C => Instruction::BitbR(0b00000010, cpu.get_h()),
            0x4D => Instruction::BitbR(0b00000010, cpu.get_l()),
            0x4E => Instruction::BitbHl(0b00000010, mmu.read_byte(cpu.get_hl())),

            // bit 2
            0x57 => Instruction::BitbR(0b00000100, cpu.get_a()),
            0x50 => Instruction::BitbR(0b00000100, cpu.get_b()),
            0x51 => Instruction::BitbR(0b00000100, cpu.get_c()),
            0x52 => Instruction::BitbR(0b00000100, cpu.get_d()),
            0x53 => Instruction::BitbR(0b00000100, cpu.get_e()),
            0x54 => Instruction::BitbR(0b00000100, cpu.get_h()),
            0x55 => Instruction::BitbR(0b00000100, cpu.get_l()),
            0x56 => Instruction::BitbHl(0b00000100, mmu.read_byte(cpu.get_hl())),

            // bit 3
            0x5F => Instruction::BitbR(0b00001000, cpu.get_a()),
            0x58 => Instruction::BitbR(0b00001000, cpu.get_b()),
            0x59 => Instruction::BitbR(0b00001000, cpu.get_c()),
            0x5A => Instruction::BitbR(0b00001000, cpu.get_d()),
            0x5B => Instruction::BitbR(0b00001000, cpu.get_e()),
            0x5C => Instruction::BitbR(0b00001000, cpu.get_h()),
            0x5D => Instruction::BitbR(0b00001000, cpu.get_l()),
            0x5E => Instruction::BitbHl(0b00001000, mmu.read_byte(cpu.get_hl())),

            // bit 4
            0x67 => Instruction::BitbR(0b00010000, cpu.get_a()),
            0x60 => Instruction::BitbR(0b00010000, cpu.get_b()),
            0x61 => Instruction::BitbR(0b00010000, cpu.get_c()),
            0x62 => Instruction::BitbR(0b00010000, cpu.get_d()),
            0x63 => Instruction::BitbR(0b00010000, cpu.get_e()),
            0x64 => Instruction::BitbR(0b00010000, cpu.get_h()),
            0x65 => Instruction::BitbR(0b00010000, cpu.get_l()),
            0x66 => Instruction::BitbHl(0b00010000, mmu.read_byte(cpu.get_hl())),

            // bit 5
            0x6F => Instruction::BitbR(0b00100000, cpu.get_a()),
            0x68 => Instruction::BitbR(0b00100000, cpu.get_b()),
            0x69 => Instruction::BitbR(0b00100000, cpu.get_c()),
            0x6A => Instruction::BitbR(0b00100000, cpu.get_d()),
            0x6B => Instruction::BitbR(0b00100000, cpu.get_e()),
            0x6C => Instruction::BitbR(0b00100000, cpu.get_h()),
            0x6D => Instruction::BitbR(0b00100000, cpu.get_l()),
            0x6E => Instruction::BitbHl(0b00100000, mmu.read_byte(cpu.get_hl())),

            // bit 6
            0x77 => Instruction::BitbR(0b01000000, cpu.get_a()),
            0x70 => Instruction::BitbR(0b01000000, cpu.get_b()),
            0x71 => Instruction::BitbR(0b01000000, cpu.get_c()),
            0x72 => Instruction::BitbR(0b01000000, cpu.get_d()),
            0x73 => Instruction::BitbR(0b01000000, cpu.get_e()),
            0x74 => Instruction::BitbR(0b01000000, cpu.get_h()),
            0x75 => Instruction::BitbR(0b01000000, cpu.get_l()),
            0x76 => Instruction::BitbHl(0b01000000, mmu.read_byte(cpu.get_hl())),

            // bit 7
            0x7F => Instruction::BitbR(0b10000000, cpu.get_a()),
            0x78 => Instruction::BitbR(0b10000000, cpu.get_b()),
            0x79 => Instruction::BitbR(0b10000000, cpu.get_c()),
            0x7A => Instruction::BitbR(0b10000000, cpu.get_d()),
            0x7B => Instruction::BitbR(0b10000000, cpu.get_e()),
            0x7C => Instruction::BitbR(0b10000000, cpu.get_h()),
            0x7D => Instruction::BitbR(0b10000000, cpu.get_l()),
            0x7E => Instruction::BitbHl(0b10000000, mmu.read_byte(cpu.get_hl())),

            // 2. SET b,r
            // Description:  Set bit b in register r.

            // bit 0
            0xC7 => Instruction::SetbR(0b00000001, cpu.get_a(), 0),
            0xC0 => Instruction::SetbR(0b00000001, cpu.get_b(), 1),
            0xC1 => Instruction::SetbR(0b00000001, cpu.get_c(), 2),
            0xC2 => Instruction::SetbR(0b00000001, cpu.get_d(), 3),
            0xC3 => Instruction::SetbR(0b00000001, cpu.get_e(), 4),
            0xC4 => Instruction::SetbR(0b00000001, cpu.get_h(), 5),
            0xC5 => Instruction::SetbR(0b00000001, cpu.get_l(), 6),
            0xC6 => Instruction::SetbR(0b00000001, mmu.read_byte(cpu.get_hl()), 7),

            // bit 1
            0xCF => Instruction::SetbR(0b00000010, cpu.get_a(), 0),
            0xC8 => Instruction::SetbR(0b00000010, cpu.get_b(), 1),
            0xC9 => Instruction::SetbR(0b00000010, cpu.get_c(), 2),
            0xCA => Instruction::SetbR(0b00000010, cpu.get_d(), 3),
            0xCB => Instruction::SetbR(0b00000010, cpu.get_e(), 4),
            0xCC => Instruction::SetbR(0b00000010, cpu.get_h(), 5),
            0xCD => Instruction::SetbR(0b00000010, cpu.get_l(), 6),
            0xCE => Instruction::SetbR(0b00000010, mmu.read_byte(cpu.get_hl()), 7),

            // bit 2
            0xD7 => Instruction::SetbR(0b00000100, cpu.get_a(), 0),
            0xD0 => Instruction::SetbR(0b00000100, cpu.get_b(), 1),
            0xD1 => Instruction::SetbR(0b00000100, cpu.get_c(), 2),
            0xD2 => Instruction::SetbR(0b00000100, cpu.get_d(), 3),
            0xD3 => Instruction::SetbR(0b00000100, cpu.get_e(), 4),
            0xD4 => Instruction::SetbR(0b00000100, cpu.get_h(), 5),
            0xD5 => Instruction::SetbR(0b00000100, cpu.get_l(), 6),
            0xD6 => Instruction::SetbR(0b00000100, mmu.read_byte(cpu.get_hl()), 7),

            // bit 3
            0xDF => Instruction::SetbR(0b00001000, cpu.get_a(), 0),
            0xD8 => Instruction::SetbR(0b00001000, cpu.get_b(), 1),
            0xD9 => Instruction::SetbR(0b00001000, cpu.get_c(), 2),
            0xDA => Instruction::SetbR(0b00001000, cpu.get_d(), 3),
            0xDB => Instruction::SetbR(0b00001000, cpu.get_e(), 4),
            0xDC => Instruction::SetbR(0b00001000, cpu.get_h(), 5),
            0xDD => Instruction::SetbR(0b00001000, cpu.get_l(), 6),
            0xDE => Instruction::SetbR(0b00001000, mmu.read_byte(cpu.get_hl()), 7),

            // bit 4
            0xE7 => Instruction::SetbR(0b00010000, cpu.get_a(), 0),
            0xE0 => Instruction::SetbR(0b00010000, cpu.get_b(), 1),
            0xE1 => Instruction::SetbR(0b00010000, cpu.get_c(), 2),
            0xE2 => Instruction::SetbR(0b00010000, cpu.get_d(), 3),
            0xE3 => Instruction::SetbR(0b00010000, cpu.get_e(), 4),
            0xE4 => Instruction::SetbR(0b00010000, cpu.get_h(), 5),
            0xE5 => Instruction::SetbR(0b00010000, cpu.get_l(), 6),
            0xE6 => Instruction::SetbR(0b00010000, mmu.read_byte(cpu.get_hl()), 7),

            // bit 5
            0xEF => Instruction::SetbR(0b00100000, cpu.get_a(), 0),
            0xE8 => Instruction::SetbR(0b00100000, cpu.get_b(), 1),
            0xE9 => Instruction::SetbR(0b00100000, cpu.get_c(), 2),
            0xEA => Instruction::SetbR(0b00100000, cpu.get_d(), 3),
            0xEB => Instruction::SetbR(0b00100000, cpu.get_e(), 4),
            0xEC => Instruction::SetbR(0b00100000, cpu.get_h(), 5),
            0xED => Instruction::SetbR(0b00100000, cpu.get_l(), 6),
            0xEE => Instruction::SetbR(0b00100000, mmu.read_byte(cpu.get_hl()), 7),

            // bit 6
            0xF7 => Instruction::SetbR(0b01000000, cpu.get_a(), 0),
            0xF0 => Instruction::SetbR(0b01000000, cpu.get_b(), 1),
            0xF1 => Instruction::SetbR(0b01000000, cpu.get_c(), 2),
            0xF2 => Instruction::SetbR(0b01000000, cpu.get_d(), 3),
            0xF3 => Instruction::SetbR(0b01000000, cpu.get_e(), 4),
            0xF4 => Instruction::SetbR(0b01000000, cpu.get_h(), 5),
            0xF5 => Instruction::SetbR(0b01000000, cpu.get_l(), 6),
            0xF6 => Instruction::SetbR(0b01000000, mmu.read_byte(cpu.get_hl()), 7),

            // bit 7
            0xFF => Instruction::SetbR(0b10000000, cpu.get_a(), 0),
            0xF8 => Instruction::SetbR(0b10000000, cpu.get_b(), 1),
            0xF9 => Instruction::SetbR(0b10000000, cpu.get_c(), 2),
            0xFA => Instruction::SetbR(0b10000000, cpu.get_d(), 3),
            0xFB => Instruction::SetbR(0b10000000, cpu.get_e(), 4),
            0xFC => Instruction::SetbR(0b10000000, cpu.get_h(), 5),
            0xFD => Instruction::SetbR(0b10000000, cpu.get_l(), 6),
            0xFE => Instruction::SetbR(0b10000000, mmu.read_byte(cpu.get_hl()), 7),

            // 3. RES b,r
            // Description: Reset bit b in register r

            // bit 0
            0x87 => Instruction::ResbR(0b00000001, cpu.get_a(), 0),
            0x80 => Instruction::ResbR(0b00000001, cpu.get_b(), 1),
            0x81 => Instruction::ResbR(0b00000001, cpu.get_c(), 2),
            0x82 => Instruction::ResbR(0b00000001, cpu.get_d(), 3),
            0x83 => Instruction::ResbR(0b00000001, cpu.get_e(), 4),
            0x84 => Instruction::ResbR(0b00000001, cpu.get_h(), 5),
            0x85 => Instruction::ResbR(0b00000001, cpu.get_l(), 6),
            0x86 => Instruction::ResbR(0b00000001, mmu.read_byte(cpu.get_hl()), 7),

            // bit 1
            0x8F => Instruction::ResbR(0b00000010, cpu.get_a(), 0),
            0x88 => Instruction::ResbR(0b00000010, cpu.get_b(), 1),
            0x89 => Instruction::ResbR(0b00000010, cpu.get_c(), 2),
            0x8A => Instruction::ResbR(0b00000010, cpu.get_d(), 3),
            0x8B => Instruction::ResbR(0b00000010, cpu.get_e(), 4),
            0x8C => Instruction::ResbR(0b00000010, cpu.get_h(), 5),
            0x8D => Instruction::ResbR(0b00000010, cpu.get_l(), 6),
            0x8E => Instruction::ResbR(0b00000010, mmu.read_byte(cpu.get_hl()), 7),

            // bit 2
            0x97 => Instruction::ResbR(0b00000100, cpu.get_a(), 0),
            0x90 => Instruction::ResbR(0b00000100, cpu.get_b(), 1),
            0x91 => Instruction::ResbR(0b00000100, cpu.get_c(), 2),
            0x92 => Instruction::ResbR(0b00000100, cpu.get_d(), 3),
            0x93 => Instruction::ResbR(0b00000100, cpu.get_e(), 4),
            0x94 => Instruction::ResbR(0b00000100, cpu.get_h(), 5),
            0x95 => Instruction::ResbR(0b00000100, cpu.get_l(), 6),
            0x96 => Instruction::ResbR(0b00000100, mmu.read_byte(cpu.get_hl()), 7),

            // bit 3
            0x9F => Instruction::ResbR(0b00001000, cpu.get_a(), 0),
            0x98 => Instruction::ResbR(0b00001000, cpu.get_b(), 1),
            0x99 => Instruction::ResbR(0b00001000, cpu.get_c(), 2),
            0x9A => Instruction::ResbR(0b00001000, cpu.get_d(), 3),
            0x9B => Instruction::ResbR(0b00001000, cpu.get_e(), 4),
            0x9C => Instruction::ResbR(0b00001000, cpu.get_h(), 5),
            0x9D => Instruction::ResbR(0b00001000, cpu.get_l(), 6),
            0x9E => Instruction::ResbR(0b00001000, mmu.read_byte(cpu.get_hl()), 7),

            // bit 4
            0xA7 => Instruction::ResbR(0b00010000, cpu.get_a(), 0),
            0xA0 => Instruction::ResbR(0b00010000, cpu.get_b(), 1),
            0xA1 => Instruction::ResbR(0b00010000, cpu.get_c(), 2),
            0xA2 => Instruction::ResbR(0b00010000, cpu.get_d(), 3),
            0xA3 => Instruction::ResbR(0b00010000, cpu.get_e(), 4),
            0xA4 => Instruction::ResbR(0b00010000, cpu.get_h(), 5),
            0xA5 => Instruction::ResbR(0b00010000, cpu.get_l(), 6),
            0xA6 => Instruction::ResbR(0b00010000, mmu.read_byte(cpu.get_hl()), 7),

            // bit 5
            0xAF => Instruction::ResbR(0b00100000, cpu.get_a(), 0),
            0xA8 => Instruction::ResbR(0b00100000, cpu.get_b(), 1),
            0xA9 => Instruction::ResbR(0b00100000, cpu.get_c(), 2),
            0xAA => Instruction::ResbR(0b00100000, cpu.get_d(), 3),
            0xAB => Instruction::ResbR(0b00100000, cpu.get_e(), 4),
            0xAC => Instruction::ResbR(0b00100000, cpu.get_h(), 5),
            0xAD => Instruction::ResbR(0b00100000, cpu.get_l(), 6),
            0xAE => Instruction::ResbR(0b00100000, mmu.read_byte(cpu.get_hl()), 7),

            // bit 6
            0xB7 => Instruction::ResbR(0b01000000, cpu.get_a(), 0),
            0xB0 => Instruction::ResbR(0b01000000, cpu.get_b(), 1),
            0xB1 => Instruction::ResbR(0b01000000, cpu.get_c(), 2),
            0xB2 => Instruction::ResbR(0b01000000, cpu.get_d(), 3),
            0xB3 => Instruction::ResbR(0b01000000, cpu.get_e(), 4),
            0xB4 => Instruction::ResbR(0b01000000, cpu.get_h(), 5),
            0xB5 => Instruction::ResbR(0b01000000, cpu.get_l(), 6),
            0xB6 => Instruction::ResbR(0b01000000, mmu.read_byte(cpu.get_hl()), 7),

            // bit 7
            0xBF => Instruction::ResbR(0b10000000, cpu.get_a(), 0),
            0xB8 => Instruction::ResbR(0b10000000, cpu.get_b(), 1),
            0xB9 => Instruction::ResbR(0b10000000, cpu.get_c(), 2),
            0xBA => Instruction::ResbR(0b10000000, cpu.get_d(), 3),
            0xBB => Instruction::ResbR(0b10000000, cpu.get_e(), 4),
            0xBC => Instruction::ResbR(0b10000000, cpu.get_h(), 5),
            0xBD => Instruction::ResbR(0b10000000, cpu.get_l(), 6),
            0xBE => Instruction::ResbR(0b10000000, mmu.read_byte(cpu.get_hl()), 7),
        }
    }