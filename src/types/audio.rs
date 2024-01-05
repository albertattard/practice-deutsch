use rand::Rng;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{stdin, BufReader};
use std::path::{Path, PathBuf};

use rodio::{Decoder, OutputStream, Sink};

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
            "quit" | "exit" => return false,
            "" | "repeat" => {
                play_file_or_print_error(file);
                continue;
            }
            input => {
                let expected = file.file_stem().unwrap().to_str().unwrap();
                if !expected.eq(input) {
                    println!("Wrong! It was: {}", expected);
                    play_file_or_print_error(file);
                }
                return true;
            }
        }
    }
}

pub(crate) fn play_file_or_print_error(file: &Path) {
    if file.exists() {
        if let Err(e) = play_file(file) {
            println!("Failed to play audio file: {:?} ({})", file, e);
        }
    } else {
        println!("File not found: {:?}", file);
    }
}

pub(crate) fn play_file(path: &Path) -> Result<(), Box<dyn Error>> {
    /* Based on: https://docs.rs/rodio/latest/rodio/ */

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let source = Decoder::new(reader)?;
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    /* The sound plays in a separate audio thread, so we need to keep the main thread alive while it's playing. */
    sink.sleep_until_end();

    Ok(())
}
