use std::fs::File;
use std::path::{Path, PathBuf};
use std::{error::Error, fs};
use std::{io, io::stdin, io::Cursor};

use clap::Parser;
use rand::seq::SliceRandom;

use crate::audio::play_file;
use crate::nouns::{Noun, NounQuestion};

mod audio;
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

fn pronounce(directory: &str) {
    let files: Vec<PathBuf> = fs::read_dir(directory)
        .expect(format!("Failed to read {}", directory).as_str())
        .map(|r| r.unwrap().path())
        .collect();

    loop {
        let mut rng = rand::thread_rng();
        let file = files.choose(&mut rng).unwrap();

        if let Err(e) = play_file(file) {
            println!("Failed to play audio file: {:?} ({})", file, e);
            continue;
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
}

fn plural() {
    println!("Not implemented yet");
}

fn verbs() {
    println!("Not implemented yet");
}

fn articles() {
    let nouns: Vec<Noun> = Noun::read_nouns().expect("Failed to read nouns");

    loop {
        let mut rng = rand::thread_rng();
        let noun = nouns.choose(&mut rng).unwrap();

        let question = &noun.random_question();
        println!("{} ({}): ", question.noun, question.english);

        play_noun(&question);

        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to user input");

            let input = &input.trim().to_ascii_lowercase();
            match input.as_str() {
                "q" | "quit" => return,
                "r" | "repeat" => {
                    play_noun(&question);
                    continue;
                }
                "die" | "der" | "das" => {
                    if !question.article.contains(input) {
                        print!("Wrong! ");
                        play_noun_with_article(&question);
                    };
                    break;
                }
                _ => {
                    println!("Expected the articles der, die or das");
                    println!("         q or quit to quit");
                    println!("         r or repeat to replay the audio");
                    continue;
                }
            }
        }

        print!("Correct answer: {}", question.article[0]);
        if question.article.len() > 1 {
            print!(" (or {})", question.article[1]);
        }
        println!(" {}", question.noun);

        play_noun_with_article(&question);
    }
}

fn play_noun(noun: &NounQuestion) {
    let path = noun.file_path.as_path();

    if !noun.file_path.is_file() {
        let link = noun.file_link.as_str();
        if let Err(e) = download_file(link, path) {
            println!("Failed to download audio file from: {} ({})", link, e);
            return;
        }
    }

    if let Err(e) = play_file(path) {
        println!("Failed to play audio file: {:?} ({})", noun.file_path, e);
    }
}

fn play_noun_with_article(noun: &NounQuestion) {
    let path = noun.with_article_file_path.as_path();

    if !noun.with_article_file_path.is_file() {
        let link = noun.with_article_file_link.as_str();
        if let Err(e) = download_file(link, path) {
            println!("Failed to download audio file from: {} ({})", link, e);
            return;
        }
    }

    if let Err(e) = play_file(path) {
        println!(
            "Failed to play audio file: {:?} ({})",
            noun.with_article_file_path, e
        );
    }
}

fn download_file(link: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Downloading audio from {} to {}", link, path.display());

    let response = reqwest::blocking::get(link)?;

    if response.status().is_success() {
        let mut content = Cursor::new(response.bytes()?);
        let mut file = File::create(path)?;
        create_parent_directory_if_missing(path)?;
        io::copy(&mut content, &mut file)?;
        Ok(())
    } else {
        Err(Box::from("Failed to download audio file"))
    }
}

fn create_parent_directory_if_missing(path: &Path) -> Result<(), Box<dyn Error>> {
    match path.parent() {
        Some(parent) => fs::create_dir_all(parent)?,
        None => {}
    };

    Ok(())
}
