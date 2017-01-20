#![feature(box_syntax)]
use std::fs::File;
use std::io::Read;
use std::ops::DerefMut;

const PAGE: usize = 0x100;
const BANK: usize = PAGE * PAGE;
const MEMORY: usize = BANK * PAGE + 8;

pub struct BytePusher {
    pub ram: Box<[u8; MEMORY]>,
    pub step_counter: u16,
}

impl BytePusher {
    pub fn new(file: &str) -> BytePusher {
        BytePusher {
            ram: BytePusher::read_file(file),
            step_counter: 0,
        }
    }

    pub fn process_input(&mut self, input: (u8, u8)) {
        self.ram[0] = input.0;
        self.ram[1] = input.1;
    }

    // TODO: properly refactor for debugger use
    // pub fn step(&mut self) {}

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

    pub fn get_screen_slice(&self) -> &[u8] {
        let offset = self.ram[5] as usize * BANK;
        &self.ram[offset..offset + BANK]
    }

    pub fn get_audio_slice(&self) -> &[u8] {
        let offset = self.ram[6] as usize * BANK + self.ram[7] as usize * PAGE;
        &self.ram[offset..offset + PAGE]
    }

    fn read_file(file: &str) -> Box<[u8; MEMORY]> {
        let mut file = File::open(file).expect("Unable to open file.");
        let mut buf = box [0u8; MEMORY];
        file.read(buf.deref_mut()).expect("Unable to read file.");
        buf
    }

    fn address_at(&self, offset: usize) -> usize {
        self.ram[offset] as usize * BANK + self.ram[offset + 1] as usize * PAGE +
        self.ram[offset + 2] as usize
    }
}
