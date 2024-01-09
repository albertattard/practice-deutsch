use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, io};

use crate::types::nouns::Noun;

pub(crate) fn download() {
    download_missing_nouns_from_verbformen();
    download_missing_nouns_from_collins_dictionary();
    manual::download_missing_nouns();
    manual::download_missing_verbs();
    // satzapp::download_missing_phrases();
    println!("Done");
}

fn download_missing_nouns_from_verbformen() {
    println!("Downloading missing nouns from verbformen.de");

    fn download_missing_noun(file: &Path, noun: &str) {
        if file.exists() {
            return;
        }

        let link_noun = noun
            .replace("Ä", "A3")
            .replace("Ö", "O3")
            .replace("Ü", "U3")
            .replace("ä", "a3")
            .replace("ö", "o3")
            .replace("ü", "u3")
            .replace("ß", "s5");

        if let Err(_) = download_file(
            &format!(
                "https://www.verbformen.de/deklination/substantive/grundform/{}.mp3",
                link_noun
            ),
            &file,
        ) {
            println!("Failed to download audio file from: {}", link_noun);
        }

        sleep(Duration::from_secs(1));
    }

    for noun in Noun::read() {
        download_missing_noun(&noun.singular_file_path(), &noun.singular);
        download_missing_noun(
            &noun.singular_with_article_file_path(),
            &format!("der_{}", &noun.singular),
        );
    }
}

fn download_missing_nouns_from_collins_dictionary() {
    println!("Downloading missing nouns from collinsdictionary.com");

    for noun in Noun::read() {
        if let None = noun.plural {
            continue;
        }

        let file = noun.plural_file_path();
        if file.is_file() {
            continue;
        }

        let link_noun = noun
            .plural
            .clone()
            .unwrap()
            .replace("Ä", "A")
            .replace("Ö", "O")
            .replace("Ü", "U")
            .replace("ä", "a")
            .replace("ö", "o")
            .replace("ü", "u")
            .to_lowercase();

        if let Err(_) = download_file(
            &format!(
                "https://www.collinsdictionary.com/sounds/hwd_sounds/de_{}.mp3",
                link_noun
            ),
            &file,
        ) {
            println!("{}", &noun.singular);
        }

        sleep(Duration::from_secs(1));
    }
}

mod manual {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::PathBuf;

    use base64::{engine::general_purpose, Engine as _};

    use crate::types::audio::play_file;
    use crate::types::nouns::Noun;
    use crate::types::utils::read_line;
    use crate::types::verbs::{Pronoun, Verb};

    pub(super) fn download_missing_nouns() {
        println!("Downloading missing nouns manually");

        for noun in Noun::read() {
            download_manually(&noun.singular, &noun.singular_file_path());
            download_manually(
                &format!("{} {}", &noun.article, &noun.singular),
                &noun.singular_with_article_file_path(),
            );
            if let Some(plural) = &noun.plural {
                download_manually(&plural, &noun.plural_file_path());
                download_manually(
                    &format!("die {}", &plural),
                    &noun.plural_with_article_file_path(),
                );
            };
        }
    }

    pub(super) fn download_missing_verbs() {
        println!("Downloading missing verbs manually");

        for verb in Verb::read() {
            download_manually(&verb.infinitive(), &verb.infinitive_audio_file_path());
            for pronoun in Pronoun::iter() {
                download_manually(
                    &verb.pronoun_conjugation(pronoun),
                    &verb.conjugation_audio_file_path(pronoun),
                );
            }
        }
    }

    fn download_manually(text: &String, file: &PathBuf) {
        if file.exists() {
            return;
        }

        let temp_base64_file = "target/tmp.base64";
        File::create(temp_base64_file).unwrap();

        let file_name = &file.file_name().unwrap().to_str().unwrap();
        read_line(&format!("{} ({})", text, file_name));

        let mut base64 = String::new();
        File::open(temp_base64_file)
            .unwrap()
            .read_to_string(&mut base64)
            .unwrap();

        if base64.is_empty() {
            panic!("No base64 string found");
        }

        let bytes = general_purpose::STANDARD.decode(base64).unwrap();
        let mut audio_file = File::create(file).unwrap();
        audio_file.write_all(&bytes).unwrap();

        play_file(file).unwrap();
    }
}

mod satzapp {
    use std::thread::sleep;

    use scraper::Node::Element;

    use crate::types::nouns::Noun;
    use crate::types::phrases::{Phrase, Phrases};

    pub(super) fn download() {
        let text = "Ananas";
        let content = request_phrases(&text);

        let document = scraper::Html::parse_document(&content);
        let selector = scraper::Selector::parse("hr").unwrap();
        let element = document.select(&selector).next().unwrap();

        let mut a = element.next_sibling().unwrap();

        loop {
            if let Element(e) = a.next_sibling().unwrap().value() {
                if "hr".eq_ignore_ascii_case(e.name()) {
                    break;
                }

                println!("{:?}", e);
            }
            a = a.next_sibling().unwrap();
        }
    }

    pub(super) fn download_missing_phrases() {
        println!("Downloading missing phrases from satzapp.com");

        let mut phrases = Phrases::read();

        for noun in Noun::read() {
            println!("Downloading phrases for {}", noun.singular);

            let text = noun.singular;
            let content = request_phrases(&text);
            let mut new_phrases = parse_phrases(&content);

            println!(
                "Found {} phrases that contains the text {}",
                new_phrases.len(),
                text
            );
            phrases.append(&mut new_phrases);
            phrases.write();

            sleep(std::time::Duration::from_secs(1));
        }
    }

    fn request_phrases(noun: &str) -> String {
        let link = &format!("https://www.satzapp.com/saetze/?w={}", noun);

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(link)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0",
            )
            .header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
            )
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Connection", "keep-alive")
            .header("Upgrade-Insecure-Requests", "1")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "none")
            .header("Sec-Fetch-User", "?1")
            .send()
            .unwrap();

        if !response.status().is_success() {
            panic!("Failed to request phrases from {}", link);
        }

        response.text().unwrap()
    }

    fn parse_phrases(content: &String) -> Vec<Phrase> {
        fn parse(document: &scraper::Html, css_selector: &str) -> Vec<String> {
            let selector = scraper::Selector::parse(css_selector).unwrap();
            document
                .select(&selector)
                .map(|phrase| phrase.text().collect::<String>())
                .map(|phrase| phrase.trim().to_string())
                .collect()
        }

        let document = scraper::Html::parse_document(content);

        let german_phrases = parse(&document, "span[class=sTxt]");
        let english_phrases = parse(&document, "p[class~=rLinks]");

        if german_phrases.len() != english_phrases.len() {
            println!("Found {} German phrases", german_phrases.len());
            for phrase in german_phrases.iter().enumerate() {
                println!("{}) {}", phrase.0, phrase.1);
            }

            println!("Found {} English phrases", english_phrases.len());
            for phrase in english_phrases.iter().enumerate() {
                println!("{}) {}", phrase.0, phrase.1);
            }

            vec![]
        } else {
            std::iter::zip(german_phrases, english_phrases)
                .map(|pair| Phrase::new(pair.0, pair.1))
                .collect()
        }
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
