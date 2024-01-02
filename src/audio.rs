use std::cmp::max;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use rodio::{source::Source, Decoder, OutputStream};

pub(crate) fn play_file(path: &Path) -> Result<(), Box<dyn Error>> {
    /* Based on: https://docs.rs/rodio/latest/rodio/ */

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let file = File::open(path)?;
    let length = file.metadata().unwrap().len();
    let reader = BufReader::new(file);
    let source = Decoder::new(reader)?;
    stream_handle.play_raw(source.convert_samples())?;

    /* The sound plays in a separate audio thread, so we need to keep the main thread alive while it's playing.
    The file size is used as an approximation of the audio length. */
    sleep(Duration::from_millis(max(length as u64 / 10, 1_750)));

    Ok(())
}
