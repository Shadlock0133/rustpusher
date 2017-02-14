extern crate rustpusher;
extern crate clap;
extern crate minifb;
extern crate cpal;
extern crate futures;

use rustpusher::*;
use clap::{App, Arg};
use minifb::{Key, Scale, Window, WindowOptions};
use futures::stream::Stream;
use futures::task;
use futures::task::{Executor, Run};
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let name = "Rustpusher";
    let app = App::new(name)
                        .version("0.1.0")
                        .author("Shadlock")
                        .about("a Bytepusher emulator")
                        .arg(Arg::with_name("INPUT")
                            .help("ROM's filename")
                            .required(true)
                            .index(1));
    let matches = app.get_matches();

    let file = matches.value_of("INPUT").expect("No filename given.");
    let mut emu = BytePusher::new(&file);

    let win_options = WindowOptions { scale: Scale::X2, ..WindowOptions::default() };
    let mut window = Window::new(name, PAGE, PAGE, win_options)
        .expect("Unable to create window.");

    // let endpoint = cpal::get_default_endpoint().expect("Unable to get endpoint.");
    // let format = endpoint.get_supported_formats_list().unwrap().next().unwrap();
    // let event_loop = cpal::EventLoop::new();
    // let (mut voice, stream) = cpal::Voice::new(&endpoint, &format, &event_loop).unwrap();
    //
    // let mut data_source = VecDeque::with_capacity(512);
    //
    // voice.play();
    //
    // struct MyExecutor;
    //
    // impl Executor for MyExecutor {
    //     fn execute(&self, r: Run) {
    //         r.run();
    //     }
    // }
    //
    // let executor = Arc::new(MyExecutor {});
    //
    // task::spawn(stream.for_each(move |buffer| -> Result<_, ()> {
    //     match buffer {
    //         cpal::UnknownTypeBuffer::U16(mut buffer) => {
    //             for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
    //                 let value = ((*value * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
    //                 for out in sample.iter_mut() { *out = value; }
    //             }
    //         },
    //
    //         cpal::UnknownTypeBuffer::I16(mut buffer) => {
    //             for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
    //                 let value = (*value * std::i16::MAX as f32) as i16;
    //                 for out in sample.iter_mut() { *out = value; }
    //             }
    //         },
    //
    //         cpal::UnknownTypeBuffer::F32(mut buffer) => {
    //             for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
    //                 for out in sample.iter_mut() { *out = *value; }
    //             }
    //         },
    //     };
    //     Ok(())
    // })).execute(executor);
    //
    // thread::spawn(move || {
    //     event_loop.run();
    // });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let timer = Instant::now();

        emu.process_input(get_input(&window));
        emu.frame();

        let window_buffer: Vec<u32> = emu
            .get_video_slice()
            .iter()
            .map(|&x| color_from_palette(x))
            .collect();
        window.update_with_buffer(&window_buffer);

        // TODO: Audio stuff

        if !window.is_key_down(Key::T) {
            window.set_title(name);
            if let Some(value) = Duration::new(0, 16666666).checked_sub(timer.elapsed()) {
                thread::sleep(value);
            }
        } else {
            let name_t = name.to_owned() + " - Turbo";
            window.set_title(&name_t);
        }
    }
}

fn get_input(ref window: &Window) -> (u8, u8) {
    let mut input = (0u8, 0u8);
    window.get_keys().map(|keys| for k in keys {
        match k {
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
