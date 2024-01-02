use crate::audio::play_file;
use crate::download::download_file;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::error::Error;
use std::io::stdin;
use std::path::{Path, PathBuf};

pub(crate) fn articles() {
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
                "" | "r" | "repeat" => {
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
    fn read_nouns() -> Result<Vec<Noun>, Box<dyn Error>> {
        let reader = csv::Reader::from_path("nouns.csv");

        let nouns: Vec<Noun> = reader?.deserialize().map(|r| r.unwrap()).collect();

        Ok(nouns)
    }

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
            self.clean_singular_file_name()
        )
    }

    fn singular_with_article_file_link(&self) -> String {
        format!(
            "https://www.verbformen.de/deklination/substantive/grundform/der_{}.mp3",
            self.clean_singular_file_name()
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

    fn clean_singular_file_name(&self) -> String {
        self.singular
            .replace("Ä", "A3")
            .replace("Ö", "O3")
            .replace("Ü", "U3")
            .replace("ä", "a3")
            .replace("ö", "o3")
            .replace("ü", "u3")
    }
}

impl NounQuestion {
    fn play(&self) {
        download_if_missing_and_play(self.file_path.as_path(), &self.file_link);
    }

    fn play_with_article(&self) {
        download_if_missing_and_play(
            self.with_article_file_path.as_path(),
            &self.with_article_file_link,
        );
    }
}

fn download_if_missing_and_play(path: &Path, link: &str) {
    if !path.is_file() {
        if let Err(e) = download_file(link, path) {
            println!("Failed to download audio file from: {} ({})", link, e);
            return;
        }
    }

    if let Err(e) = play_file(path) {
        println!("Failed to play audio file: {:?} ({})", path, e);
    }
}
