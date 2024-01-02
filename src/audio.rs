use rand::prelude::SliceRandom;
use std::cmp::max;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{stdin, BufReader};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

use rodio::{source::Source, Decoder, OutputStream};

pub(crate) fn pronounce(directory: &str) {
    let files: Vec<PathBuf> = fs::read_dir(directory)
        .expect(format!("Failed to read {}", directory).as_str())
        .map(|r| r.unwrap().path())
        .collect();

    loop {
        let mut rng = rand::thread_rng();
        let file = files.choose(&mut rng).unwrap();
        play_file_and_verify(file);
    }
}

fn play_file_and_verify(file: &PathBuf) {
    if let Err(e) = play_file(file) {
        println!("Failed to play audio file: {:?} ({})", file, e);
        return;
    }

    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read the user input");
        let input = input.trim();

        match input {
            "q" | "quit" => return,
            "r" | "repeat" => {
                if let Err(e) = play_file(file) {
                    println!("Failed to replay play audio file: {:?} ({})", file, e);
                }
                continue;
            }
            input => {
                let expected = file.file_stem().unwrap().to_str().unwrap();
                if !expected.eq(input) {
                    println!("Wrong! It was: {}", expected);
                    if let Err(e) = play_file(file) {
                        println!("Failed to replay play audio file: {:?} ({})", file, e);
                    }
                }
                break;
            }
        }
    }
}

pub(crate) fn play_file(path: &Path) -> Result<(), Box<dyn Error>> {
    /* Based on: https://docs.rs/rodio/latest/rodio/ */

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let file = File::open(path)?;
    let length = file.metadata().unwrap().len();
    let reader = BufReader::new(file);
    let source = Decoder::new(reader)?;
    stream_handle.play_raw(source.convert_samples())?;

    /* The sound plays in a separate audio thread, so we need to keep the main thread alive while it's playing.
    The file size is used as an approximation of the audio length. */
    sleep(Duration::from_millis(max(length as u64 / 10, 1_750)));

    Ok(())
}
