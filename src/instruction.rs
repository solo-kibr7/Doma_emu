pub use crate::cpu::CPU;

#[derive(Debug)]
pub enum Instruction {

    //8 bit loads
    LdN(u8, u8),

    LdAR2(u8, u8),
    LdAnn(u8),
    LdAd8(u8),
    LdBR2(u8, bool),
    LdCR2(u8, bool),
    LdDR2(u8, bool),
    LdER2(u8, bool),
    LdHR2(u8, bool),
    LdLR2(u8, bool),
    LdHlR2(u8, bool),

    LdnA(u8, u8),
    Ldn16A(u16, u8),
    Lda16A(u16, u8),

    LdAc(u8),
    LdCa(u8),

    LddAhl,
    LddAHl(u16),
    LddHlA(u16),
    LdIAHl(u16),
    LdIHlA(u16),

    LdHnA(u8),
    LdHAn(u8),

    //16 bit loads
    LdBc(u16),
    LdDe(u16),
    LdHl(u16),
    LdSp(u16),
    LdSpHl(u16),
    LdHlSp(i8),
    LdnnSp(u16),
    Pushnn(u16),
    Popnn(u8),

    //8 bit ALU
    AddN(u8),
    AddHl(u8),
    AddD8(u8),
    AdcN(u8),
    AdcHl(u8),
    AdcD8(u8),
    SubN(u8),
    SubHl(u8),
    SubD8(u8),
    SbcN(u8),
    SbcHl(u8),
    SbcD8(u8),
    Andn(u8),
    AndHl(u8),
    AndD8(u8),
    OrN(u8),
    OrHl(u8),
    OrD8(u8),

    Xor(u8),
    XorHl(u8),
    XorD8(u8),

    Cp(u8),
    CpHl(u8),
    CpD8(u8),

    IncN(u8, u8),
    IncHl(u16),

    DecN(u8, u8),
    DecHl(u16),

    // 16bit Arithmetic
    AddHlN(u16),
    AddSpN(i8),
    IncNN(u8, u16),
    DecNN(u8, u16),

    // Miscellaneous
    SwapN(u8, u8),
    SwapHl(u8),
    Daa,
    Cpl(u8),
    Ccf,
    Scf,
    Nop,
    Halt,
    Stop,
    Di,
    Ei,

    //Rotates and Shifts
    Rlca(u8),
    Rla(u8),
    Rrca(u8),
    Rra(u8),
    RlcN(u8, u8),
    RlN(u8, u8),
    RrcN(u8, u8),
    RrN(u8, u8),
    SlaN(u8, u8),
    SraN(u8, u8),
    SrlN(u8, u8),

    //bit opcodes
    BitbR(u8, u8),
    BitbHl(u8, u8),
    SetbR(u8, u8, u8),
    ResbR(u8, u8, u8),

    //jumps
    Jpnn(u16),
    Jpcc(u16, u8),
    JpHl(u16),
    JrN(i8),
    Jrcc(i8, u8),

    //calls
    Callnn(u16),
    Callcc(u16, u8),

    //restarts
    Rst(u8),

    //returns
    Ret,
    Retcc(u8),
    Reti,

}