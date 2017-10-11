#[macro_use]
extern crate clap;
extern crate minifb;
extern crate rustpusher_cpu;

use clap::{Arg};
use minifb::*;
use rustpusher_cpu::*;

use std::path::Path;

enum State {
    Paused,
    Running,
}

use State::*;

struct Emu {
    pub state: State,
    pub cpu: Cpu,
    pub palette: [u32; 256],
    pub video_buffer: [u32; 256 * 256],
    pub audio_buffer: [u32; 256 * 256],
    pub video_window: Window,
    pub audio_window: Window,
}

impl Emu {
    pub fn new<P: AsRef<Path>>(rom_file: P) -> Self {
        let mut cpu = Cpu::new();
        cpu.load_file(rom_file).unwrap();
        let video_buffer = [0u32; 256 * 256];
        let audio_buffer = [0u32; 256 * 256];
        let mut video_window = Window::new("Video", 256, 256, WindowOptions::default()).unwrap();
        let mut audio_window = Window::new("Audio", 256, 256, WindowOptions::default()).unwrap();
        video_window.update_with_buffer(&video_buffer).unwrap();
        audio_window.update_with_buffer(&audio_buffer).unwrap();
        Emu {
            state: Paused,
            cpu,
            palette: Self::default_palette(),
            video_buffer, audio_buffer, video_window, audio_window,
        }
    }

    fn default_palette() -> [u32; 256] {
        let mut palette = [0; 256];
        for index in 0..256 {
            palette[index] = match index {
                0x00...0xd7 =>
                    (index as u32 / 36 * 0x33) << 16 |
                    (index as u32 / 6 % 6 * 0x33) << 8 |
                    (index as u32 % 6 * 0x33),
                _ => 0,
            }
        }
        palette
    }

    fn are_windows_open(&self) -> bool {
        self.video_window.is_open() && self.audio_window.is_open()
    }

    fn is_key_pressed(&self, key: Key, key_repeat: KeyRepeat) -> bool {
        self.video_window.is_key_pressed(key, key_repeat) ||
        self.audio_window.is_key_pressed(key, key_repeat)
    }

    fn is_key_down(&self, key: Key) -> bool {
        self.video_window.is_key_down(key) ||
        self.audio_window.is_key_down(key)
    }

    pub fn run(&mut self) {
        while self.are_windows_open() && !self.is_key_down(Key::Escape) {
            match self.state {
                Paused => {
                    if self.is_key_pressed(Key::Space, KeyRepeat::No) {
                        self.cpu.step();
                    }
                    if self.is_key_pressed(Key::S, KeyRepeat::No) {
                        self.cpu.finish_frame();
                    }
                },
                Running => self.cpu.finish_frame(),
            }
            if self.is_key_pressed(Key::P, KeyRepeat::No) {
                self.state = match self.state {
                    Paused => Running,
                    _ => Paused
                };
            }
            self.update();
            ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000 / 60));
        }
    }

    fn update(&mut self) {
        for (out, byte) in self.video_buffer.iter_mut().zip(self.cpu.get_video_slice().iter()) {
            *out = self.palette[*byte as usize];
        }
        self.audio_buffer = [0; 256 * 256];
        for (index, byte) in self.cpu.get_audio_slice().iter().enumerate() {
            self.audio_buffer[(383 - *byte as usize) % 255 * 256 + index as usize] = 0xffffff;
        }
        self.video_window.set_title(&format!("Video - step: {}", self.cpu.get_step()));
        self.video_window.update_with_buffer(&self.video_buffer).unwrap();
        self.audio_window.update_with_buffer(&self.audio_buffer).unwrap();
    }
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("INPUT")
            .required(true)
            .index(1))
        .get_matches();
    
    let rom_file = matches.value_of("INPUT").unwrap();
    let mut emu = Emu::new(rom_file);

    emu.run();
}
