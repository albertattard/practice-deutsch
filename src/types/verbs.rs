use crate::types::audio::play_file;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::slice::Iter;

pub(crate) fn verbs() {
    let mut verbs = Vec::new();

    loop {
        if verbs.is_empty() {
            verbs = Verb::read();
            if verbs.is_empty() {
                println!("No verbs found");
                return;
            }

            println!("----------------------------------------");
            println!("Loaded {} verbs", verbs.len());
            println!("----------------------------------------");
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..verbs.len());
        let verb = verbs.remove(index);

        println!("{} ({}): ", verb.german, verb.english);
        verb.play();

        let mut incorrect = false;

        for pronoun in Pronoun::iter() {
            print!("{}: ", pronoun);
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to user input");

            let input = &input.trim().to_ascii_lowercase();
            match input.as_str() {
                "quit" => return,
                input => {
                    let conjugation = verb.conjugation(pronoun);
                    if conjugation != input {
                        println!("Wrong! Correct answer is {} {}", pronoun, conjugation);
                        incorrect = true;
                    };
                    verb.play_conjugation(pronoun);
                }
            }
        }

        if incorrect {
            verbs.push(verb);
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct Verb {
    english: String,
    german: String,
    ich: String,
    du: String,
    er: String,
    wir: String,
    ihr: String,
    sie: String,
}

enum Pronoun {
    Ich,
    Du,
    SieFormal,
    Er,
    Sie,
    Es,
    Man,
    Wir,
    Ihr,
    SiePluralFormal,
    SiePlural,
}

impl Verb {
    fn read() -> Vec<Verb> {
        let reader = csv::Reader::from_path("verbs.csv");

        reader
            .expect("Failed to read verbs")
            .deserialize()
            .map(|r| r.unwrap())
            .collect()
    }

    fn conjugation(&self, pronoun: &Pronoun) -> String {
        match pronoun {
            Pronoun::Ich => self.ich.clone(),
            Pronoun::Du => self.du.clone(),
            Pronoun::SieFormal => self.sie.clone(),
            Pronoun::Er => self.er.clone(),
            Pronoun::Sie => self.er.clone(),
            Pronoun::Es => self.er.clone(),
            Pronoun::Man => self.er.clone(),
            Pronoun::Wir => self.wir.clone(),
            Pronoun::Ihr => self.ihr.clone(),
            Pronoun::SiePluralFormal => self.sie.clone(),
            Pronoun::SiePlural => self.sie.clone(),
        }
    }

    fn play(&self) {
        let file = Path::new("audio/verbs")
            .join(&self.german)
            .with_extension("mp3");

        Self::play_file(file);
    }

    fn play_conjugation(&self, pronoun: &Pronoun) {
        let file = Path::new("audio/verbs")
            .join(&format!("{} {}", pronoun, self.conjugation(pronoun)))
            .with_extension("mp3");

        Self::play_file(file);
    }

    fn play_file(file: PathBuf) {
        if file.exists() {
            if let Err(e) = play_file(file.as_path()) {
                println!("Failed to play audio file: {:?} ({})", file, e);
            }
        } else {
            println!("Audio file not found: {:?}", file);
        }
    }
}

impl Pronoun {
    pub fn iter() -> Iter<'static, Pronoun> {
        static PRONOUNS: [Pronoun; 11] = [
            Pronoun::Ich,
            Pronoun::Du,
            Pronoun::SieFormal,
            Pronoun::Er,
            Pronoun::Sie,
            Pronoun::Es,
            Pronoun::Man,
            Pronoun::Wir,
            Pronoun::Ihr,
            Pronoun::SiePluralFormal,
            Pronoun::SiePlural,
        ];
        PRONOUNS.iter()
    }
}

impl Display for Pronoun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let p = match &self {
            Pronoun::Ich => "ich",
            Pronoun::Du => "du",
            Pronoun::SieFormal => "Sie",
            Pronoun::Er => "er",
            Pronoun::Sie => "sie",
            Pronoun::Es => "es",
            Pronoun::Man => "man",
            Pronoun::Wir => "wir",
            Pronoun::Ihr => "ihr",
            Pronoun::SiePluralFormal => "Sie",
            Pronoun::SiePlural => "sie",
        };
        write!(f, "{}", p)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::verbs::Verb;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn read_all() {
        let verbs = Verb::read();

        assert_eq!(verbs.len(), count_entries_in_csv_file());
    }

    fn count_entries_in_csv_file() -> usize {
        let file = File::open("verbs.csv").expect("Failed to open file");
        BufReader::new(file).lines().count() - 1
    }
}
