#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LetterFeedback {
    Gray,
    Yellow,
    Green,
}

pub fn score_guess(guess: &str, answer: &str) -> Vec<LetterFeedback> {
    let guess_chars: Vec<char> = guess.chars().collect();
    let answer_chars: Vec<char> = answer.chars().collect();

    let mut result = vec![LetterFeedback::Gray; 5];
    let mut counts = [0u8; 26];

    for i in 0..5 {
        if guess_chars[i] == answer_chars[i] {
            result[i] = LetterFeedback::Green;
        } else {
            let idx = (answer_chars[i] as u8 - b'a') as usize;
            counts[idx] += 1;
        }
    }

    for i in 0..5 {
        if result[i] == LetterFeedback::Green {
            continue;
        }

        let idx = (guess_chars[i] as u8 - b'a') as usize;
        if counts[idx] > 0 {
            result[i] = LetterFeedback::Yellow;
            counts[idx] -= 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scores_all_green_for_exact_match() {
        let result = score_guess("crane", "crane");
        assert_eq!(
            result,
            vec![
                LetterFeedback::Green,
                LetterFeedback::Green,
                LetterFeedback::Green,
                LetterFeedback::Green,
                LetterFeedback::Green,
            ]
        );
    }
    #[test]
    fn scores_mixed_feedback_correctly() {
        let result = score_guess("crane", "trace");
        assert_eq!(
            result,
            vec![
                LetterFeedback::Yellow,
                LetterFeedback::Green,
                LetterFeedback::Green,
                LetterFeedback::Gray,
                LetterFeedback::Green,
            ]
        );
    }

    #[test]
    fn handles_repeated_letters_correctly() {
        let result = score_guess("robot", "boost");
        assert_eq!(
            result,
            vec![
                LetterFeedback::Gray,
                LetterFeedback::Green,
                LetterFeedback::Yellow,
                LetterFeedback::Yellow,
                LetterFeedback::Green,
            ]
        );
    }
}