use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

pub mod validation;
pub mod error;

pub use validation::{ValidationError, LoginValidator};
pub use error::{NokError, NokResult, ErrorSeverity};

// Save data to a JSON file
pub fn save_to_file<T: Serialize>(data: &T, path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

// Load data from a JSON file
pub fn load_from_file<T: for<'de> Deserialize<'de>>(path: &Path) -> io::Result<T> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data = serde_json::from_str(&contents)?;
    Ok(data)
}

// Ensure a directory exists
pub fn ensure_dir(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}
