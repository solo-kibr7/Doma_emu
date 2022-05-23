#![allow(dead_code)]
use std::fs::File;
use std::io::Read;

//mod mmu;
//use mmu::MMU;
use crate::mmu::MMU;
use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::decoder::Decoder;

mod mmu;
mod cpu;
mod instruction;
mod decoder;


pub struct Gameboy {

}


fn main() {
    //println!("Hello, gameboy!");
    let mut f = File::open("roms/dmg_boot.bin").unwrap_or_else(|error| {
        panic!("Problem opening the file: {:?}", error);
    });

    let mut rom_file = Vec::<u8>::new();
    f.read_to_end(&mut rom_file);

    let mut mem = MMU::new();
    let mut com = CPU::new();

    // put rom into memory
    mem.from_rom_file(&rom_file);

    println!("first byte at 0000 is {:#X}", mem.read_byte(0x0000) as u16);

    loop {
        com.run_instruction(&mut mem);
    }
}