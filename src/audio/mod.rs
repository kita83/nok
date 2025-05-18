use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink, Source};

// Simple ASCII representation of a knock sound
const KNOCK_SOUND_DATA: &[u8] = include_bytes!("knock.mp3");

pub fn play_knock_sound() -> Result<(), Box<dyn std::error::Error>> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = Cursor::new(KNOCK_SOUND_DATA);
    
    // Decode that sound file into a source
    let source = Decoder::new(file)?;
    
    // Play the sound directly on the device
    sink.append(source);
    
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing
    sink.sleep_until_end();
    
    Ok(())
}
