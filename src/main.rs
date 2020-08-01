use std::env;
use std::process;
use std::fs;
use std::iter;

use rand::Rng;

mod print_hangman;
mod misc;

struct Game {
    lives: i32,
    letters_guessed: String,
    word_to_guess: String,
    masked_word: String,
}


fn main() {
    let mut rng = rand::thread_rng();
    let args: Vec<String> = env::args().collect();
    
    let arguments = misc::Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                // if error is help close program
                process::exit(0);
            } else {
                // if error is not help print error then close program
                eprintln!("{} problem parsing arguments: {}", args[0], err);
                process::exit(0);
            }
        }
    );
    
    // Read in a wordlist file
    let wordlist_raw = fs::read_to_string(arguments.filename)
        .expect("Something went wrong reading the file");
    let split = wordlist_raw.split("\n");
    let wordlist_vec: Vec<&str> = split.collect();

    // Select a random word from the wordlist
    let index_val = rng.gen_range(0, wordlist_vec.len());
    let selected_word = wordlist_vec[index_val];
    println!("The {} word in the list is {}", index_val, selected_word);

    // Mask the string as a String of "-"
    let mut masked_word = iter::repeat("-").take(selected_word.len()).collect::<String>();
    println!("{}", masked_word);

    let mut current_game = Game {
        lives: 6i32,
        letters_guessed: String::from(""),
        word_to_guess: String::from(selected_word),
        masked_word: masked_word,
    };

    println!("lives: {}
            \n letters guessed: {}
            \n word to guess: {}
            \n masked_word: {}", current_game.lives, current_game.letters_guessed,
        current_game.word_to_guess, current_game.masked_word);

    print_hangman::print_hangman(current_game.lives);

}

