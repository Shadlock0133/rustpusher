#![feature(duration_checked_ops)]
extern crate minifb;
extern crate rustpusher;

use std::env::args;
use std::thread::sleep;
use std::time::{Duration, Instant};
use minifb::Key;
use rustpusher::*;

fn main() {
    let file = args().nth(1).expect("No filename given.");
    let mut emu = BytePusher::new(&file);
    emu.update_window();

    loop {
        if !emu.window.is_open() || emu.window.is_key_down(Key::Escape) {
            break;
        }
        let timer = Instant::now();
        emu.update();
        if let Some(value) = Duration::new(0, 16666666).checked_sub(timer.elapsed()) {
            sleep(value);
        }
    }
}
