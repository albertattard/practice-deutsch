use audio::pronounce;
use clap::Parser;
use nouns::articles;
use types::{Args, Mode};

mod audio;
mod download;
mod nouns;
mod types;
mod verbs;

fn main() {
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
    todo!("Not implemented yet");
}

fn verbs() {
    todo!("Not implemented yet");
}
