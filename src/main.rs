use std::io::stdin;

use clap::Parser;
use rand::seq::SliceRandom;

use crate::audio::pronounce;
use crate::nouns::Noun;

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

fn articles() {
    let nouns: Vec<Noun> = Noun::read_nouns().expect("Failed to read nouns");

    loop {
        let mut rng = rand::thread_rng();
        let noun = nouns.choose(&mut rng).unwrap();

        let question = &noun.random_question();
        println!("{} ({}): ", question.noun, question.english);

        question.play();

        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to user input");

            let input = &input.trim().to_ascii_lowercase();
            match input.as_str() {
                "q" | "quit" => return,
                "r" | "repeat" => {
                    question.play();
                    continue;
                }
                "die" | "der" | "das" => {
                    if !question.article.contains(input) {
                        print!("Wrong! ");
                        question.play_with_article();
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

        question.play_with_article();
    }
}
