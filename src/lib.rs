#![feature(box_syntax)]
extern crate minifb;

use std::fs::File;
use std::io::Read;
use std::ops::DerefMut;
use minifb::{Key, Window, WindowOptions};

const PAGE: usize = 0x100;
const BANK: usize = 0x1_0000;
const MEMORY: usize = 0x100_0008;

pub struct BytePusher {
    pub ram: Box<[u8; MEMORY]>,
    pub window: Window,
}

impl BytePusher {
    fn read_file(file: &str) -> Box<[u8; MEMORY]> {
        let mut file = File::open(file).expect("Unable to open file.");
        let mut buf = box [0u8; MEMORY];
        file.read(buf.deref_mut()).expect("Unable to read file.");
        buf
    }

    pub fn new(file: &str) -> BytePusher {
        let ram = BytePusher::read_file(file);

        let window = Window::new("BytePusher Test",
                                    PAGE,
                                    PAGE,
                                    WindowOptions::default())
                                    .expect("Unable to create window");

        BytePusher {
            ram: ram,
            window: window,
        }
    }

    pub fn process_input(&mut self) {
        let mut keyboard = 0u16;
        self.window.get_keys().map(|keys|
            for k in keys {
                match k {
                    Key::X    => keyboard += 0b0000000000000001,

                    Key::Key1 => keyboard += 0b0000000000000010,
                    Key::Key2 => keyboard += 0b0000000000000100,
                    Key::Key3 => keyboard += 0b0000000000001000,

                    Key::Q    => keyboard += 0b0000000000010000,
                    Key::W    => keyboard += 0b0000000000100000,
                    Key::E    => keyboard += 0b0000000001000000,

                    Key::A    => keyboard += 0b0000000010000000,
                    Key::S    => keyboard += 0b0000000100000000,
                    Key::D    => keyboard += 0b0000001000000000,

                    Key::Z    => keyboard += 0b0000010000000000,
                    Key::C    => keyboard += 0b0000100000000000,

                    Key::Key4 => keyboard += 0b0001000000000000,
                    Key::R    => keyboard += 0b0010000000000000,
                    Key::F    => keyboard += 0b0100000000000000,
                    Key::V    => keyboard += 0b1000000000000000,

                    _ => {}
                }
            }
        );
        self.ram[0] = (keyboard / PAGE as u16) as u8;
        self.ram[1] = (keyboard % PAGE as u16) as u8;
    }

    fn address_at(&self, offset: usize) -> usize {
        self.ram[offset    ] as usize * BANK +
        self.ram[offset + 1] as usize * PAGE +
        self.ram[offset + 2] as usize
    }

    pub fn frame(&mut self) {
        let mut pc = self.address_at(2);
        for _ in 0..65536 {
            let src = self.address_at(pc);
            let byte = self.ram[src];
            let dst = self.address_at(pc + 3);
            self.ram[dst] = byte;
            pc = self.address_at(pc + 6);
        }
    }

    fn color_from_palette(index: u8) -> u32 {
        match index {
            0x00...0xd7 =>  index as u32 / 36    * 0x33 * BANK as u32 +
                            index as u32 / 6 % 6 * 0x33 * PAGE as u32 +
                            index as u32     % 6 * 0x33,
            _ => 0x000000
        }
    }

    pub fn update_window(&mut self) {
        let mut window_buffer: Vec<u32> = vec![0; PAGE * PAGE];

        let offset = self.ram[5] as usize * BANK;

        for (i, pixel) in window_buffer.iter_mut().enumerate() {
            *pixel = BytePusher::color_from_palette(self.ram[offset + i]);
        }

        self.window.update_with_buffer(&window_buffer);
    }

    fn audio(&self) {
        let start = self.ram[6] as usize * BANK + self.ram[7] as usize * PAGE;
        let end = start + PAGE;
        let buf = &self.ram[start..end];
    }

    pub fn update(&mut self) {
        self.process_input();
        self.frame();
        self.update_window();
        self.audio();
    }
}
