mod cli;
mod feedback;
mod solver;
mod word_list;

use cli::prompt;
use feedback::{parse_pattern, GuessResult};
use solver::filter_candidates;
use word_list::load_words;

fn main() {
    let words = load_words("words.txt").expect("Failed to load words");
    let mut history: Vec<GuessResult> = Vec::new();

    println!("Wordle Solver");
    println!("Enter guesses and feedback patterns.");
    println!("Pattern format: 0 = gray, 1 = yellow, 2 = green");
    println!("Type 'quit' to exit.");
    println!();

    loop {
        let guess = prompt("Enter guess: ").expect("Failed to read guess");

        if guess == "quit" {
            println!("Goodbye.");
            break;
        }

        if guess.len() != 5 || !guess.chars().all(|c| c.is_ascii_alphabetic()) {
            println!("Guess must be exactly 5 alphabetic letters.\n");
            continue;
        }

        if guess == "history" {
            if history.is_empty() {
                println!("No guesses entered yet.\n");
            } else {
                println!("Guess history:");
                for entry in &history {
                    println!("{} -> {}", entry.guess, feedback::pattern_to_string(&entry.pattern));
                }
                println!();
            }
            continue;
        }

        let pattern_input = prompt("Enter pattern: ").expect("Failed to read pattern");

        if pattern_input == "quit" {
            println!("Goodbye.");
            break;
        }

        let pattern = match parse_pattern(&pattern_input) {
            Some(pattern) => pattern,
            None => {
                println!("Pattern must be exactly 5 characters using only 0, 1, or 2.\n");
                continue;
            }
        };

        if pattern.iter().all(|p| matches!(p, feedback::LetterFeedback::Green)) {
            println!("\n🎉 Solved! The answer is: {}\n", guess);
            break;
        }

        history.push(GuessResult {
            guess,
            pattern,
        });

        let candidates = filter_candidates(&words, &history);

        println!("\nRemaining candidates: {}", candidates.len());

        for word in candidates.iter().take(20) {
            println!("{word}");
        }

        if candidates.len() > 20 {
            println!("...and {} more", candidates.len() - 20);
        }

        println!();
    }
}