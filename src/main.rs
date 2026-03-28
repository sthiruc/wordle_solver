mod word_list;
mod feedback;

use feedback::{score_guess, LetterFeedback};
use word_list::load_words;

fn main() {
    let words = load_words("words.txt").expect("Failed to load words");
    println!("Loaded {} words", words.len());

    let guess = "crane";
    let answer = "trace";

    let result = score_guess(guess, answer);

    println!("Guess: {guess}");
    println!("Answer: {answer}");
    println!("Feedback: {:?}", result);

    for tile in result {
        match tile {
            LetterFeedback::Gray => print!("⬛"),
            LetterFeedback::Yellow => print!("🟨"),
            LetterFeedback::Green => print!("🟩"),
        }
    }
    println!();
}