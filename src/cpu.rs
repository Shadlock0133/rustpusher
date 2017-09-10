use std::fs::File;
use std::io::{Read, Write};

use errors::*;

pub const PAGE: usize = 0x100;
pub const BANK: usize = PAGE * PAGE;
pub const MEMORY: usize = BANK * PAGE;
pub const FULL_MEMORY: usize = MEMORY + 8;

pub const KEYBOARD: usize = 0;
pub const PC: usize = 2;
pub const VIDEO: usize = 5;
pub const AUDIO: usize = 6;

pub struct Cpu {
    memory: Box<[u8; FULL_MEMORY]>,
    pc: u32,
    step_counter: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: box [0; FULL_MEMORY],
            pc: 0,
            step_counter: 0,
        }
    }

    pub fn load_file(&mut self, file: &str) -> Result<()> {
        let mut file = File::open(file)?;
        file.read(&mut self.memory[..MEMORY])?;
        Ok(())
    }

    pub fn load_data(&mut self, data: &[u8]) -> Result<()> {
        (&mut self.memory[..MEMORY]).write(data)?;
        Ok(())
    }

    pub fn clear_memory(&mut self) {
        self.memory = box [0; FULL_MEMORY]
    }

    pub fn process_input(&mut self, input: (u8, u8)) {
        self.memory[KEYBOARD] = input.0;
        self.memory[KEYBOARD + 1] = input.1;
    }

    #[inline]
    fn step(&mut self) {
        self.step_counter += 1;
        let pc = self.pc as usize;
        let src = self.address_at(pc);
        let byte = self.memory[src];
        let dst = self.address_at(pc + 3);
        self.memory[dst] = byte;
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
        let offset = self.memory[VIDEO] as usize * BANK;
        &self.memory[offset..offset + BANK]
    }

    pub fn get_audio_slice(&self) -> &[u8] {
        let offset = self.memory[AUDIO] as usize * BANK + self.memory[AUDIO + 1] as usize * PAGE;
        &self.memory[offset..offset + PAGE]
    }

    fn address_at(&self, offset: usize) -> usize {
        self.memory[offset] as usize * BANK +
        self.memory[offset + 1] as usize * PAGE +
        self.memory[offset + 2] as usize
    }
}
