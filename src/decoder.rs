
pub mod Decoder {
    pub use crate::cpu::CPU;
    pub use crate::instruction::Instruction;
    pub use crate::mmu::MMU;

    //CPU::get_a();
    pub fn decoder(cpu: &mut CPU, mmu: &MMU, byte: u8, n1: u16, d16: u16) -> Instruction {
        match byte {
    
            // 8 bit loads
            // 1. LD nn,n
            // Description: Put value nn into n.
            0x06 => Instruction::LdB(n1 as u8),
            0x0E => Instruction::LdC(n1 as u8),
            0x16 => Instruction::LdD(n1 as u8),
            0x1E => Instruction::LdE(n1 as u8),
            0x26 => Instruction::LdH(n1 as u8),
            0x2E => Instruction::LdL(n1 as u8),
    
    
            // 2. LD r1,r2
            // Description: Put value r2 into r1.
    
            // r1 = a is the same as 3. LD A,n
    
            0x7F => Instruction::LdAR2(cpu.get_a()),
            0x78 => Instruction::LdAR2(cpu.get_b()),
            0x79 => Instruction::LdAR2(cpu.get_c()),
            0x7A => Instruction::LdAR2(cpu.get_d()),
            0x7B => Instruction::LdAR2(cpu.get_e()),
            0x7C => Instruction::LdAR2(cpu.get_h()),
            0x7D => Instruction::LdAR2(cpu.get_l()),
            0x7E => Instruction::LdAR2(mmu.read_byte(cpu.get_hl())),
            0x0A => Instruction::LdAR2(mmu.read_byte(cpu.get_bc())),
            0x1A => Instruction::LdAR2(mmu.read_byte(cpu.get_de())),
            0xFA => Instruction::LdAnn(mmu.read_byte(d16)),
            0x3E => Instruction::LdAd8(n1 as u8),
    
            0x40 => Instruction::LdBR2(cpu.get_b()),
            0x41 => Instruction::LdBR2(cpu.get_c()),
            0x42 => Instruction::LdBR2(cpu.get_d()),
            0x43 => Instruction::LdBR2(cpu.get_e()),
            0x44 => Instruction::LdBR2(cpu.get_h()),
            0x45 => Instruction::LdBR2(cpu.get_l()),
            0x46 => Instruction::LdBR2(mmu.read_byte(cpu.get_hl())),
    
            0x48 => Instruction::LdCR2(cpu.get_b()),
            0x49 => Instruction::LdCR2(cpu.get_c()),
            0x4A => Instruction::LdCR2(cpu.get_d()),
            0x4B => Instruction::LdCR2(cpu.get_e()),
            0x4C => Instruction::LdCR2(cpu.get_h()),
            0x4D => Instruction::LdCR2(cpu.get_l()),
            0x4E => Instruction::LdCR2(mmu.read_byte(cpu.get_hl())),
    
            0x50 => Instruction::LdDR2(cpu.get_b()),
            0x51 => Instruction::LdDR2(cpu.get_c()),
            0x52 => Instruction::LdDR2(cpu.get_d()),
            0x53 => Instruction::LdDR2(cpu.get_e()),
            0x54 => Instruction::LdDR2(cpu.get_h()),
            0x55 => Instruction::LdDR2(cpu.get_l()),
            0x56 => Instruction::LdDR2(mmu.read_byte(cpu.get_hl())),
    
            0x58 => Instruction::LdER2(cpu.get_b()),
            0x59 => Instruction::LdER2(cpu.get_c()),
            0x5A => Instruction::LdER2(cpu.get_d()),
            0x5B => Instruction::LdER2(cpu.get_e()),
            0x5C => Instruction::LdER2(cpu.get_h()),
            0x5D => Instruction::LdER2(cpu.get_l()),
            0x5E => Instruction::LdER2(mmu.read_byte(cpu.get_hl())),
    
            0x60 => Instruction::LdHR2(cpu.get_b()),
            0x61 => Instruction::LdHR2(cpu.get_c()),
            0x62 => Instruction::LdHR2(cpu.get_d()),
            0x63 => Instruction::LdHR2(cpu.get_e()),
            0x64 => Instruction::LdHR2(cpu.get_h()),
            0x65 => Instruction::LdHR2(cpu.get_l()),
            0x66 => Instruction::LdHR2(mmu.read_byte(cpu.get_hl())),
    
            0x68 => Instruction::LdLR2(cpu.get_b()),
            0x69 => Instruction::LdLR2(cpu.get_c()),
            0x6A => Instruction::LdLR2(cpu.get_d()),
            0x6B => Instruction::LdLR2(cpu.get_e()),
            0x6C => Instruction::LdLR2(cpu.get_h()),
            0x6D => Instruction::LdLR2(cpu.get_l()),
            0x6E => Instruction::LdLR2(mmu.read_byte(cpu.get_hl())),
    
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
            0x77 => Instruction::Ldn16A(cpu.get_hl(), cpu.get_a()),
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

            // 4. LDHL SP,n
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
            0x9F => Instruction::SubN(cpu.get_a()),
            0x98 => Instruction::SubN(cpu.get_b()),
            0x99 => Instruction::SubN(cpu.get_c()),
            0x9A => Instruction::SubN(cpu.get_d()),
            0x9B => Instruction::SubN(cpu.get_e()),
            0x9C => Instruction::SubN(cpu.get_h()),
            0x9D => Instruction::SubN(cpu.get_l()),
            0x9E => Instruction::SubHl(mmu.read_byte(cpu.get_hl())),
            0xDE => Instruction::SubD8(n1 as u8),

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
            0x3C => Instruction::IncN(0),
            0x04 => Instruction::IncN(1),
            0x0C => Instruction::IncN(2),
            0x14 => Instruction::IncN(3),
            0x1C => Instruction::IncN(4),
            0x24 => Instruction::IncN(5),
            0x2C => Instruction::IncN(6),
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

            // 3. CPL
            // Description:  Complement A register. (Flip all bits.)
            0x2F => Instruction::Cpl(cpu.get_a()),

            // 5. SCF
            // Description:  Set Carry flag
            0x37 => Instruction::Scf,

            // 6. NOP
            // Description:  No operation.
            0x00 => Instruction::Nop,


    
            // rotate and shift
            // 1. RLCADescription:
            // Rotate A left. Old bit 7 to Carry flag.
            0x07 => Instruction::Rlca(cpu.get_a()),

            // 2. RLA
            // Description: Rotate A left through Carry flag.
            0x17 => Instruction::Rla(cpu.get_a()),
    
            // jumps

            // 4. JR n
            // Description: Add n to current address and jump to it
            0x18 => Instruction::JrN(n1 as i8),

            //5. JR cc,n
            // Description: If following condition is true then add n to current address and jump to it:
            0x20 => Instruction::JrZ(n1 as i8, false),
            0x28 => Instruction::JrZ(n1 as i8, true),
            0x30 => Instruction::JrC(n1 as i8, false),
            0x38 => Instruction::JrC(n1 as i8, true),

            // calls
            // 1. CALL nn
            // Description: Push address of next instruction onto stack and then  jump to address nn
            0xCD => Instruction::Callnn(d16),

            //returns
            // 1. RET
            // Description: Pop two bytes from stack & jump to that address
            0xC9 => Instruction::Ret(cpu.get_sp()),
    
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
            // 6. RL n
            // Description:  Rotate n left through Carry flag.
            0x17 => Instruction::RlN(0, cpu.get_a()),
            0x10 => Instruction::RlN(1, cpu.get_b()),
            0x11 => Instruction::RlN(2, cpu.get_c()),
            0x12 => Instruction::RlN(3, cpu.get_d()),
            0x13 => Instruction::RlN(4, cpu.get_e()),
            0x14 => Instruction::RlN(5, cpu.get_h()),
            0x15 => Instruction::RlN(6, cpu.get_l()),
            0x16 => Instruction::RlHl(mmu.read_byte(cpu.get_hl())),

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

            // bit 1
            0x4F => Instruction::BitbR(0b00000010, cpu.get_a()),
            0x48 => Instruction::BitbR(0b00000010, cpu.get_b()),
            0x49 => Instruction::BitbR(0b00000010, cpu.get_c()),
            0x4A => Instruction::BitbR(0b00000010, cpu.get_d()),
            0x4B => Instruction::BitbR(0b00000010, cpu.get_e()),
            0x4C => Instruction::BitbR(0b00000010, cpu.get_h()),
            0x4D => Instruction::BitbR(0b00000010, cpu.get_l()),

            // bit 2
            0x57 => Instruction::BitbR(0b00000100, cpu.get_a()),
            0x50 => Instruction::BitbR(0b00000100, cpu.get_b()),
            0x51 => Instruction::BitbR(0b00000100, cpu.get_c()),
            0x52 => Instruction::BitbR(0b00000100, cpu.get_d()),
            0x53 => Instruction::BitbR(0b00000100, cpu.get_e()),
            0x54 => Instruction::BitbR(0b00000100, cpu.get_h()),
            0x55 => Instruction::BitbR(0b00000100, cpu.get_l()),

            // bit 3
            0x5F => Instruction::BitbR(0b00001000, cpu.get_a()),
            0x58 => Instruction::BitbR(0b00001000, cpu.get_b()),
            0x59 => Instruction::BitbR(0b00001000, cpu.get_c()),
            0x5A => Instruction::BitbR(0b00001000, cpu.get_d()),
            0x5B => Instruction::BitbR(0b00001000, cpu.get_e()),
            0x5C => Instruction::BitbR(0b00001000, cpu.get_h()),
            0x5D => Instruction::BitbR(0b00001000, cpu.get_l()),

            // bit 4
            0x67 => Instruction::BitbR(0b00010000, cpu.get_a()),
            0x60 => Instruction::BitbR(0b00010000, cpu.get_b()),
            0x61 => Instruction::BitbR(0b00010000, cpu.get_c()),
            0x62 => Instruction::BitbR(0b00010000, cpu.get_d()),
            0x63 => Instruction::BitbR(0b00010000, cpu.get_e()),
            0x64 => Instruction::BitbR(0b00010000, cpu.get_h()),
            0x65 => Instruction::BitbR(0b00010000, cpu.get_l()),

            // bit 5
            0x6F => Instruction::BitbR(0b00100000, cpu.get_a()),
            0x68 => Instruction::BitbR(0b00100000, cpu.get_b()),
            0x69 => Instruction::BitbR(0b00100000, cpu.get_c()),
            0x6A => Instruction::BitbR(0b00100000, cpu.get_d()),
            0x6B => Instruction::BitbR(0b00100000, cpu.get_e()),
            0x6C => Instruction::BitbR(0b00100000, cpu.get_h()),
            0x6D => Instruction::BitbR(0b00100000, cpu.get_l()),

            // bit 6
            0x77 => Instruction::BitbR(0b01000000, cpu.get_a()),
            0x70 => Instruction::BitbR(0b01000000, cpu.get_b()),
            0x71 => Instruction::BitbR(0b01000000, cpu.get_c()),
            0x72 => Instruction::BitbR(0b01000000, cpu.get_d()),
            0x73 => Instruction::BitbR(0b01000000, cpu.get_e()),
            0x74 => Instruction::BitbR(0b01000000, cpu.get_h()),
            0x75 => Instruction::BitbR(0b01000000, cpu.get_l()),

            // bit 7
            0x7F => Instruction::BitbR(0b10000000, cpu.get_a()),
            0x78 => Instruction::BitbR(0b10000000, cpu.get_b()),
            0x79 => Instruction::BitbR(0b10000000, cpu.get_c()),
            0x7A => Instruction::BitbR(0b10000000, cpu.get_d()),
            0x7B => Instruction::BitbR(0b10000000, cpu.get_e()),
            0x7C => Instruction::BitbR(0b10000000, cpu.get_h()),
            0x7D => Instruction::BitbR(0b10000000, cpu.get_l()),
            
            _ => panic!(
                "Unreconized cb byte {:#X} on pc {:#X}\n CPU STATE: {:?}",
                byte, cpu.get_pc(), cpu
            ),
        }
    }
}
