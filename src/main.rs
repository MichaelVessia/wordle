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

fn print_guesses(guesses: &[String; 5]) -> () {
    println!("===");
    println!("Guesses:");
    for val in guesses.iter() {
        if !val.eq("") {
            println!("{}", val);
        }
    }
    println!("===");
}


fn main() {
    let solution: String = find_word();
    println!("{}", solution);

    let mut guess_count: u8 = 1;
    let mut guesses: [String; 5] = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new()
    ];


    while guess_count <= MAX_GUESSES {
        println!("Please input guess {}/{}.", guess_count, MAX_GUESSES);
        let guess: String = read!("{}\n");

        if guess.len() != 5 {
            println!("Guess must be a 5 character word");
            continue;
        }
        let index: usize = (guess_count - 1) as usize;
        guesses[index] = String::from(&guess);
        guess_count+= 1;

        print_guesses(&guesses);

        if guess.eq(&solution) {
            println!("You win!");
            return;
        }
    }
    if guess_count > MAX_GUESSES {
        println!("You lose!");
    }
}
