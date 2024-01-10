use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use crate::types::audio::play_file_or_print_error;
use crate::types::utils::{play_and_read_line, read_line, remove_random};

pub(crate) fn articles() {
    let mut nouns: Vec<Noun> = Noun::read();
    if nouns.is_empty() {
        println!("No nouns found");
        return;
    }

    println!("----------------------------------------");
    println!("Loaded {} nouns", nouns.len());
    println!("----------------------------------------");

    play_file_or_print_error(Path::new("./audio/program/articles.mp3"));

    loop {
        let noun = remove_random(&mut nouns);
        let mut repeat_noun = false;

        loop {
            let input = &play_and_read_line(
                &format!("{} ({})", noun.singular, noun.english),
                &noun.singular_file_path(),
            )
            .to_lowercase();

            match input.as_str() {
                "quit" | "exit" => return,
                "" | "repeat" => {
                    continue;
                }
                "die" | "der" | "das" => {
                    if !noun.article.eq_ignore_ascii_case(input) {
                        print!("Wrong! ");
                        noun.play_singular_with_article();
                        repeat_noun = true;
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

        println!("Correct answer: {} {}", noun.article, noun.singular);
        noun.play_singular_with_article();

        if repeat_noun {
            nouns.push(noun);
        } else if nouns.is_empty() {
            break;
        }
    }
}

pub(crate) fn plural() {
    let mut nouns: Vec<Noun> = Noun::read();
    /* Keep it simple for now */
    nouns.retain(|noun| noun.plural.is_some() && noun.singular.len() <= 4);

    if nouns.is_empty() {
        println!("No plural nouns found (matching criteria)");
        return;
    }

    println!("----------------------------------------");
    println!("Loaded {} plural nouns", nouns.len());
    println!("----------------------------------------");

    loop {
        let noun = remove_random(&mut nouns);
        let plural = noun.plural.clone().unwrap();

        let mut repeat_noun = false;

        loop {
            noun.play_singular();
            noun.play_singular_with_article();

            let input = read_line(&format!("{} ({}) [ÄÖÜäöüß]", noun.singular, noun.english));

            match input.as_str() {
                "quit" | "exit" => return,
                "" | "repeat" => {
                    continue;
                }
                input => {
                    noun.play_plural();
                    noun.play_plural_with_article();

                    if plural.eq(input) {
                        println!("Correct answer: {}", plural);
                        break;
                    } else {
                        println!("Wrong! Correct answer: {}", plural);
                        repeat_noun = true;
                    }
                }
            }
        }

        if repeat_noun {
            nouns.push(noun);
        } else if nouns.is_empty() {
            break;
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Noun {
    pub(crate) english: String,
    pub(crate) article: String,
    pub(crate) singular: String,
    pub(crate) plural: Option<String>,
}

impl Noun {
    pub(crate) fn read() -> Vec<Noun> {
        csv::Reader::from_path("nouns.csv")
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

    fn play_singular_with_article(&self) {
        play_file_or_print_error(&self.singular_with_article_file_path());
    }

    fn play_plural(&self) {
        play_file_or_print_error(&self.plural_file_path());
    }

    fn play_plural_with_article(&self) {
        play_file_or_print_error(&self.plural_with_article_file_path());
    }
}

impl Display for Noun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({})", self.article, self.singular, self.english)
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
