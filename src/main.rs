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
    //roms/mem_timing/01-read_timing.gb
    //roms/11-op a,(hl).gb Passed!
    //roms/10-bit ops.gb Passed!
    //roms/09-op r,r.gb Passed!
    //roms/08-misc instrs.gb Passed!
    //roms/07-jr,jp,call,ret,rst.gb Passed!
    //roms/06-ld r,r.gb Passed!
    //roms/05-op rp.gb Passed!
    //roms/cpu_instr/04-op r,imm.gb Passed!
    //roms/03-op sp,hl.gb Passed!
    //roms/cpu_instr/02-interrupts.gb Passed!
    //roms/01-special.gb Passed!
    let mut f = File::open("roms/cpu_instr/04-op r,imm.gb").unwrap_or_else(|error| {
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

    
    let mut c = 0;
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
        // && com.get_l() == 0x3A && com.get_a() == 0x12


        /* let boo: bool = com.get_pc() == 0xDEF8 && com.get_l() == 0xF4 && com.get_a() == 0x01;
        //let boo: bool = com.get_pc() == 0xC6C7;
        // com.get_pc() == 0xC5FA && com.get_l() == 0xD1 && com.get_a() == 0x00
        

        if (c == 0 && boo) || (com.get_pc() < 0xC000 && com.get_pc() > 0x300) || com.get_pc() >= 0xDF00 {
            com.do_cycle(&mut mem, &mut dbg);
            com.do_cycle(&mut mem, &mut dbg);
            com.do_cycle(&mut mem, &mut dbg);
            com.do_cycle(&mut mem, &mut dbg);
            com.do_cycle(&mut mem, &mut dbg);
            com.do_cycle(&mut mem, &mut dbg);
            //println!("c: {}", c);
            break;
        } else {
            if boo {
                c += 1;
                //println!("c: {}", c);
            }
            com.do_cycle(&mut mem, &mut dbg);
        } */
    }
}