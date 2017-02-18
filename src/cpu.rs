use std::fs::File;
use std::io::Read;
use std::ops::DerefMut;

pub const PAGE: usize = 0x100;
pub const BANK: usize = PAGE * PAGE;
pub const MEMORY: usize = BANK * PAGE + 8;

pub struct Cpu {
    pub ram: Box<[u8; MEMORY]>,
    // pub step_counter: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            ram: box [0u8; MEMORY],
            // step_counter: 0,
        }
    }

    #[allow(unused_io_amount)]
    pub fn load_file(&mut self, file: &str) {
        let mut file = File::open(file).unwrap();
        file.read(self.ram.deref_mut()).expect("Unable to read file.");
    }

    pub fn process_input(&mut self, input: (u8, u8)) {
        self.ram[0] = input.0;
        self.ram[1] = input.1;
    }

    // TODO: properly refactor for debugger use
    // pub fn step(&mut self, mut pc: &usize) {
    //     let src = self.address_at(pc);
    //     let byte = self.ram[src];
    //     let dst = self.address_at(pc + 3);
    //     self.ram[dst] = byte;
    //     pc = self.address_at(pc + 6);
    // }

    pub fn frame(&mut self) {
        let mut pc = self.address_at(2);
        for _ in 0..65536 {
            let src = self.address_at(pc);
            let byte = self.ram[src];
            let dst = self.address_at(pc + 3);
            self.ram[dst] = byte;
            pc = self.address_at(pc + 6);
            // self.step(pc);
        }
    }

    pub fn get_video_slice(&self) -> &[u8] {
        let offset = self.ram[5] as usize * BANK;
        &self.ram[offset..offset + BANK]
    }

    pub fn get_audio_slice(&self) -> &[u8] {
        let offset = self.ram[6] as usize * BANK + self.ram[7] as usize * PAGE;
        &self.ram[offset..offset + PAGE]
    }

    fn address_at(&self, offset: usize) -> usize {
        self.ram[offset] as usize * BANK +
        self.ram[offset + 1] as usize * PAGE +
        self.ram[offset + 2] as usize
    }
}

pub fn color_from_palette(index: u8) -> u32 {
    match index {
        0x00...0xd7 => {
            index as u32 / 36 * 0x33 * BANK as u32 +
            index as u32 / 6 % 6 * 0x33 * PAGE as u32 +
            index as u32 % 6 * 0x33
        }
        _ => 0x000000,
    }
}
