use rand::Rng;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Noun {
    english: String,
    article: String,
    singular: String,
    plural: Option<String>,
}

pub(crate) struct NounQuestion {
    pub(crate) english: String,
    pub(crate) noun: String,
    pub(crate) article: Vec<String>,
    pub(crate) file_path: PathBuf,
    pub(crate) with_article_file_path: PathBuf,
    pub(crate) file_link: String,
    pub(crate) with_article_file_link: String,
}

impl Noun {
    pub(crate) fn read_nouns() -> Result<Vec<Noun>, Box<dyn Error>> {
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

    pub(crate) fn random_question(&self) -> NounQuestion {
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
