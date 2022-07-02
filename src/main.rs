#![allow(dead_code)]
use std::fs::File;
use std::io::Read;
//use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use minifb::{CursorStyle, MouseMode, Scale, Key, Window, WindowOptions};
/* use sdl2::event::Event;
use sdl2::render;
//use sdl2::pixels::Color::RGB;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use std::time::Duration; */

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

//160 x 144
pub const SCALE: usize = 2;

pub const GAMEBOY_WIDTH: usize = 160 * SCALE;
pub const GAMEBOY_HEIGHT: usize = 144 * SCALE;

pub const SCREEN_WIDTH: usize = 144 * SCALE;
pub const SCREEN_HEIGHT: usize = 216 * SCALE;

pub const TILE_COLORS: [u32; 4] = [0xFFFFFF, 0x555555, 0xAAAAAA, 0x000000];

pub struct Gameboy {

}

struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
}

impl Rect {
    pub fn new(x: usize, y: usize, width:usize, height: usize, color: u32) -> Rect {
        let mut rect = Rect {
            x,
            y,
            width,
            height,
            color,
        };
        rect
    }
}

fn display_tile(mmu: &MMU, dest: &mut [u32], start_address:u16, tile_num:u16, x:u32, y:u32) {
    for tile_y in 0..=7 {
        let b1:u8 = mmu.read_byte(start_address + (tile_num * 16) + tile_y * 2);
        let b2:u8 = mmu.read_byte(start_address + (tile_num * 16) + tile_y * 2 + 1);
        //println!("hey");
        for bit in (0..=7).rev() {
            //println!("hey2");
            let hi:bool = (b1 & (1 << bit)) != 0;
            let low:bool = b2 & (1 << bit) != 0;

            //println!("hey");
            let mut color:u8 = 0x0;
            if hi {color |= 0b10;}
            if low {color |= 0b01;}
            
            

            let rec_x = x as usize + ((7 - bit) * SCALE);
            let rec_y = y as usize + (tile_y as usize * SCALE);
            let rec_w = SCALE;
            let rec_h = SCALE;

            /* if color > 0 {
                println!("b1:{:#X}, b2:{:#X}, rec_x:{}, rec_y:{}, w:{}, h:{}, color:{:#X}", b1, b2, rec_x, rec_y, rec_w, rec_h, color);
            } */
            let rec = Rect::new(rec_x, rec_y, rec_w, rec_h, TILE_COLORS[color as usize]);
            //println!("b1:{:#X}, b2:{:#X}, rec_x:{}, rec_y:{}, w:{}, h:{}, color:{:#X}", b1, b2, rec_x, rec_y, rec_w, rec_h, color);
            fill_rect(dest, &rec);
        }
    }
}

fn fill_rect(dest: &mut [u32], rect: &Rect) {
    for y in 0..rect.height {
        for x in 0..rect.width {
            dest[((rect.y + y) * SCREEN_WIDTH) + rect.x + x] = rect.color;
        }
    }
}

fn update_dbg_window(mmu: &MMU, dest: &mut [u32]) {
    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_num = 0;

    let addr:u16 = 0x8000;

    //384 tiles, 24 x 16
    for y in 0..24 {
        for x in 0..16 {
            //println!("x_draw:{}, x:{}, y_draw:{}, y:{}", x_draw, x, y_draw, y);
            display_tile(mmu, dest, addr, tile_num, x_draw + (x * SCALE) as u32, y_draw + (y * SCALE) as u32);
            x_draw += (8 * SCALE) as u32;
            tile_num += 1;
        }

        y_draw += (8 * SCALE) as u32;
        x_draw = 0;
    }

}


fn main() {
    //println!("Hello, gameboy!");
    //roms/dmg_boot.bin
    //roms/mem_timing/01-read_timing.gb
    //roms/01-special.gb Passed!
    let mut f = File::open("roms/dmg-acid2.gb").unwrap_or_else(|error| {
        panic!("Problem opening the file: {:?}", error);
    });

    let mut rom_file = Vec::<u8>::new();
    f.read_to_end(&mut rom_file);

    let mut mem = MMU::new();
    let mut com = CPU::default();
    let mut dbg = DBG::default();

    let mut gameboy_window = Window::new(
        "Gameboy - ESC to exit",
        GAMEBOY_WIDTH,
        GAMEBOY_HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()},
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut dbg_window = Window::new(
        "Debug - ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()},
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit to max ~60 fps update rate
    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut gameboy_buffer: Vec<u32> = vec![0; GAMEBOY_WIDTH * GAMEBOY_HEIGHT* SCALE * SCALE];
    let mut dbg_buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT* SCALE * SCALE];

    

    // put rom into memory
    mem.from_rom_file(&rom_file);

    println!("first byte at 0000 is {:#X}", mem.read_byte(0x0000) as u16);
    //let rom_types: [String] = ["str", "nini"];

    
    //let mut c = 0;
    //'running: loop
    while gameboy_window.is_open() && !gameboy_window.is_key_down(Key::Escape) {

        /* for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        } */
        let mut p = 0;
        for i in 0..dbg_buffer.len() {
            //update_dbg_window(&mem, &mut buffer);
            com.do_cycle(&mut mem, &mut dbg); // write something more funny here!
            if p == 5000 && dbg_window.is_open() && !dbg_window.is_key_down(Key::Escape) {
                update_dbg_window(&mem, &mut dbg_buffer);
                p = 0;
            } else {
                p += 1;
            }
        }
        

        /* let boo: bool = com.get_pc() == 0xDEF8 && com.get_l() == 0xF4 && com.get_a() == 0x01;

        if (c == 0 && boo) || (com.get_pc() < 0xC000 && com.get_pc() > 0x300) || com.get_pc() >= 0xDF00 {
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
        gameboy_window
            .update_with_buffer(&gameboy_buffer, GAMEBOY_WIDTH, GAMEBOY_HEIGHT)
            .unwrap();

        dbg_window
            .update_with_buffer(&dbg_buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();

        /* canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); */
    }
}