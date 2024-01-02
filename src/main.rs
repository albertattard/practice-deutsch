use clap::Parser;

use crate::audio::pronounce;
use crate::nouns::articles;

mod audio;
mod download;
mod nouns;
mod types;

fn main() {
    use types::{Args, Mode};

    let args = Args::parse();

    match args.mode {
        Mode::Articles => articles(),
        Mode::Plural => plural(),
        Mode::Verbs => verbs(),
        Mode::Numbers => numbers(),
        Mode::Alphabet => alphabet(),
    }
}

fn numbers() {
    pronounce("audio/numbers")
}

fn alphabet() {
    pronounce("audio/alphabet")
}

fn plural() {
    println!("Not implemented yet");
}

fn verbs() {
    println!("Not implemented yet");
}
