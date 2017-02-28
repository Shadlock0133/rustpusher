#![feature(box_syntax)]
extern crate minifb;
extern crate hound;

mod cpu;

use cpu::*;
use minifb::{Key, Scale, Window, WindowOptions};
use hound::WavWriter;
use std::fs::File;
use std::io::BufWriter;
use std::thread;
use std::time::{Duration, Instant};

pub struct Emu {
    cpu: Cpu,
    window: Window,
    audio_output: Option<WavWriter<BufWriter<File>>>,
    turbo: bool,
    paused: bool,
}

impl Emu {
    pub fn from_file(rom_file: &str, wav_file: Option<&str>) -> Self {
        let mut cpu = Cpu::new();
        cpu.load_file(rom_file);

        let win_options = WindowOptions { scale: Scale::X2, ..WindowOptions::default() };
        let window = Window::new("Rustpusher", PAGE, PAGE, win_options)
        .expect("Unable to create window.");

        let audio_spec = hound::WavSpec {
            channels: 2,
            sample_rate: 15360,
            bits_per_sample: 8,
            sample_format: hound::SampleFormat::Int
        };
        let audio_output = match wav_file {
            Some(wav_file) => Some(WavWriter::create(wav_file, audio_spec).unwrap()),
            None => None
        };

        Emu {
            cpu, window, audio_output,
            turbo: false,
            paused: false,
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let timer = Instant::now();

            let input = self.get_input();
            self.cpu.process_input(input);
            if !self.paused {
                self.cpu.finish_frame();
            }

            let window_buffer: Vec<u32> = self.cpu
                .get_video_slice()
                .iter()
                .map(|&x| Self::color_from_palette(x))
                .collect();
            self.window.update_with_buffer(&window_buffer);

            if let Some(ref mut writer) = self.audio_output {
                for sample in self.cpu.get_audio_slice() {
                    writer.write_sample(*sample as i8).unwrap();
                    writer.write_sample(*sample as i8).unwrap();
                }
            }

            if !self.turbo {
                if let Some(value) = Duration::new(0, 1_000_000_000 / 60).checked_sub(timer.elapsed()) {
                    thread::sleep(value);
                }
            }
        }
    }

    fn color_from_palette(index: u8) -> u32 {
        match index {
            0x00...0xd7 => {
                index as u32 / 36 * 0x33 * BANK as u32 +
                index as u32 / 6 % 6 * 0x33 * PAGE as u32 +
                index as u32 % 6 * 0x33
            }
            _ => 0x000000,
        }
    }

    fn get_input(&mut self) -> (u8, u8) {
        let mut input = (0u8, 0u8);
        self.window.get_keys().map(|keys| for k in keys {
            match k {
                Key::T => self.turbo = !self.turbo,
                Key::P => self.paused = !self.paused,

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
}
