use crate::types::alphabet::alphabet;
use crate::types::cla::{Args, Mode};
use crate::types::nouns::{articles, plural};
use crate::types::numbers::numbers;
use crate::types::verbs::verbs;

mod types;

fn main() {
    let args = Args::from_args();

    match args.mode {
        Mode::Articles => articles(),
        Mode::Plural => plural(),
        Mode::Verbs => verbs(),
        Mode::Numbers => numbers(),
        Mode::Alphabet => alphabet(),
    }
}
