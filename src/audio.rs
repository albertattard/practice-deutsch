use rand::Rng;
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
    let mut files = Vec::new();

    loop {
        if files.is_empty() {
            files = list_audio_files_in_directory(directory);
            if files.is_empty() {
                println!("No audio files found in {}", directory);
                return;
            }

            println!("Loaded {} audio files from {}", files.len(), directory);
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..files.len());
        let file = files.remove(index as usize);
        if !play_file_and_verify(&file) {
            return;
        }
    }
}

fn list_audio_files_in_directory(directory: &str) -> Vec<PathBuf> {
    fs::read_dir(directory)
        .expect(&format!("Failed to read {}", directory))
        .map(|r| r.unwrap().path())
        .collect()
}

fn play_file_and_verify(file: &PathBuf) -> bool {
    if let Err(e) = play_file(file) {
        println!("Failed to play audio file: {:?} ({})", file, e);
        return true;
    }

    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read the user input");
        let input = input.trim();

        match input {
            "q" | "quit" => return false,
            "" | "r" | "repeat" => {
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
                return true;
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
