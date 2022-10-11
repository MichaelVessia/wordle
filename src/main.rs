use std::collections::HashSet;

use bracket_random::prelude::RandomNumberGenerator;
use colored::*;

const WORD_LENGTH: usize = 5;
const ALL_WORDS: &str = include_str!("../words.txt");

struct GameState {
    all_words: Vec<String>,
    solution: String,
    guessed_letters: HashSet<char>,
    guesses: Vec<String>
}

impl GameState {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let all_words = get_words(ALL_WORDS);
        let solution = rng.random_slice_entry(&all_words).unwrap().clone();
        Self {
            all_words,
            solution,
            guessed_letters: HashSet::new(),
            guesses: Vec::new(),
        }
    }
    fn display_guesses(&mut self) {
        self.guesses.iter().enumerate().for_each(|(guess_number, guess)| {
            print!("{}: ", guess_number+1);
            guess.chars().enumerate().for_each(|(pos, c)| {
                let display = if self.solution.chars().nth(pos).unwrap() == c {
                    format!("{c}").bright_green()
                } else if self.solution.chars().any(|wc| wc == c) {
                    format!("{c}").bright_yellow()
                } else {
                    self.guessed_letters.insert(c);
                    format!("{c}").red()
                };
                print!("{display}");
            });
            println!();
        })
    }
}

fn main() {
    let mut game = GameState::new();
    game.guesses = Vec::from(["ABELS".to_string(), "HELLO".to_string()]);
    game.display_guesses();
}


pub fn sanitize_word(word: &str) -> String {
    word.trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}

pub fn is_valid_word(word: &str) -> bool {
    return word.len() == WORD_LENGTH;
}

pub fn get_words(words: &str) -> Vec<String> {
    words
        .split('\n')
        .map(sanitize_word)
        .filter(|l| is_valid_word(l))
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sanitze_word() {
        assert_eq!(sanitize_word("HELLO"), "HELLO");
        assert_eq!(sanitize_word("  HELLO"), "HELLO");
        assert_eq!(sanitize_word("HELLO\n"), "HELLO");
        assert_eq!(sanitize_word("HELLO  "), "HELLO");
        assert_eq!(sanitize_word("HEL  LO"), "HELLO");
        assert_eq!(sanitize_word("H3L\nL0"), "HLL");
    }

    #[test]
    fn test_is_valid_word() {
        assert_eq!(is_valid_word("HELLO"), true);
        assert_eq!(is_valid_word("WORLD"), true);
        assert_eq!(is_valid_word("WRLD"), false);
        assert_eq!(is_valid_word("WOORLD"), false);
    }
    #[test]
    fn test_get_words() {
        assert_eq!(get_words("HELLO\nWORLD"), ["HELLO", "WORLD"]);
        assert_eq!(get_words("HELLO\n\nWORLD"), ["HELLO", "WORLD"]);
        assert_eq!(get_words("HEY\nWORLD"), ["WORLD"]);
        let empty_vec: Vec<String> = Vec::new();
        assert_eq!(get_words("HEY\nYOU\nPIKACHU"), empty_vec);
    }
}
