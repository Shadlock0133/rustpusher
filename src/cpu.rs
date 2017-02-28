use std::fs::File;
use std::io::Read;

pub const PAGE: usize = 0x100;
pub const BANK: usize = PAGE * PAGE;
pub const MEMORY: usize = BANK * PAGE;
pub const FULL_MEMORY: usize = MEMORY + 8;
pub const KEYBOARD: usize = 0;
pub const PC: usize = 2;
pub const VIDEO: usize = 5;
pub const AUDIO: usize = 6;

pub struct Cpu {
    pub ram: Box<[u8; FULL_MEMORY]>,
    pub pc: u32,
    pub step_counter: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            ram: box [0; FULL_MEMORY],
            pc: 0,
            step_counter: 0,
        }
    }

    #[allow(unknown_lints)]
    #[allow(unused_io_amount)]
    pub fn load_file(&mut self, file: &str) {
        let mut file = File::open(file).unwrap();
        file.read(&mut self.ram[..MEMORY]).expect("Unable to read file.");
        for x in 0..8 {
            assert_eq!(self.ram[MEMORY + x], 0);
        }
    }

    pub fn process_input(&mut self, input: (u8, u8)) {
        self.ram[KEYBOARD] = input.0;
        self.ram[KEYBOARD + 1] = input.1;
    }

    pub fn step(&mut self) {
        self.step_counter += 1;
        let pc = self.pc as usize;
        let src = self.address_at(pc);
        let byte = self.ram[src];
        let dst = self.address_at(pc + 3);
        self.ram[dst] = byte;
        self.pc = self.address_at(pc + 6) as u32;
    }

    pub fn finish_frame(&mut self) {
        self.pc = self.address_at(PC) as u32;
        while self.step_counter <= 65535 {
            self.step();
        }
        self.step_counter = 0;
    }

    pub fn get_video_slice(&self) -> &[u8] {
        let offset = self.ram[VIDEO] as usize * BANK;
        &self.ram[offset..offset + BANK]
    }

    pub fn get_audio_slice(&self) -> &[u8] {
        let offset = self.ram[AUDIO] as usize * BANK + self.ram[AUDIO + 1] as usize * PAGE;
        &self.ram[offset..offset + PAGE]
    }

    fn address_at(&self, offset: usize) -> usize {
        self.ram[offset] as usize * BANK +
        self.ram[offset + 1] as usize * PAGE +
        self.ram[offset + 2] as usize
    }
}
