mod word_list;
mod feedback;
mod solver;

use feedback::{parse_pattern, GuessResult};
use solver::filter_candidates;
use word_list::load_words;

fn main() {
    let words = load_words("words.txt").expect("Failed to load words");

    let history = vec![
        GuessResult {
            guess: "crane".to_string(),
            pattern: parse_pattern("00100").expect("Invalid pattern"),
        },
    ];

    let candidates = filter_candidates(&words, &history);

    println!("Remaining candidates: {}", candidates.len());

    for word in candidates.iter().take(20) {
        println!("{word}");
    }
}