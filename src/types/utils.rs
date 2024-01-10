use std::io::{stdin, stdout, Write};
use std::path::Path;

use rand::Rng;

use crate::types::audio::play_file_or_print_error;

pub(crate) fn remove_random<T>(vec: &mut Vec<T>) -> T {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..vec.len());
    vec.remove(index)
}

pub(crate) fn read_line(prompt: &str) -> String {
    print!("{}: ", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to user input");
    input.trim().to_string()
}

pub(crate) fn play_and_read_line(prompt: &str, file_path: &Path) -> String {
    print!("{}: ", prompt);
    stdout().flush().unwrap();

    play_file_or_print_error(file_path);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to user input");
    input.trim().to_string()
}
