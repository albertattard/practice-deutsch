use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::{fs, io};

pub(crate) fn download_file(link: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    //println!("Downloading audio from {} to {}", link, path.display());

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

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::thread::sleep;
    use std::time::Duration;

    use crate::types::download::download_file;
    use crate::types::nouns::{list_nouns_plurals, Noun};

    #[test]
    fn download_missing_nouns_from_verbformen() {
        let skip = vec!["Internetadresse"];

        for noun in Noun::read() {
            if skip.contains(&noun.singular.as_str()) {
                println!("Skipping {}", &noun);
                continue;
            }

            download_missing_noun_from_verbformen(&noun.singular_file_path(), &noun.singular);
            download_missing_noun_from_verbformen(
                &noun.singular_with_article_file_path(),
                &format!("der_{}", &noun.singular),
            );
        }
    }

    fn download_missing_noun_from_verbformen(file: &Path, noun: &str) {
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

    #[test]
    fn download_missing_nouns_from_collins_dictionary() {
        let skip = vec![
            "Adresse",
            "Anmeldung",
            "Anwalt",
            "Anwältin",
            "Aprikose",
            "Architekt",
            "Architektin",
            "Ärztin",
            "Assistent",
            "Assistentin",
            "Aufzug",
            "Ausgang",
            "Auto",
            "Automat",
            "Avocado",
            "Baby",
            "Bäckerin",
            "Banane",
            "Bankkauffrau",
            "Bankkaufmann",
            "Beere",
            "Beispiel",
            "Beraterin",
            "Beruf",
            "Bettbezug",
            "Bettdecke",
            "Birne",
            "Blaubeere",
            "Bleistift",
            "Block",
            "Blume",
            "Brief",
            "Brombeere",
            "Büro",
            "Cafeteria",
            "Chef",
            "Dame",
            "Designerin",
            "Dokument",
            "E-Mail",
            "Eingang",
            "Elektrikerin",
            "Erdbeere",
            "Erstaufnahme",
            "Familienname",
            "Feige",
            "Fotograf",
            "Fotografin",
            "Frau",
            "Garderobe",
            "Guave",
            "Handy",
            "Hausaufgabe",
            "Hausfrau",
            "Hausmann",
            "Hausnummer",
            "Heft",
            "Heim",
            "Heizung",
            "Hemd",
            "Herr",
            "Homepage",
            "Horst",
            "Hose",
            "Hund",
            "Hündin",
            "Informatikerin",
            "Ingenieur",
            "Ingenieurin",
            "Internetadresse",
            "Jahr",
            "Kaffee",
            "Karte",
            "Kassiererin",
            "Kellnerin",
            "Kind",
            "Kirsche",
            "Kiwi",
            "Köchin",
            "Kokosnuss",
            "Kölnerin",
            "Korrespondenz",
            "Krawatte",
            "Kühlschrank",
            "Kunde",
            "Künderin",
            "Kurs",
            "Lampe",
            "Laptop",
            "Lehrerin",
            "Lektion",
            "Lineal",
            "Mango",
            "Maracuja",
            "Maschine",
            "Melone",
            "Mieterin",
            "Nachbar",
            "Nachbarin",
            "Nachname",
            "Nummer",
            "Orange",
            "Paar",
            "Papaya",
            "Pfirsich",
            "Pflaume",
            "Polizist",
            "Polizistin",
            "Portemonnaie",
            "Postleitzahl",
            "Regal",
            "Rezeption",
            "Rufnummer",
            "Sache",
            "Schlange",
            "Schlosserin",
            "Schreibtisch",
            "Schuh",
            "Schülerin",
            "Schwester",
            "Seite",
            "Sekretariat",
            "Shirt",
            "Sofa",
            "Sprache",
            "Steckdose",
            "Stift",
            "Straße",
            "Student",
            "Studentin",
            "Tag",
            "Tasche",
            "Tasse",
            "Teppich",
            "Tier",
            "Tierheim",
            "Tisch",
            "Toilette",
            "Traube",
            "Tür",
            "Übung",
            "Universität",
            "Unterkunft",
            "Vase",
            "Verkäuferin",
            "Vermieterin",
            "Visitenkarte",
            "Vorhang",
            "Vorname",
            "Ware",
            "Wassermelone",
            "Weg",
            "Wein",
            "Weintraube",
            "Welt",
            "Wirt",
            "Wirtin",
            "Woche",
            "Zentrale",
            "Zitrone",
        ];

        for noun in Noun::read() {
            if skip.contains(&noun.singular.as_str()) {
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
}
