use std::fs;
use std::io;

pub fn load_words(path: &str) -> Result<Vec<String>, io::Error> {
    let content = fs::read_to_string(path)?;

    let words = content
        .lines()
        .map(str::trim)
        .filter(|line| line.len() == 5)
        .map(|line| line.to_lowercase())
        .collect();

    Ok(words)
}