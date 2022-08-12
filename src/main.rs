#![allow(dead_code)]
use std::fs::File;
use std::io::Read;
use std::time::{Instant, Duration};
use std::thread;
use std::iter::Iterator;
//use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use minifb::{CursorStyle, MouseMode, Scale, Key, KeyRepeat, Window, WindowOptions};
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
use crate::ppu::{PPU, lcd::state_machine};
use crate::timer::Timer;
use crate::dbg::DBG; 
use crate::cartridge::Cartridge; 
use crate::joypad::JoypadButtons;

mod mmu;
mod cpu;
mod instruction;
mod ppu;
mod timer;
mod dbg;
mod cartridge;
mod joypad;

//160 x 144
pub const SCALE: usize = 2;

pub const XRES: usize = 160;
pub const YRES: usize = 144;

pub const GAMEBOY_WIDTH: usize = XRES * SCALE;
pub const GAMEBOY_HEIGHT: usize = YRES * SCALE;

pub const SCREEN_WIDTH: usize = 144 * SCALE;
pub const SCREEN_HEIGHT: usize = 216 * SCALE;

pub const TILE_COLORS: [u32; 4] = [0xFFFFFF, 0xAAAAAA, 0x555555, 0x000000];

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
        Rect {
            x,
            y,
            width,
            height,
            color,
        }
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
            fill_rect(dest, &rec, false);
        }
    }
}

fn fill_rect(dest: &mut [u32], rect: &Rect, gameboy: bool) {
    for y in 0..rect.height {
        for x in 0..rect.width {
            //if rect.color == 0x0 {println!("rect_color2:{}", rect.color)};
            if gameboy {
                dest[((rect.y + y) * GAMEBOY_WIDTH) + rect.x + x] = rect.color;
            } else {
                dest[((rect.y + y) * SCREEN_WIDTH) + rect.x + x] = rect.color;
            }
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

fn update_gameboy_window(mmu: &MMU, dest: &mut [u32]) {
    for line_num in 0..YRES {
        for x in 0..XRES {
            let color = mmu.ppu.video_buffer[x + line_num * XRES];
            let rect = Rect::new(x * SCALE, line_num * SCALE, SCALE, SCALE, color);
            //if line_num * SCALE > 270 {println!("rect x:{}, y:{}, h:{}, w:{}, color:{:#X}", rect.x, rect.y, rect.height, rect.width, rect.color)};
            //if recta.color != 0xFFFFFF {println!("rect_color1:{}, color:{}", recta.color, color)};
            fill_rect(dest, &rect, true);
        }
    }
}

fn press_keys(mmu: &mut MMU, k: Key, pressed: bool, released: bool) {
    if pressed {
        match k {
            Key::W => mmu.joypad.press_joypad(JoypadButtons::Up),
            Key::A => mmu.joypad.press_joypad(JoypadButtons::Left),
            Key::S => mmu.joypad.press_joypad(JoypadButtons::Down),
            Key::D => mmu.joypad.press_joypad(JoypadButtons::Right),
            Key::Comma => mmu.joypad.press_joypad(JoypadButtons::B),
            Key::Period => mmu.joypad.press_joypad(JoypadButtons::A),
            Key::Enter => mmu.joypad.press_joypad(JoypadButtons::Start),
            Key::RightShift => mmu.joypad.press_joypad(JoypadButtons::Select),
            _ => (),
        }
    } 
    if released {
        match k {
            Key::W => mmu.joypad.release_joypad(JoypadButtons::Up),
            Key::A => mmu.joypad.release_joypad(JoypadButtons::Left),
            Key::S => mmu.joypad.release_joypad(JoypadButtons::Down),
            Key::D => mmu.joypad.release_joypad(JoypadButtons::Right),
            Key::Comma => mmu.joypad.release_joypad(JoypadButtons::B),
            Key::Period => mmu.joypad.release_joypad(JoypadButtons::A),
            Key::Enter => mmu.joypad.release_joypad(JoypadButtons::Start),
            Key::RightShift => mmu.joypad.release_joypad(JoypadButtons::Select),
            _ => (),
        }
    }    
}

//pub static now:Instant = Instant::now();
const TARGET_FRAME_TIME:u32 = 17 as u32; // 1000/60 is about 16.667


fn main() {
    //println!("Hello, gameboy!");
    //roms/dmg_boot.bin
    //roms/mem_timing/01-read_timing.gb
    //roms/cpu_instr/01-special.gb Passed!
    //roms/dmg-acid2.gb
    //roms/drmario.gb
    //roms/instr_timing.gb
    //roms/ppu-acceptance/hblank_ly_scx_timing-GS.gb
    //roms/ppu-acceptance/intr_2_0_timing.gb
    let mut f = File::open("roms/drmario.gb").unwrap_or_else(|error| {
        panic!("Problem opening the file: {:?}", error);
    });

    let mut bootfile = File::open("roms/dmg_boot.bin").unwrap_or_else(|error| {
        panic!("Problem opening the file: {:?}", error);
    });

    let mut rom_file = Vec::<u8>::new();
    let mut boot_file = Vec::<u8>::new();
    f.read_to_end(&mut rom_file);
    bootfile.read_to_end(&mut boot_file);

    let mut mem = MMU::default();
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

    dbg_window.set_position(500, 100);
    // Limit to max ~60 fps update rate
    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut gameboy_buffer: Vec<u32> = vec![0; GAMEBOY_WIDTH * GAMEBOY_HEIGHT];
    let mut dbg_buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT* SCALE * SCALE];

    

    // put rom into memory
    mem.cartridge.from_rom_file(&rom_file);
    //mem.cartridge.from_boot_file(&boot_file);

    //println!("first byte at 0000 is {:#X}", mem.read_byte(0x0000) as u16);
    //let rom_types: [String] = ["str", "nini"];

    
    //let mut c = 0;
    //'running: loop
    let mut prev_frame: u32 = 0;
    let mut prev_time: u128 = 0;
    let mut start_timer: u128 = 0;
    let mut frame_count: u64 = 0;

    let mut gran: u8 = 0;
    let gran_length = 10;


    let now:Instant = Instant::now();
    //let mut p = 0;
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
        //let mut p = 0;
        //dbg_buffer.len() 
        let key_array = [Key::W, Key::A, Key::S, Key::D, Key::Comma, Key::Period, Key::Enter, Key::RightShift];
        for k in key_array {
            press_keys(&mut mem, k, gameboy_window.is_key_pressed(k, KeyRepeat::Yes), gameboy_window.is_key_released(k));
            /* if gameboy_window.is_key_pressed(k, KeyRepeat::Yes) {
                println!("key:{:?}", k);
            }
            if gameboy_window.is_key_released(k) {
                println!("key2:{:?}", k);
            } */
        }
        for i in 0..gameboy_buffer.len() {
            //update_dbg_window(&mem, &mut buffer);
            /* gameboy_window.get_keys_pressed(KeyRepeat::No).map(|keys| {
                for t in keys {
                    match t {
                        Key::W => println!("pressed w"),
                        Key::T => println!("pressed t"),
                        _ => (),
                    }
                }
            }); */
            
            com.do_cycle(&mut mem, &mut dbg); 
            //state_machine::current_time(now.elapsed().as_millis());
            //mem.ppu.fps.write_current_time
            /* println!("prev:{}, current:{}, status:{:#X}", 
                prev_frame, mem.ppu.current_frame, mem.read_byte(0xFF41)); */
            if prev_frame != mem.ppu.current_frame {
                gran += 1;
            }
            if gran == gran_length {
                let end: u128 = now.elapsed().as_millis();
                let frame_time: u128 = end - prev_time;

                if (frame_time) / (gran_length as u128) < TARGET_FRAME_TIME as u128 {
                    thread::sleep(Duration::from_millis((TARGET_FRAME_TIME as u128 * (gran_length as u128) - frame_time) as u64));
                }

                if (end - start_timer) / (gran_length as u128) >= 1000 {
                    let fps = frame_count;
                    start_timer = end;
                    frame_count = 0;
    
                    println!("FPS: {}", fps);
                }
                frame_count += 1;
                gran = 0;

                prev_time = end;

                update_gameboy_window(&mem, &mut gameboy_buffer);
                if dbg_window.is_open() && !dbg_window.is_key_down(Key::Escape) { 
                    update_dbg_window(&mem, &mut dbg_buffer);
                }
            }
            prev_frame = mem.ppu.current_frame;
        }
        /* for i in 0..gameboy_buffer.len() {
            update_gameboy_window(&mem, &mut gameboy_buffer);
        } */
        

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

        //let buffer_index = mem.ppu.pixel_fifo.push_x as usize + mem.ppu.lcd.ly as usize * XRES;
                
        /* if buffer_index >= 23000 {
            if buffer_index == 23000 {p += 1;}
            println!("buffer_index:{}, push_x:{}, ly:{}, scroll_x:{}, line_ticks:{}, mode:{}, fetch_x:{}, p:{}",
                buffer_index, self.pixel_fifo.push_x, self.lcd.ly, self.lcd.scroll_x,
                self.line_ticks, self.lcd.lcd_status.current_mode(), self.pixel_fifo.fetch_x, p);
            if p == 2 {panic!("pp");}
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