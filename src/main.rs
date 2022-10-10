use rand::seq::IteratorRandom; // 0.7.3
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use text_io::read;


const FILENAME: &str = "words.txt";
const MAX_GUESSES: u8 = 5;

/*
* Get a random word from our file
*/
fn find_word() -> String {
    let f = File::open(FILENAME)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", FILENAME, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}


fn main() {
    let solution: String = find_word();
    println!("{}", solution);

    let mut guess_count: u8 = 1;

    while guess_count <= MAX_GUESSES {
        println!("Please input your guess.");
        let guess: String = read!("{}\n");

        println!("Guess {}/{}, you guessed: {}", guess_count, MAX_GUESSES, guess);
        guess_count+= 1;

        if guess.eq(&solution) {
            println!("You win!");
            break;
        }
    }

    if guess_count > MAX_GUESSES {
        println!("You lose!");
    }
}
