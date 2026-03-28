use crate::feedback::{score_guess, GuessResult};

pub fn filter_candidates(words: &[String], history: &[GuessResult]) -> Vec<String> {
    words.iter()
        .filter(|candidate| {
            history.iter().all(|past_guess| {
                score_guess(&past_guess.guess, candidate) == past_guess.pattern
            })
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feedback::LetterFeedback;

    #[test]
    fn filters_words_that_match_single_guess_history() {
        let words = vec![
            "trace".to_string(),
            "grace".to_string(),
            "slate".to_string(),
            "flame".to_string(),
        ];

        let history = vec![
            GuessResult {
                guess: "crane".to_string(),
                pattern: vec![
                    LetterFeedback::Yellow,
                    LetterFeedback::Green,
                    LetterFeedback::Green,
                    LetterFeedback::Gray,
                    LetterFeedback::Green,
                ],
            },
        ];

        let candidates = filter_candidates(&words, &history);

        assert_eq!(candidates, vec!["trace".to_string(), "grace".to_string()]);
    }
}