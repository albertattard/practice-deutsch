use std::fs::File;
use std::path::{Path, PathBuf};
use std::{error::Error, fs};
use std::{io, io::stdin, io::BufReader, io::Cursor};

use clap::{Parser, ValueEnum};
use rand::{seq::SliceRandom, Rng};

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

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read the user input");
        let input = input.trim();

        match input {
            "q" => break,
            input => {
                let expected = file.file_stem().unwrap().to_str().unwrap();
                if !expected.eq(input) {
                    println!("Wrong! It was: {}", expected);
                    if let Err(e) = play_file(file) {
                        println!("Failed to replay play audio file: {:?} ({})", file, e);
                    }
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
    let nouns: Vec<Noun> = read_nouns().expect("Failed to read nouns");

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

fn play_file(path: &Path) -> Result<(), Box<dyn Error>> {
    /* Based on: https://docs.rs/rodio/latest/rodio/ */
    use rodio::{source::Source, Decoder, OutputStream};

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?;
    stream_handle.play_raw(source.convert_samples())?;

    /* The sound plays in a separate audio thread,
    so we need to keep the main thread alive while it's playing. */
    std::thread::sleep(std::time::Duration::from_millis(1_500));

    Ok(())
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

fn read_nouns() -> Result<Vec<Noun>, Box<dyn Error>> {
    let reader = csv::Reader::from_path("nouns.csv");

    let nouns: Vec<Noun> = reader?.deserialize().map(|r| r.unwrap()).collect();

    Ok(nouns)
}

#[derive(Debug, serde::Deserialize)]
struct Noun {
    english: String,
    article: String,
    singular: String,
    plural: Option<String>,
}

struct NounQuestion {
    english: String,
    noun: String,
    article: Vec<String>,
    file_path: PathBuf,
    with_article_file_path: PathBuf,
    file_link: String,
    with_article_file_link: String,
}

impl Noun {
    fn singular_file_path(&self) -> PathBuf {
        Path::new("audio/nouns")
            .join(&self.singular)
            .with_extension("mp3")
    }

    fn singular_with_article_file_path(&self) -> PathBuf {
        Path::new("audio/nouns")
            .join(format!("{} {}", &self.article, &self.singular).as_str())
            .with_extension("mp3")
    }

    fn singular_file_link(&self) -> String {
        format!(
            "https://www.verbformen.de/deklination/substantive/grundform/{}.mp3",
            clean_file_name(&self.singular)
        )
    }

    fn singular_with_article_file_link(&self) -> String {
        format!(
            "https://www.verbformen.de/deklination/substantive/grundform/der_{}.mp3",
            clean_file_name(&self.singular)
        )
    }

    fn random_question(&self) -> NounQuestion {
        let mut rng = rand::thread_rng();

        if rng.gen_bool(1.0) {
            self.singular_question()
        } else {
            self.plural_question()
        }
    }

    fn singular_question(&self) -> NounQuestion {
        NounQuestion {
            english: self.english.to_owned(),
            noun: self.singular.to_owned(),
            article: vec![self.article.to_owned()],
            file_path: self.singular_file_path(),
            with_article_file_path: self.singular_with_article_file_path(),
            file_link: self.singular_file_link(),
            with_article_file_link: self.singular_with_article_file_link(),
        }
    }

    fn plural_question(&self) -> NounQuestion {
        let noun = match &self.plural {
            Some(t) => t.to_owned(),
            None => self.singular.to_owned(),
        };

        let mut article = vec!["die".to_owned()];
        if !"die".eq(&self.article) && noun.eq(&self.singular) {
            article.push(self.article.to_owned());
        }

        NounQuestion {
            english: self.english.to_owned(),
            noun,
            article,
            file_path: self.singular_file_path(),
            with_article_file_path: self.singular_with_article_file_path(),
            file_link: self.singular_file_link(),
            with_article_file_link: self.singular_with_article_file_link(),
        }
    }
}

fn clean_file_name(name: &str) -> String {
    name.replace("Ä", "A3")
        .replace("Ö", "O3")
        .replace("Ü", "U3")
        .replace("ä", "a3")
        .replace("ö", "o3")
        .replace("ü", "u3")
}

/// Simple program to help me learn the German language
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The mode to run the program in
    #[clap(value_enum, default_value_t = Mode::Articles)]
    mode: Mode,
}

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    #[clap(name = "articles")]
    Articles,
    #[clap(name = "plural")]
    Plural,
    #[clap(name = "verbs")]
    Verbs,
    #[clap(name = "letters")]
    Alphabet,
    #[clap(name = "numbers")]
    Numbers,
}
