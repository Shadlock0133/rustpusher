#![feature(box_syntax, slice_get_slice)]

use std::fs::File;
use std::io::{self, Read, Write};
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::slice::SliceIndex;

// Sizes
pub const PAGE: usize = 0x100;
pub const BANK: usize = PAGE * 256;
pub const MEMORY: usize = BANK * 256;
pub const FULL_MEMORY: usize = MEMORY + 8;
pub const SAMPLE_RATE: usize = PAGE * 60;

// Offsets
pub const INPUT: usize = 0;
pub const PC: usize = 2;
pub const VIDEO: usize = 5;
pub const AUDIO: usize = 6;

type CpuResult = io::Result<()>;

pub struct Memory {
    inner: Box<[u8; FULL_MEMORY]>,
}

impl Memory {
    pub fn new() -> Self {
        Self { inner: box [0; FULL_MEMORY] }
    }

    pub fn address_at(&self, offset: usize) -> usize {
        self.inner[offset] as usize * BANK + self.inner[offset + 1] as usize * PAGE +
            self.inner[offset + 2] as usize
    }

    pub fn clear(&mut self) {
        self.inner = box [0; FULL_MEMORY];
    }

    pub fn fill_page(&mut self, offset: u8, f: fn(usize) -> u8) {
        let offset = (offset as usize) << 16;
        for i in 0..PAGE {
            self.inner[offset + i] = f(i);
        }
    }

    pub fn fill_bank(&mut self, offset: u16, f: fn(usize) -> u8) {
        let offset = (offset as usize) << 8;
        for i in 0..BANK {
            self.inner[offset + i] = f(i);
        }
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for Memory {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.inner[index]
    }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for Memory {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

pub struct Cpu {
    pub memory: Memory,
    pc: u32,
    step: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            pc: 0,
            step: 0,
        }
    }

    pub fn default_palette() -> [[u8; 4]; 256] {
        let mut palette = [[0; 4]; 256];
        for index in 0..256 {
            match index {
                0x00...0xd7 => {
                    palette[index] = [
                        (index as u32 / 36 * 0x33) as u8,
                        (index as u32 / 6 % 6 * 0x33) as u8,
                        (index as u32 % 6 * 0x33) as u8,
                        0,
                    ]
                }
                _ => (),
            }
        }
        palette
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, file: P) -> CpuResult {
        let mut file = File::open(file)?;
        file.read(&mut self.memory[..MEMORY])?;
        Ok(())
    }

    pub fn load_data(&mut self, data: &[u8]) -> CpuResult {
        (&mut self.memory[..MEMORY]).write(data)?;
        Ok(())
    }

    pub fn clear_memory(&mut self) {
        self.memory.clear();
    }

    pub fn process_input(&mut self, input: (u8, u8)) {
        self.memory[INPUT] = input.0;
        self.memory[INPUT + 1] = input.1;
    }

    pub fn cycle(&mut self) {
        if self.step == 0 {
            self.pc = self.memory.address_at(PC) as u32;
        }
        self.step += 1;
        let pc = self.pc as usize;
        let src = self.memory.address_at(pc);
        let byte = self.memory[src];
        let dst = self.memory.address_at(pc + 3);
        self.memory[dst] = byte;
        self.pc = self.memory.address_at(pc + 6) as u32;
        if self.step > 65535 {
            self.step = 0;
        }
    }

    pub fn finish_frame(&mut self) {
        for _ in self.step..65536 {
            self.cycle();
        }
    }

    pub fn pc(&self) -> u32 { self.pc }

    pub fn step(&self) -> u32 { self.step }

    pub fn get_video_slice(&self) -> &[u8] {
        let offset = self.memory[VIDEO] as usize * BANK;
        &self.memory[offset..offset + BANK]
    }

    pub fn get_audio_slice(&self) -> &[u8] {
        let offset = self.memory[AUDIO] as usize * BANK + self.memory[AUDIO + 1] as usize * PAGE;
        &self.memory[offset..offset + PAGE]
    }
}
