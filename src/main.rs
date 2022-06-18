#![allow(dead_code)]
use std::fs::File;
use std::io::Read;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

//mod mmu;
//use mmu::MMU;
use crate::mmu::{MMU, interrupts, serial};
use crate::cpu::CPU;
use crate::instruction::Instruction;
use crate::ppu::PPU;
use crate::timer::Timer;
use crate::dbg::DBG; 
use crate::cart::Cartridge; 

mod mmu;
mod cpu;
mod instruction;
mod ppu;
mod timer;
mod dbg;
mod cart;



pub struct Gameboy {

}


fn main() {
    //println!("Hello, gameboy!");
    //roms/dmg_boot.bin
    //roms/03-op sp,hl.gb
    //roms/02-interrupts.gb
    //roms/01-special.gb
    let mut f = File::open("roms/01-special.gb").unwrap_or_else(|error| {
        panic!("Problem opening the file: {:?}", error);
    });

    let mut rom_file = Vec::<u8>::new();
    f.read_to_end(&mut rom_file);

    let mut mem = MMU::new();
    let mut com = CPU::default();
    let mut dbg = DBG::default();

    // put rom into memory
    mem.from_rom_file(&rom_file);

    println!("first byte at 0000 is {:#X}", mem.read_byte(0x0000) as u16);
    //let rom_types: [String] = ["str", "nini"];

    

    loop {
        /*match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                com.do_cycle(&mut mem, &mut dbg);
                if com.get_pc() > 0xEE00 {
                    panic!("oops");
                }
            },
            _ => (),
        }*/
        
        com.do_cycle(&mut mem, &mut dbg);
        /*if com.get_pc() == 0xC249 {
            com.do_cycle(&mut mem, &mut dbg);
            com.do_cycle(&mut mem, &mut dbg);
            break;
        } else {
            com.do_cycle(&mut mem, &mut dbg);
        }*/
    }
}