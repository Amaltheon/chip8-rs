use crate::chip8_core::State;
use sdl2::event::Event;
use sdl2::{ keyboard::{Keycode, Scancode}, EventPump};

pub fn operate(state: &mut State, event_pump: &mut EventPump) -> bool {
    let first_byte: u8 = state.ram[state.program_counter as usize];
    let last_byte: u8 = state.ram[(state.program_counter + 1) as usize];
    let opcode: u16 = ((first_byte as u16) << 8) | last_byte as u16;
    println!("opcode: {:#06x}", opcode);
    let op: u16 = (opcode & 0xF000) >> 12;
    let x: u16 = (opcode & 0x0F00) >> 8;
    let y: u16 = (opcode & 0x00F0) >> 4;
    let n: u16 = opcode & 0x000F;
    let nn: u16 = opcode & 0x00FF;
    let nnn: u16 = opcode & 0x0FFF;
    let mut draw: bool = false;
    match op {
        0x0 => match n {
            0x0 => clear_screen(state),
            0xE => ret(state),
            _ => invalid_opcode(opcode)
        },
        0x1 => jump(state, nnn),
        0x2 => subroutine(state, nnn),
        0x3 => skip_3xnn(state, x, nn),
        0x4 => skip_4xnn(state, x, nn),
        0x5 => skip_5xy0(state, x, y),
        0x6 => set_6xnn(state, x, nn),
        0x7 => add_7xnn(state, x, nn),
        0x8 => match n {
            0x0 => set_8xy0(state, x, y),
            0x1 => or_8xy1(state, x, y),
            0x2 => and_8xy2(state, x, y),
            0x3 => xor_8xy3(state, x, y),
            0x4 => add_8xy4(state, x, y),
            0x5 => sub_8xy5(state, x, y),
            0x6 => shift_8xy6(state, x, y),
            0x7 => sub_8xy7(state, x, y),
            0xE => shift_8xye(state, x, y),
            _ => invalid_opcode(opcode)
        }
        0x9 => skip_9xy0(state, x, y),
        0xA => set_index_annn(state, nnn),
        0xB => jump_bnnn(state, nnn),
        0xC => rand_cxnn(state, x, nn),
        0xD => {draw_dxyn(state, x, y, n); draw = true;},
        0xE => match nn {
            0x9E => skip_key_ex9e(state, event_pump, x),
            0xA1 => skip_key_exa1(state, event_pump, x),
            _ => invalid_opcode(opcode)
        },
        0xF => match nn {
            0x07 => timer_fx07(state, x),
            0x15 => timer_fx15(state, x),
            0x18 => timer_fx18(state, x),
            0x0A => get_key_fx0a(state, x, event_pump),
            0x29 => get_char_fx29(state, x),
            0x33 => bcd_fx33(state, x),
            0x55 => store_load_fx55(state, x),
            0x65 => store_load_fx65(state, x),
            0x1E => add_ind_fx1e(state, x),

            _ => invalid_opcode(opcode)
        },
        _ => invalid_opcode(opcode)
    }
    state.program_counter+=2;
    return draw
}

// Invalid opcode
fn invalid_opcode(opcode: u16) {
    println!("Invalid Opcode: {:#06x}", opcode);
}

// 00E0 
fn clear_screen(state: &mut State) {
    println!("clear_screen");
    state.screen = [[false; 64]; 32];
}

// 00EE
fn ret(state: &mut State) {
    println!("ret");
    state.program_counter = state.stack[state.stack_pointer as usize];
    state.stack_pointer-=1;
}

// 1NNN
fn jump(state: &mut State, nnn: u16) {
    println!("jump");
    state.program_counter = nnn;
    state.program_counter-=2;
}

// 2NNN
fn subroutine(state: &mut State, nnn: u16) {
    println!("subroutine");
    state.stack_pointer+=1;
    state.stack[state.stack_pointer as usize] = state.program_counter;
    state.program_counter = nnn;
    state.program_counter-=2;
}

// 3XNN
fn skip_3xnn(state: &mut State, x: u16, nn: u16) {
    if state.v_buffer[x as usize] == nn as u8 {
        state.program_counter+=2;
    }
}

// 4XNN
fn skip_4xnn(state: &mut State, x: u16, nn: u16) {
    if state.v_buffer[x as usize] != nn as u8 {
        state.program_counter+=2;
    }
}

// 5XY0
fn skip_5xy0(state: &mut State, x: u16, y: u16) {
    if state.v_buffer[x as usize] == state.v_buffer[y as usize] {
        state.program_counter+=2;
    }
}

// 6XNN
fn set_6xnn(state: &mut State, x: u16, nn: u16) {
    state.v_buffer[x as usize] = nn as u8;
}

// 7XNN
fn add_7xnn(state: &mut State, x: u16, nn: u16) {
    state.v_buffer[x as usize] = state.v_buffer[x as usize].wrapping_add(nn as u8);
}

// 8XY0
fn set_8xy0(state: &mut State, x: u16, y: u16) {
    state.v_buffer[x as usize] = state.v_buffer[y as usize];
}

// 8XY1
fn or_8xy1(state: &mut State, x: u16, y: u16) {
    let bin_or: u8 = state.v_buffer[x as usize] | state.v_buffer[y as usize];
    state.v_buffer[x as usize] = bin_or;
}

// 8XY2
fn and_8xy2(state: &mut State, x: u16, y: u16) {
    let bin_and: u8 = state.v_buffer[x as usize] & state.v_buffer[y as usize];
    state.v_buffer[x as usize] = bin_and;
}

// 8XY3
fn xor_8xy3(state: &mut State, x: u16, y: u16) {
    let bin_xor: u8 = state.v_buffer[x as usize] ^ state.v_buffer[y as usize];
    state.v_buffer[x as usize] = bin_xor;
}

// 8XY4
fn add_8xy4(state: &mut State, x: u16, y: u16) {
    if (state.v_buffer[x as usize] as u16 + state.v_buffer[y as usize] as u16) > 255 {
        state.v_buffer[0xF] = 1;
    }
    else {
        state.v_buffer[0xF] = 0;
    }
    state.v_buffer[x as usize] = state.v_buffer[x as usize].wrapping_add(state.v_buffer[y as usize]);
}

// 8XY5
fn sub_8xy5(state: &mut State, x: u16, y: u16) {
    if state.v_buffer[x as usize] > state.v_buffer[y as usize] {
        state.v_buffer[0xF] = 1;
    }
    else {
        state.v_buffer[0xF] = 0;
    }
    state.v_buffer[x as usize] = state.v_buffer[x as usize].wrapping_sub(state.v_buffer[y as usize]);
}

// 8XY6
fn shift_8xy6(state: &mut State, x: u16, y: u16) {
    let bit: u8 = state.v_buffer[x as usize] & 0b00000001;
    state.v_buffer[x as usize] = state.v_buffer[y as usize];
    state.v_buffer[x as usize] = state.v_buffer[x as usize] >> 1;
    state.v_buffer[0xF] = bit;
}

// 8XY7
fn sub_8xy7(state: &mut State, x: u16, y: u16) {
    if state.v_buffer[x as usize] < state.v_buffer[y as usize] {
        state.v_buffer[0xF] = 1;
    }
    else {
        state.v_buffer[0xF] = 0;
    }
    state.v_buffer[x as usize] = state.v_buffer[y as usize].wrapping_sub(state.v_buffer[x as usize]);
}

// 8XYE
fn shift_8xye(state: &mut State, x: u16, y: u16) {
    let bit: u8 = state.v_buffer[x as usize] & 0b10000000;
    state.v_buffer[x as usize] = state.v_buffer[y as usize];
    state.v_buffer[x as usize] = state.v_buffer[x as usize] << 1;
    state.v_buffer[0xF] = bit / 128;
}

// 9XY0
fn skip_9xy0(state: &mut State, x: u16, y: u16) {
    if state.v_buffer[x as usize] != state.v_buffer[y as usize] {
        state.program_counter+=2;
    }
}

// ANNN
fn set_index_annn(state: &mut State, nnn: u16) {
    state.index = nnn;
}

// BNNN
fn jump_bnnn(state: &mut State, nnn: u16) {
    state.program_counter = state.v_buffer[0xF] as u16 + nnn;
}

// CXNN
fn rand_cxnn(state: &mut State, x: u16, nn: u16) {
    let random_number = rand::random::<u8>();
    state.v_buffer[x as usize] = nn as u8 & random_number;
}

// DXYN
fn draw_dxyn(state: &mut State, x: u16, y: u16, n:u16) {
    let mut x_coord: u8;
    let mut y_coord: u8 = state.v_buffer[y as usize] % 32;
    state.v_buffer[0xF] = 0;

    for i in 0..n {
        if y_coord > 31 {
            break;
        }
        let mut sprite: u8 = state.ram[(state.index + i) as usize];
        x_coord = state.v_buffer[x as usize] % 64;
        for _j in 0..8 {
            if (x_coord) > 63 {
                break;
            }
            if (sprite & 0b10000000) != 0 {
                if state.screen[y_coord as usize][x_coord as usize] {
                    state.screen[y_coord as usize][x_coord as usize] = false;
                    state.v_buffer[0xF] = 1;
                }
                else {
                    state.screen[y_coord as usize][x_coord as usize] = true;
                }
            }
            sprite <<= 1;
            x_coord+=1;
        }
        y_coord+=1;
    }

}

// EX9E
fn skip_key_ex9e(state: &mut State, event_pump: &mut EventPump, x: u16) {
    let key_vx: u8 = state.v_buffer[x as usize];
    let key_code: Scancode = lookup_key(key_vx);
    if event_pump.keyboard_state().is_scancode_pressed(key_code) {
        state.program_counter+=2;
    }
}

// EXA1
fn skip_key_exa1(state: &mut State, event_pump: &mut EventPump, x: u16) {
    let key_vx: u8 = state.v_buffer[x as usize];
    let key_code: Scancode = lookup_key(key_vx);
    if !event_pump.keyboard_state().is_scancode_pressed(key_code) {
        state.program_counter+=2;
    }
}

// FX07
fn timer_fx07(state: &mut State, x: u16) {
    state.v_buffer[x as usize] = state.delay_timer;
}

// FX15
fn timer_fx15(state: &mut State, x: u16) {
    state.delay_timer = state.v_buffer[x as usize];
}

// FX18
fn timer_fx18(state: &mut State, x: u16) {
    state.sound_timer = state.v_buffer[x as usize];
}

// FX1E
fn add_ind_fx1e(state: &mut State, x: u16) {
    state.index = state.index.wrapping_add(state.v_buffer[x as usize] as u16);
}

// FX0A
fn get_key_fx0a(state: &mut State, x: u16, event_pump: &mut EventPump) {
    if event_pump.keyboard_state().pressed_scancodes().count() == 0 {
        state.program_counter-=2;
    }
    else {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Num1), ..} => {state.v_buffer[x as usize] = 0x1},
                Event::KeyDown { keycode: Some(Keycode::Num2), ..} => {state.v_buffer[x as usize] = 0x2},
                Event::KeyDown { keycode: Some(Keycode::Num3), ..} => {state.v_buffer[x as usize] = 0x3},
                Event::KeyDown { keycode: Some(Keycode::Num4), ..} => {state.v_buffer[x as usize] = 0xC},
                Event::KeyDown { keycode: Some(Keycode::Q), ..} => {state.v_buffer[x as usize] = 0x4},
                Event::KeyDown { keycode: Some(Keycode::W), ..} => {state.v_buffer[x as usize] = 0x5},
                Event::KeyDown { keycode: Some(Keycode::E), ..} => {state.v_buffer[x as usize] = 0x6},
                Event::KeyDown { keycode: Some(Keycode::R), ..} => {state.v_buffer[x as usize] = 0xD},
                Event::KeyDown { keycode: Some(Keycode::A), ..} => {state.v_buffer[x as usize] = 0x7},
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {state.v_buffer[x as usize] = 0x8},
                Event::KeyDown { keycode: Some(Keycode::D), ..} => {state.v_buffer[x as usize] = 0x9},
                Event::KeyDown { keycode: Some(Keycode::F), ..} => {state.v_buffer[x as usize] = 0xE},
                Event::KeyDown { keycode: Some(Keycode::Z), ..} => {state.v_buffer[x as usize] = 0xA},
                Event::KeyDown { keycode: Some(Keycode::X), ..} => {state.v_buffer[x as usize] = 0x0},
                Event::KeyDown { keycode: Some(Keycode::C), ..} => {state.v_buffer[x as usize] = 0xB},
                Event::KeyDown { keycode: Some(Keycode::V), ..} => {state.v_buffer[x as usize] = 0xF},
                _ => state.v_buffer[x as usize] = 0
            }
        }
    }
}

// FX29
fn get_char_fx29(state: &mut State, x: u16) {
    let font_char: u8 = state.v_buffer[x as usize];
    state.index = (font_char as u16) * 5;
}

// FX33
fn bcd_fx33(state: &mut State, x: u16) {
    let decimal = state.v_buffer[x as usize];

    let ones: u8 = decimal % 10;
    let tens: u8 = (decimal/10) % 10;
    let hundreds: u8 = (decimal/100) % 10;

    state.ram[state.index as usize] = hundreds;
    state.ram[(state.index + 1) as usize] = tens;
    state.ram[(state.index + 2) as usize] = ones;
}

// FX55
fn store_load_fx55(state: &mut State, x: u16) {
    for i in 0..x+1 {
        state.ram[(state.index + i) as usize] = state.v_buffer[i as usize];
    }
    state.index+=x+1;
}

// FX65
fn store_load_fx65(state: &mut State, x: u16) {
    for i in 0..x+1 {
        state.v_buffer[i as usize] = state.ram[(state.index + i) as usize];
    }
    state.index+=x+1;
}

fn lookup_key(key: u8) -> Scancode {
    match key {
        0x1 => return Scancode::Num1,
        0x2 => return Scancode::Num2,
        0x3 => return Scancode::Num3,
        0xC => return Scancode::Num4,
        0x4 => return Scancode::Q,
        0x5 => return Scancode::W,
        0x6 => return Scancode::E,
        0xD => return Scancode::R,
        0x7 => return Scancode::A,
        0x8 => return Scancode::S,
        0x9 => return Scancode::D,
        0xE => return Scancode::F,
        0xA => return Scancode::Z,
        0x0 => return Scancode::X,
        0xB => return Scancode::C,
        0xF => return Scancode::V,
        _ => return Scancode::Escape
    }
}