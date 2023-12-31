use clap::{Parser, ValueEnum};

/// Simple program to help me learn the German language
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// The mode to run the program in
    #[clap(value_enum, default_value_t = Mode::Articles)]
    pub(crate) mode: Mode,
}

impl Args {
    pub(crate) fn from_args() -> Self {
        Self::parse()
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub(crate) enum Mode {
    #[clap(name = "articles")]
    Articles,
    #[clap(name = "plural")]
    Plural,
    #[clap(name = "verbs")]
    Verbs,
    #[clap(name = "alphabet")]
    Alphabet,
    #[clap(name = "numbers")]
    Numbers,
    #[clap(name = "play")]
    Play,
    #[clap(name = "download")]
    Download,
}
