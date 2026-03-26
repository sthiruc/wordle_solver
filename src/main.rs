mod word_list;

use word_list::load_words;

fn main() {
    let words = load_words("words.txt").expect("Failed to load words");

    println!("Loaded {} words", words.len());

    for word in words.iter().take(10) {
        println!("{word}");
    }
}
