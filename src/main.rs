extern crate rustpusher;
extern crate clap;

use rustpusher::*;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Rustpusher")
        .version("0.1.0")
        .author("Shadlock")
        .about("Bytepusher emulator")
        .arg(Arg::with_name("wavout")
            .help("If specified, creates WAV file. Used for debug purposes.")
            .short("-w")
            .long("--wav")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("ROM's filename")
            .required(true)
            .index(1)).get_matches();

    let rom_file = matches.value_of("INPUT").unwrap();
    let wav_file = matches.value_of("wavout");

    let mut emu = Emu::from_file(rom_file, wav_file);
    emu.run();
}
