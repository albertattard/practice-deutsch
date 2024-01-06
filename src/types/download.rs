use crate::types::audio::play_file;
use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::types::nouns::Noun;
use crate::types::utils::read_line;

pub(crate) fn download_file(link: &str, path: &Path) -> Result<(), Box<dyn Error>> {
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

pub(crate) fn download_missing_nouns_manually() {
    fn download_manually(text: &String, file: &PathBuf) {
        use base64::{engine::general_purpose, Engine as _};

        if !file.exists() {
            let file_name = &file.file_name().unwrap().to_str().unwrap();
            read_line(&format!("{} ({})", text, file_name));

            let mut base64 = String::new();
            File::open("src/resources/tmp.base64")
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

            File::create("src/resources/tmp.base64").unwrap();
        }
    }

    for noun in Noun::read() {
        download_manually(&noun.singular, &noun.singular_file_path());
        download_manually(
            &format!("{} {}", &noun.article, &noun.singular),
            &noun.singular_with_article_file_path(),
        );
        if let Some(plural) = &noun.plural {
            if plural.len() > 9 {
                continue;
            }

            download_manually(&plural, &noun.plural_file_path());
            download_manually(
                &format!("die {}", &plural),
                &noun.plural_with_article_file_path(),
            );
        };
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use std::thread::sleep;
    use std::time::Duration;

    use crate::types::download::download_file;
    use crate::types::nouns::Noun;

    #[test]
    #[test]
    fn download_missing_nouns_from_verbformen() {
        let skip = read_skip_file("skip_nouns_from_verbformen");

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
            if skip.contains(&noun.singular) {
                println!("Skipping {}", &noun);
                continue;
            }

            download_missing_noun(&noun.singular_file_path(), &noun.singular);
            download_missing_noun(
                &noun.singular_with_article_file_path(),
                &format!("der_{}", &noun.singular),
            );
        }
    }

    #[test]
    fn download_missing_nouns_from_collins_dictionary() {
        let skip = read_skip_file("skip_nouns_from_collins_dictionary");

        for noun in Noun::read() {
            if skip.contains(&noun.singular) {
                println!("Skipping {}", &noun);
                continue;
            }

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

    fn read_skip_file(file_name: &str) -> Vec<String> {
        let file = File::open(
            &Path::new("src/resources")
                .join(file_name)
                .with_extension("csv"),
        )
        .expect("Failed to open file");
        BufReader::new(file)
            .lines()
            .skip(1)
            .map(|l| l.unwrap())
            .collect()
    }
}
