#![feature(duration_checked_ops)]
extern crate minifb;
extern crate rustpusher;

use std::env::args;
use std::thread::sleep;
use std::time::{Duration, Instant};
use minifb::*;
use rustpusher::*;

const PAGE: usize = 0x100;
const BANK: usize = PAGE * PAGE;

fn main() {
    let file = args().nth(1).expect("No filename given.");
    let mut emu = BytePusher::new(&file);
    let mut window = Window::new("BytePusher Test", PAGE, PAGE, WindowOptions::default())
        .expect("Unable to create window");
    let mut window_buffer: Vec<u32> = vec![0; PAGE * PAGE];

    loop {
        if !window.is_open() || window.is_key_down(Key::Escape) {
            break;
        }
        let timer = Instant::now();

        emu.process_input(get_input(&window));
        emu.frame();

        // TODO: properly refactor
        let offset = emu.ram[5] as usize * BANK;
        for (i, pixel) in window_buffer.iter_mut().enumerate() {
            *pixel = color_from_palette(emu.ram[offset + i]);
        }

        window.update_with_buffer(&window_buffer);

        if let Some(value) = Duration::new(0, 16666666).checked_sub(timer.elapsed()) {
            sleep(value);
        }
    }
}

fn get_input(ref window: &Window) -> (u8, u8) {
    let mut input = (0u8, 0u8);
    window.get_keys().map(|keys| for k in keys {
        match k {
            Key::X => input.1 |= 0b00000001,

            Key::Key1 => input.1 |= 0b00000010,
            Key::Key2 => input.1 |= 0b00000100,
            Key::Key3 => input.1 |= 0b00001000,

            Key::Q => input.1 |= 0b00010000,
            Key::W => input.1 |= 0b00100000,
            Key::E => input.1 |= 0b01000000,

            Key::A => input.1 |= 0b10000000,
            Key::S => input.0 |= 0b00000001,
            Key::D => input.0 |= 0b00000010,

            Key::Z => input.0 |= 0b00000100,
            Key::C => input.0 |= 0b00001000,

            Key::Key4 => input.0 |= 0b00010000,
            Key::R => input.0 |= 0b00100000,
            Key::F => input.0 |= 0b01000000,
            Key::V => input.0 |= 0b10000000,

            _ => {}
        }
    });
    input
}

fn color_from_palette(index: u8) -> u32 {
    match index {
        0x00...0xd7 => {
            index as u32 / 36 * 0x33 * BANK as u32 + index as u32 / 6 % 6 * 0x33 * PAGE as u32 +
            index as u32 % 6 * 0x33
        }
        _ => 0x000000,
    }
}
