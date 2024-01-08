use valid::constraint::CharCount;
use valid::Validate;

#[derive(Debug, serde::Deserialize, serde::Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Phrase {
    english: String,
    german: String,
}

#[derive(Debug)]
pub(crate) struct Phrases {
    data: Vec<Phrase>,
}

impl Phrase {
    pub(crate) fn new(german: String, english: String) -> Self {
        Phrase {
            english: Self::check_phrase_argument("English phrase", english),
            german: Self::check_phrase_argument("German phrase", german),
        }
    }

    fn check_phrase_argument(field_name: &'static str, phrase: String) -> String {
        phrase
            .validate(field_name, &CharCount::MinMax(12, 64))
            .result()
            .unwrap()
            .unwrap()
    }
}

impl Phrases {
    pub(crate) fn read() -> Self {
        let data = csv::Reader::from_path("phrases.csv")
            .expect("Failed to read phrases")
            .deserialize()
            .map(|r| r.unwrap())
            .collect();
        Phrases { data }
    }

    pub(crate) fn append(&mut self, phrases: &mut Vec<Phrase>) {
        self.data.append(phrases);
        self.data.dedup_by(|a, b| a.german == b.german);
        self.data.sort_by(|a, b| a.german.cmp(&b.german));
    }

    pub(crate) fn write(&self) {
        let mut writer = csv::Writer::from_path("phrases.csv").expect("Failed to write phrases");
        for phrase in self.data.iter() {
            writer
                .serialize(phrase)
                .expect("Failed to serialize phrase");
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
}

impl Drop for Phrases {
    fn drop(&mut self) {
        self.write();
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::types::phrases::Phrases;

    #[test]
    fn read_all() {
        let phrases = Phrases::read();
        assert_eq!(phrases.len(), count_entries_in_csv_file());
    }

    fn count_entries_in_csv_file() -> usize {
        let file = File::open("phrases.csv").expect("Failed to open file");
        BufReader::new(file).lines().count() - 1
    }
}
