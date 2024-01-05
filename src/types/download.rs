use crate::types::nouns::list_nouns_plurals;
use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::thread::sleep;
use std::{fs, io};

pub(crate) fn download_missing_nouns_from_collins_dictionary() {
    for noun in list_nouns_plurals() {
        let link_noun = noun
            .replace("Ä", "A")
            .replace("Ö", "O")
            .replace("Ü", "U")
            .replace("ä", "a")
            .replace("ö", "o")
            .replace("ü", "u")
            .to_lowercase();

        let path_buf = std::path::Path::new("audio/nouns")
            .join(noun)
            .with_extension("mp3");

        if path_buf.is_file() {
            continue;
        }

        if let Err(_) = download_file(
            &format!(
                "https://www.collinsdictionary.com/sounds/hwd_sounds/de_{}.mp3",
                link_noun
            ),
            path_buf.as_path(),
        ) {
            println!("Failed to download audio file from: {}", link_noun);
        }

        sleep(std::time::Duration::from_secs(1));
    }
}

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
