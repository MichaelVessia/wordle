use std::collections::HashSet;

use bracket_random::prelude::RandomNumberGenerator;
use colored::*;

const WORD_LENGTH: usize = 5;
const MAX_TRIES: usize = 5;
const ALL_WORDS: &str = include_str!("../words.txt");

struct GameState {
    all_words: Vec<String>,
    solution: String,
    guessed_letters: HashSet<char>,
    guesses: Vec<String>,
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
        self.guesses
            .iter()
            .enumerate()
            .for_each(|(guess_number, guess)| {
                print!("{}: ", guess_number + 1);
                let mut masked_solution = &self.solution;
                guess.chars().enumerate().for_each(|(pos, c)| {
                    let display: ColoredString = match get_guess_color(&masked_solution, pos, c) {
                        "green" => format!("{c}").bright_green(),
                        "yellow" => format!("{c}").bright_yellow(),
                        "red" => {
                            self.guessed_letters.insert(c);
                            format!("{c}").red()
                        },
                        _ => format!("{c}").red()
                    };
                    print!("{display}");
                    masked_solution = &masked_solution.replacen(&c.to_string(), "_", 1);
                });
                println!();
            })
    }

    fn display_invalid_letters(&self) {
        if !self.guessed_letters.is_empty() {
            print!("Letters not in the word: ");
            self.guessed_letters
                .iter()
                .for_each(|letter| print!("{letter} "));
            println!();
        }
    }

    fn ask_for_guess(&mut self) -> String {
        println!(
            "{}",
            format!(
                "Enter your word guess ({} letters) and press ENTER",
                WORD_LENGTH
                )
            .cyan()
            );
        self.display_invalid_letters();
        let mut guess = String::new();
        let mut valid_guess = false;
        while !valid_guess {
            guess = String::new();
            std::io::stdin().read_line(&mut guess).unwrap();
            guess = sanitize_word(&guess);
            if guess.len() != WORD_LENGTH {
                println!(
                    "{}",
                    format!("Your guess must be {} letters.", WORD_LENGTH).red()
                    )
            } else if !self.all_words.iter().any(|word| word == &guess) {
                println!("{}", format!("{} isn't in the list", guess).red());
            } else {
                self.guesses.push(guess.clone());
                valid_guess = true;
            }
        }
        guess
    }

    fn is_game_over(&self, guess: &str) -> bool {
        let n_tries = self.guesses.len();
        if guess == self.solution {
            println!("Correct! You guessed the word in {} tries.", n_tries);
            true
        } else if n_tries >= MAX_TRIES {
            println!(
                "{}",
                format!("You ran out of tries! The word was {}", self.solution).bright_red()
                );
            true
        } else {
            false
        }
    }
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

pub fn get_guess_color(solution: &String, pos: usize, c: char) -> &str {
    if solution.chars().nth(pos).unwrap() == c {
        return "green";
    } else if solution.chars().find(|&wc| {
        c == wc
    }).is_none() {
        return "red";
    } else {
        let mut has_misplaced_char = false; 
        solution.chars().enumerate().for_each(|(idx, wc)| {
            if wc == c && idx != pos {
                has_misplaced_char = true;
            }
        });
        if has_misplaced_char {
            return "yellow";
        }
    }
    "red"
}

fn main() {
    let mut game = GameState::new();
    loop {
        game.display_guesses();
        let guess = game.ask_for_guess();
        if game.is_game_over(&guess) {
            break;
        }
    }
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
    #[test]
    fn test_get_guess_color() {
        assert_eq!(get_guess_color(&"MONEY".to_string(), 0, 'M'), "green");
        assert_eq!(get_guess_color(&"MONEY".to_string(), 0, 'Y'), "yellow");
        assert_eq!(get_guess_color(&"MONEY".to_string(), 0, 'D'), "red");
    }
}
