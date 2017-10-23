#[macro_use]
extern crate clap;
extern crate minifb;
extern crate rustpusher_cpu;
mod font;
#[macro_use]
mod gprint;

use clap::Arg;
use minifb::*;
use rustpusher_cpu::*;

use gprint::*;

use std::path::Path;
use std::time::{Duration, Instant};
use std::thread;

enum State {
    Paused,
    Running,
    RunningCycles,
    #[doc(hidden)]
    //#[allow(non_camel_case_types)]
    _NonExhaustive,
}

use State::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

struct Dbg {
    pub cpu: Cpu,
    pub rom_file: String,
    pub palette: [u32; 256],
    pub window_buffer: Box<[u32; WIDTH * HEIGHT]>,
    pub window: Window,
    pub state: State,
    pub frame: u64,
    pub step_size: u16,
    pub mem_peek: u32,
}

impl Dbg {
    pub fn new<P: AsRef<Path> + Clone + Into<String>>(rom_file: P) -> Self {
        let mut cpu = Cpu::new();
        let file = rom_file.clone().into();
        cpu.load_file(rom_file).unwrap();

        let window_buffer = Box::new([0u32; WIDTH * HEIGHT]);
        let wo = WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        };
        let mut window = Window::new("Debugger", WIDTH, HEIGHT, wo).unwrap();
        window.update_with_buffer(&*window_buffer).unwrap();

        Self {
            cpu,
            rom_file: file,
            palette: Self::default_palette(),
            window_buffer,
            window,
            state: Paused,
            frame: 0,
            step_size: 1,
            mem_peek: 0,
        }
    }

    fn default_palette() -> [u32; 256] {
        let pal = Cpu::default_palette();
        let mut palette = [0; 256];
        for index in 0..256 {
            palette[index] = (pal[index][0] as u32) << 16 | (pal[index][1] as u32) << 8 |
                (pal[index][2] as u32);
        }
        palette
    }

    pub fn run(&mut self) {
        let timer = Instant::now();
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            match self.state {
                Paused => {
                    if self.window.is_key_pressed(Key::L, KeyRepeat::Yes) {
                        for _ in 0..self.step_size {
                            self.cpu.cycle();
                            if self.cpu.step() > 65535 {
                                self.frame += 1;
                            }
                        }
                    }
                    if self.window.is_key_pressed(Key::I, KeyRepeat::Yes) {
                        self.cpu.finish_frame();
                        self.frame += 1;
                    }
                    if self.window.is_key_pressed(Key::U, KeyRepeat::No) {
                        self.cpu.finish_frame();
                        self.cpu.memory.clear();
                        self.cpu.load_file(&self.rom_file).unwrap();
                        self.frame = 0;
                    }
                }
                Running => {
                    self.cpu.finish_frame();
                    self.frame += 1;
                }
                RunningCycles => {
                    for _ in 0..self.step_size {
                        self.cpu.cycle();
                        if self.cpu.step() > 65535 {
                            self.frame += 1;
                        }
                    }
                }
                _ => (),
            }
            if self.window.is_key_pressed(Key::P, KeyRepeat::No) {
                self.state = match self.state {
                    Paused => Running,
                    _ => Paused,
                };
            }
            if self.window.is_key_pressed(Key::O, KeyRepeat::No) {
                self.state = match self.state {
                    Paused => RunningCycles,
                    _ => Paused,
                };
            }
            if self.window.is_key_pressed(Key::Equal, KeyRepeat::Yes) {
                self.step_size = self.step_size.saturating_add(1).min(65535);
            }
            if self.window.is_key_pressed(Key::Minus, KeyRepeat::Yes) {
                self.step_size = self.step_size.saturating_sub(1).max(1);
            }
            if self.window.is_key_pressed(Key::Key0, KeyRepeat::No) {
                self.step_size = 1;
            }
            macro_rules! process_input {
                ( $( $key:expr => $f:expr ,)* ) => {
                    $(if self.window.is_key_pressed($key, KeyRepeat::No) {
                        $f;
                    })*
                };
            }
            fn toggle_bit(byte: &mut u8, offset: u8) {
                if *byte & 1 << offset == 0 {
                    *byte |= 1 << offset;
                } else {
                    *byte &= !(1 << offset);
                }
            }
            let mut input = (self.cpu.memory[0], self.cpu.memory[1]);
            process_input!(
                Key::X    => toggle_bit(&mut input.1, 0),
                Key::Key1 => toggle_bit(&mut input.1, 1),
                Key::Key2 => toggle_bit(&mut input.1, 2),
                Key::Key3 => toggle_bit(&mut input.1, 3),
                Key::Q    => toggle_bit(&mut input.1, 4),
                Key::W    => toggle_bit(&mut input.1, 5),
                Key::E    => toggle_bit(&mut input.1, 6),
                Key::A    => toggle_bit(&mut input.1, 7),
                Key::S    => toggle_bit(&mut input.0, 0),
                Key::D    => toggle_bit(&mut input.0, 1),
                Key::Z    => toggle_bit(&mut input.0, 2),
                Key::C    => toggle_bit(&mut input.0, 3),
                Key::Key4 => toggle_bit(&mut input.0, 4),
                Key::R    => toggle_bit(&mut input.0, 5),
                Key::F    => toggle_bit(&mut input.0, 6),
                Key::V    => toggle_bit(&mut input.0, 7),
            );
            self.cpu.process_input(input);
            // Directly display vram, with clearing
            *self.window_buffer = [0; WIDTH * HEIGHT];
            let video_slice = self.cpu.get_video_slice();
            for y in 0..256 {
                for x in 0..256 {
                    self.window_buffer[y * WIDTH + x] = self.palette[video_slice[y * 256 + x] as
                                                                         usize];
                }
            }
            // Clear then display audio_ram as waveform
            // We're already clearing screen, so optimize it
            // Update: Done
            let audio_slice = self.cpu.get_audio_slice();
            for x in 0..256 {
                let oy = ((256 + 127) - audio_slice[x] as usize) % 255;
                // for y in 0..256 {
                    self.window_buffer[oy * WIDTH + x + 256] = 0xffffff;
                    //if y == oy { 0xffffff } else { 0 };
                // }
            }
            // Print current instruction + current status (frame, step)
            {
                let mut gp = GPrinter::new(&mut *self.window_buffer, WIDTH);
                gprint!(
                    &mut gp,
                    0,
                    256,
                    gprint::WHITE,
                    "frame: {}, step: {:5}, step_size: {:5}     ",
                    self.frame,
                    self.cpu.step(),
                    self.step_size
                );
                let input = (self.cpu.memory[0] as u16) << 8 | self.cpu.memory[1] as u16;
                gprint!(&mut gp, 0, 256 + 8, gprint::WHITE, "input: {:016b}", input);
                let init = self.cpu.memory.address_at(2);
                let video = self.cpu.memory[5];
                let audio = (self.cpu.memory[6] as u16) << 8 | self.cpu.memory[7] as u16;
                gprint!(
                    &mut gp,
                    0,
                    256 + 16,
                    gprint::WHITE,
                    "init: {:06x}, video: {:02x}0000, audio: {:04x}00",
                    init,
                    video,
                    audio
                );
                let pc = self.cpu.pc() as usize;
                let src = self.cpu.memory.address_at(pc);
                let src_byte = self.cpu.memory[src];
                let dst = self.cpu.memory.address_at(pc + 3);
                let dst_byte = self.cpu.memory[dst];
                let jmp = self.cpu.memory.address_at(pc + 6);
                let jmp = if jmp == pc + 9 {
                    " next".into()
                } else if jmp == pc {
                    " loop".into()
                } else {
                    format!(" {:06x}", jmp)
                };
                gprint!(
                    &mut gp,
                    0,
                    256 + 24,
                    gprint::WHITE,
                    "{:06x}: mv {:06x} ({:02x}), {:06x} ({:02x});{}",
                    pc,
                    src,
                    src_byte,
                    dst,
                    dst_byte,
                    jmp
                );

                self.mem_peek = ((pc as u32) & 0xfffffff0).saturating_sub(64);
                for y in 0..32 {
                    gprint!(
                        &mut gp,
                        256,
                        y * 8 + 256,
                        gprint::WHITE,
                        "{:06x}",
                        (self.mem_peek as usize) + y * 16
                    );
                    for x in 0..16 {
                        let addr = (self.mem_peek as usize) + y * 16 + x;
                        let byte = self.cpu.memory[addr];
                        let colour = match byte {
                            _ if addr - pc < 9 => gprint::RED,
                            0x00 => gprint::GREEN,
                            0x7f => gprint::BLUE,
                            0xff => gprint::RED,
                            0x20...0x7f => gprint::YELLOW,
                            _ => gprint::WHITE,
                        };
                        gprint!(
                            &mut gp,
                            256 + (8 * FONT_WIDTH) + x * (3 * FONT_WIDTH),
                            y * 8 + 256,
                            colour,
                            " {:02x}",
                            byte
                        );
                    }
                }
                for c in 0x20..0x80 {
                    gprint!(&mut gp,
                        (c % 16) * FONT_WIDTH,
                        256 + (4 + c / 16) * FONT_HEIGHT,
                        gprint::WHITE,
                        "{}", c as u8 as char);
                }
                gprint!(&mut gp, FONT_HEIGHT * 0, HEIGHT - 8, gprint::GREEN, "00");
                gprint!(&mut gp, FONT_HEIGHT * 1, HEIGHT - 8, gprint::BLUE, "7f");
                gprint!(&mut gp, FONT_HEIGHT * 2, HEIGHT - 8, gprint::RED, "ff");
                gprint!(&mut gp, FONT_HEIGHT * 3, HEIGHT - 8, gprint::YELLOW, "20..7f");
                gprint!(&mut gp, FONT_HEIGHT * 6, HEIGHT - 8, gprint::WHITE, "99");
            }

            self.window
                .update_with_buffer(&*self.window_buffer)
                .unwrap();
            
            if let Some(time) = Duration::new(0, 1_000_000_000 / 60).checked_sub(timer.elapsed()) {
                thread::sleep(time);
            }
        }
    }
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("INPUT").required(true).index(1))
        .get_matches();

    let rom_file = matches.value_of("INPUT").unwrap();
    let mut dbg = Dbg::new(rom_file);

    // for x in 0x20..0x80 {
    //     print!("{}", x as u8 as char);
    //     if x % 16 == 15 { println!(); }
    // }

    dbg.run();
}
