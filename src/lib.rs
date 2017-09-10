#![feature(box_syntax)]

#[macro_use]
extern crate libretro_backend;
use libretro_backend::*;

mod cpu;
use cpu::*;

pub struct RPCore {
    cpu: Cpu,
    palette: [u32; 256],
    video_buffer: [u32; BANK],
    audio_buffer: Vec<i16>,
    game_data: Option<GameData>,
}

impl RPCore {
    fn new() -> Self {
        RPCore {
            cpu: Cpu::new(),
            palette: Self::default_palette(),
            video_buffer: [0; BANK],
            audio_buffer: Vec::with_capacity(512),
            game_data: None,
        }
    }

    fn default_palette() -> [u32; 256] {
        let mut palette = [0; 256];
        for (index, out) in palette.iter_mut().enumerate() {
            *out = match index {
                0x00...0xd7 => {
                    index as u32 / 36 * 0x33 * BANK as u32 +
                    index as u32 / 6 % 6 * 0x33 * PAGE as u32 +
                    index as u32 % 6 * 0x33
                }
                _ => 0x000000,
            }
        }
        palette
    }
}

impl Default for RPCore {
    fn default() -> Self {
        Self::new()
    }
}

impl Core for RPCore {
    fn info() -> CoreInfo {
        CoreInfo::new("Rustpusher", env!("CARGO_PKG_VERSION"))
            .supports_roms_with_extension(".BytePusher")
    }

    fn on_load_game(&mut self, game_data: GameData) -> LoadGameResult {
        if game_data.is_empty() {
            return LoadGameResult::Failed(game_data);
        }
        
        match if let Some(data) = game_data.data() {
            self.cpu.load_data(data)
        } else if let Some(path) = game_data.path() {
            self.cpu.load_file(path)
        } else {
            panic!("Unable to load game data")
        } {
            Ok(_) => {
                self.game_data = Some(game_data);
                LoadGameResult::Success(
                    AudioVideoInfo::new()
                        .video(256, 256, 60.0, PixelFormat::ARGB8888)
                        .audio(cpu::SAMPLE_RATE as _)
                )
            }
            Err(_) => LoadGameResult::Failed(game_data)
        }

    }

    fn on_unload_game(&mut self) -> GameData {
        self.cpu.clear_memory();
        self.game_data.take().unwrap()
    }

    fn on_run(&mut self, handle: &mut RuntimeHandle) {
        macro_rules! update_input {
            ( $( $button:ident => $expr:expr ,)* ) => (
                use JoypadButton::*;
                $( if handle.is_joypad_button_pressed(0, $button) { $expr } )*
            )
        }
        let mut input = (0u8, 0u8);
        update_input!(
            Up     => input.1 |= 0x1,
            Down   => input.1 |= 0x2,
            Left   => input.1 |= 0x4,
            Right  => input.1 |= 0x8,
            A      => input.1 |= 0x10,
            B      => input.1 |= 0x20,
            X      => input.1 |= 0x40,
            Y      => input.1 |= 0x80,
            Select => input.0 |= 0x1,
            Start  => input.0 |= 0x2,
            L1     => input.0 |= 0x4,
            L2     => input.0 |= 0x8,
            L3     => input.0 |= 0x10,
            R1     => input.0 |= 0x20,
            R2     => input.0 |= 0x40,
            R3     => input.0 |= 0x80,
        );
        self.cpu.process_input(input);

        self.cpu.finish_frame();

        for (&pixel, out) in self.cpu.get_video_slice().iter().zip(self.video_buffer.iter_mut()) {
            *out = self.palette[pixel as usize];
        }
        handle.upload_video_frame(as_bytes(&self.video_buffer));

        for &x in self.cpu.get_audio_slice().iter() {
            let x = (x as i16) << 8;
            self.audio_buffer.push(x);
            self.audio_buffer.push(x);
        }
        handle.upload_audio_frame(&self.audio_buffer[..]);
        self.audio_buffer.clear();
    }

    fn on_reset(&mut self) {
        if let Some(ref game_data) = self.game_data {
            if let Some(data) = game_data.data() {
                let _ = self.cpu.load_data(data);
            } else if let Some(path) = game_data.path() {
                let _ = self.cpu.load_file(path);
            }
        }
    }
}

#[inline]
pub fn as_bytes<T: Copy>(array: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute(array.as_ptr()), 
            std::mem::size_of::<T>() * array.len())
    }
}

libretro_core!(RPCore);