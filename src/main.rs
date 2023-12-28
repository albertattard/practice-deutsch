use std::{error::Error, fs};
use std::{io, io::BufReader, io::Cursor, io::stdin};
use std::fs::File;
use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};
use rand::{Rng, seq::SliceRandom};

fn main() {
    let args = Args::parse();

    match args.mode {
        Mode::Articles => articles(),
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
        stdin().read_line(&mut input).expect("Failed to read the user input");
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

fn articles() {
    let nouns: Vec<Noun> = read_nouns().expect("Failed to read nouns");
    println!("Found {} nouns", nouns.len());

    loop {
        let mut rng = rand::thread_rng();
        let noun = nouns.choose(&mut rng).unwrap();

        let question = &noun.random_question();
        println!("{} ({}): ", question.noun, question.english);

        if let Err(e) = play_noun(&question) {
            println!("Failed to play audio: {}", e);
        }

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to user input");

        let input = input.trim().to_ascii_lowercase();
        match input.as_str() {
            "q" => break,
            article => if !question.article.contains(&article.to_owned()) {
                print!("Wrong! ");
            }
        }

        print!("Correct answer: {}", question.article[0]);
        if question.article.len() > 1 {
            print!(" (or {})", question.article[1]);
        }
        println!(" {}", question.noun);

        if let Err(e) = play_noun_with_article(&question) {
            println!("Failed to play audio: {}", e);
        }
    }
}

fn play_noun(noun: &NounQuestion) -> Result<(), Box<dyn Error>> {
    if !noun.file_path.is_file() {
        download_file(noun.file_link.as_str(), noun.file_path.as_path())?;
    }

    play_file(noun.file_path.as_path())
}

fn play_noun_with_article(noun: &NounQuestion) -> Result<(), Box<dyn Error>> {
    if !noun.with_article_file_path.is_file() {
        download_file(noun.with_article_file_link.as_str(), noun.with_article_file_path.as_path())?;
    }

    play_file(noun.with_article_file_path.as_path())
}

fn play_file(path: &Path) -> Result<(), Box<dyn Error>> {
    /* Based on: https://docs.rs/rodio/latest/rodio/ */
    use rodio::{Decoder, OutputStream, source::Source};

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?;
    stream_handle.play_raw(source.convert_samples())?;

    /* The sound plays in a separate audio thread,
        so we need to keep the main thread alive while it's playing. */
    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}

fn download_file(link: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    match path.parent() {
        Some(parent) => fs::create_dir_all(parent)?,
        None => {}
    };

    let response = reqwest::blocking::get(link)?;

    if response.status().is_success() {
        let mut content = Cursor::new(response.bytes()?);
        let mut file = File::create(path)?;
        io::copy(&mut content, &mut file)?;
        Ok(())
    } else {
        Err(Box::from("Failed to download audio file"))
    }
}

fn read_nouns() -> Result<Vec<Noun>, Box<dyn Error>> {
    let reader = csv::Reader::from_path("nouns.csv");

    let nouns: Vec<Noun> = reader?.deserialize()
        .map(|r| r.unwrap())
        .collect();

    Ok(nouns)
}

#[derive(Debug, serde::Deserialize)]
struct Noun {
    english: String,
    article: String,
    singular: String,
    plural: Option<String>,
    audio_file_name: Option<String>,
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
        let file_name = match &self.audio_file_name {
            Some(file_name) => file_name.to_owned(),
            None => clean_file_name(&self.singular)
        };
        format!("https://www.verbformen.de/deklination/substantive/grundform/{}.mp3", file_name)
    }

    fn singular_with_article_file_link(&self) -> String {
        let file_name = match &self.audio_file_name {
            Some(file_name) => file_name.to_owned(),
            None => clean_file_name(&self.singular)
        };
        format!("https://www.verbformen.de/deklination/substantive/grundform/der_{}.mp3", file_name)
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
    name
        .replace("Ä", "A3")
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
    #[clap(name = "letters")]
    Alphabet,
    #[clap(name = "numbers")]
    Numbers,
}
