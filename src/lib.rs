#![feature(box_syntax)]
extern crate clap;
extern crate minifb;
extern crate hound;

mod cpu;

use cpu::*;
use clap::{App, Arg};
use minifb::{Key, Scale, Window, WindowOptions};
use std::thread;
use std::time::{Duration, Instant};

pub fn run() {
    let app = App::new("Rustpusher")
                        .version("0.1.0")
                        .author("Shadlock")
                        .about("a Bytepusher emulator")
                        .arg(Arg::with_name("wavout")
                            .help("Output WAV's filename")
                            .short("-w")
                            .long("--wav")
                            .takes_value(true))
                        .arg(Arg::with_name("INPUT")
                            .help("ROM's filename")
                            .required(true)
                            .index(1));
    let name = &String::from(app.get_name());
    let matches = app.get_matches();

    let mut emu = Cpu::new();
    let rom_file = matches.value_of("INPUT").unwrap();
    emu.load_file(rom_file);

    let win_options = WindowOptions { scale: Scale::X2, ..WindowOptions::default() };
    let mut window = Window::new(name, PAGE, PAGE, win_options)
        .expect("Unable to create window.");

    let audio_spec = hound::WavSpec {
        channels: 2,
        sample_rate: 15360,
        bits_per_sample: 8,
        sample_format: hound::SampleFormat::Int
    };
    let wav_file = matches.value_of("wavout");
    let mut writer = match wav_file {
        Some(wav_file) => Some(hound::WavWriter::create(wav_file, audio_spec).unwrap()),
        None => None
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let timer = Instant::now();

        emu.process_input(get_input(&window));
        emu.frame();

        let window_buffer: Vec<u32> = emu
            .get_video_slice()
            .iter()
            .map(|&x| color_from_palette(x))
            .collect();
        window.update_with_buffer(&window_buffer);

        if let Some(ref mut writer) = writer {
            for sample in emu.get_audio_slice() {
                writer.write_sample(*sample as i8).unwrap();
                writer.write_sample(*sample as i8).unwrap();
            }
        }

        if !window.is_key_down(Key::T) {
            window.set_title(name);
            if let Some(value) = Duration::new(0, 1_000_000_000 / 60).checked_sub(timer.elapsed()) {
                thread::sleep(value);
            }
        } else {
            let name_t = format!("{} - Turbo", name);
            window.set_title(&name_t);
        }
    }
}

fn get_input(window: &Window) -> (u8, u8) {
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
