use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};

use rand::Rng;

use crate::types::audio::play_file_or_print_error;

pub(crate) fn articles() {
    let mut nouns: Vec<Noun> = Vec::new();

    loop {
        if nouns.is_empty() {
            nouns = Noun::read();
            if nouns.is_empty() {
                println!("No nouns found");
                return;
            }

            println!("----------------------------------------");
            println!("Loaded {} nouns", nouns.len());
            println!("----------------------------------------");
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..nouns.len());
        let noun = nouns.remove(index);

        let question = &noun.singular_question();
        println!("{} ({}): ", question.noun, question.english);

        question.play();

        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to user input");

            let input = &input.trim().to_ascii_lowercase();
            match input.as_str() {
                "quit" | "exit" => return,
                "" | "repeat" => {
                    question.play();
                    continue;
                }
                "die" | "der" | "das" => {
                    if !question.article.contains(input) {
                        print!("Wrong! ");
                        question.play_with_article();
                        nouns.push(noun);
                    };
                    break;
                }
                _ => {
                    println!("Expected the articles der, die or das");
                    println!("         quit to quit");
                    println!("         repeat to replay the audio");
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

pub(crate) fn plural() {
    let mut nouns: Vec<Noun> = Vec::new();

    loop {
        if nouns.is_empty() {
            nouns = Noun::read();
            if nouns.is_empty() {
                println!("No nouns found");
                return;
            }

            println!("----------------------------------------");
            println!("Loaded {} nouns", nouns.len());
            println!("----------------------------------------");
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..nouns.len());
        let noun = nouns.remove(index);

        print!("{} ({}): ", noun.singular, noun.english);
        stdout().flush().unwrap();
        noun.play_singular();

        let plural = noun.plural.clone().unwrap();

        let mut incorrect = false;
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to user input");

            let input = input.trim();
            match input {
                "quit" | "exit" => return,
                "" | "repeat" => {
                    noun.play_singular();
                    continue;
                }
                input => {
                    if !plural.eq(input) {
                        print!("Wrong! ");
                        noun.play_plural();
                        incorrect = true;
                    };
                    break;
                }
            }
        }

        println!("Correct answer: {}", plural);
        noun.play_plural();

        if incorrect {
            nouns.push(noun);
        }
    }
}

pub(crate) fn list_nouns_plurals() -> Vec<String> {
    Noun::read()
        .iter()
        .filter(|noun| match noun.plural {
            Some(_) => true,
            None => false,
        })
        .map(|noun| noun.plural.clone().unwrap())
        .collect()
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Noun {
    pub(crate) english: String,
    pub(crate) article: String,
    pub(crate) singular: String,
    pub(crate) plural: Option<String>,
}

struct NounQuestion {
    english: String,
    noun: String,
    article: Vec<String>,
    file_path: PathBuf,
    with_article_file_path: PathBuf,
}

impl Noun {
    pub(crate) fn read() -> Vec<Noun> {
        let reader = csv::Reader::from_path("nouns.csv");
        reader
            .expect("Failed to read nouns")
            .deserialize()
            .map(|r| r.unwrap())
            .collect()
    }

    pub(crate) fn singular_file_path(&self) -> PathBuf {
        Path::new("audio/nouns")
            .join(&self.singular)
            .with_extension("mp3")
    }

    pub(crate) fn singular_with_article_file_path(&self) -> PathBuf {
        Path::new("audio/nouns")
            .join(&format!("{} {}", &self.article, &self.singular))
            .with_extension("mp3")
    }

    pub(crate) fn plural_file_path(&self) -> PathBuf {
        Path::new("audio/nouns")
            .join(&self.plural.clone().unwrap())
            .with_extension("mp3")
    }

    pub(crate) fn plural_with_article_file_path(&self) -> PathBuf {
        Path::new("audio/nouns")
            .join(&format!("die {}", &self.plural.clone().unwrap()))
            .with_extension("mp3")
    }

    fn play_singular(&self) {
        play_file_or_print_error(&self.singular_file_path());
    }

    fn play_plural(&self) {
        play_file_or_print_error(&self.plural_file_path());
    }

    fn random_question(&self) -> NounQuestion {
        let mut rng = rand::thread_rng();

        if None == self.plural || rng.gen_bool(0.5) {
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
        }
    }

    fn plural_question(&self) -> NounQuestion {
        let noun = match &self.plural {
            Some(t) => t.to_owned(),
            None => panic!("Plural form is not available for noun: {}", &self.singular),
        };

        /* When the plural is the same as the singular form, like Fenster, the user cannot tell apart so we need to
        accept both articles */
        let mut article = vec!["die".to_owned()];
        if !"die".eq(&self.article) && noun.eq(&self.singular) {
            article.push(self.article.to_owned());
        }

        NounQuestion {
            english: self.english.to_owned(),
            noun,
            article,
            file_path: self.plural_file_path(),
            with_article_file_path: self.plural_with_article_file_path(),
        }
    }
}

impl Display for Noun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({})", self.article, self.singular, self.english)
    }
}

impl NounQuestion {
    fn play(&self) {
        play_file_or_print_error(&self.file_path);
    }

    fn play_with_article(&self) {
        play_file_or_print_error(&self.with_article_file_path);
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::types::nouns::Noun;

    #[test]
    fn read_all() {
        let nouns = Noun::read();
        assert_eq!(nouns.len(), count_entries_in_csv_file());
    }

    fn count_entries_in_csv_file() -> usize {
        let file = File::open("nouns.csv").expect("Failed to open file");
        BufReader::new(file).lines().count() - 1
    }
}
