extern crate sdl2;

mod operations;
mod chip8_core;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use std::fs;

fn main() {
    // Init machine state
    let mut chip8_state = chip8_core::State {
        ram: [0; 4096],
        screen: [[false; 64]; 32],
        program_counter: 512,
        index: 0,
        stack: [0; 64],
        stack_pointer: 0,
        delay_timer: 0,
        sound_timer: 0,
        v_buffer: [0; 16],
        screen_width: 640,
        screen_height: 320
    };
    // Load fonts into memory
    init_fonts(&mut chip8_state.ram);

    // Load rom into memory
    let contents: Vec<u8> = fs::read("./roms/4-flags.ch8").expect("no file found");
    let mut load_index: usize = 512;
    for element in contents {
        chip8_state.ram[load_index] = element;
        load_index+=1;
    }

    // Init sdl window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip-8 Rust", chip8_state.screen_width, chip8_state.screen_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        println!("Program counter: {}", chip8_state.program_counter);

        let draw: bool = operations::operate(&mut chip8_state, &mut event_pump);
        let mut rects: Vec<Rect> = Vec::new();
        for i in 0..32 {
            for j in 0..64 {
                if chip8_state.screen[i][j] {
                    rects.push(Rect::new((j * 10) as i32, (i * 10) as i32, 10, 10));
                }
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rects(&rects).expect("Error");
        if draw {
            println!("draw");
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 700));
    }

}

fn init_fonts(ram: &mut [u8; 4096]) {
    let fonts: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
    for i in 0..80 {
        ram[i] = fonts[i]
    }
}