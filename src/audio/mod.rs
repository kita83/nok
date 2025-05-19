use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink};

// Use include_bytes! to embed the sound file into the binary
const KNOCK_SOUND_DATA: &[u8] = include_bytes!("knock.mp3");

pub fn play_knock_sound() -> Result<(), String> {
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| format!("Failed to get output stream: {}", e))?;
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| format!("Failed to create sink: {}", e))?;

    // Use the embedded sound data
    let file_cursor = Cursor::new(KNOCK_SOUND_DATA);
    let source = Decoder::new(file_cursor)
        .map_err(|e| format!("Failed to decode embedded sound data: {}", e))?;

    sink.append(source);
    sink.sleep_until_end(); // Play the sound synchronously
    Ok(())
}
