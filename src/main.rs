use std::env;
use std::process;
use std::fs;
use std::iter;
use std::io;

use rand::Rng;

mod print_hangman;
mod misc;

struct Game {
    lives: i32,
    letters_guessed: String,
    word_to_guess: String,
    masked_word: String,
    remaining_letters: String,
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

    // Mask the string as a String of "-"
    let masked_word = iter::repeat("-").take(selected_word.len()).collect::<String>();

    let mut current_game = Game {
        lives: 6i32,
        letters_guessed: String::from(""),
        word_to_guess: String::from(selected_word),
        masked_word: masked_word,
        remaining_letters: String::from("a b c d e f g h i j k l m n o p q r s t u v w x y z"),
    };
    
    // Main game loop
    while current_game.lives > 0 && current_game.masked_word != current_game.word_to_guess {
        
        println!("You have {} lives remaining", current_game.lives);
        print_hangman::print_hangman(current_game.lives);
        println!("{}", current_game.masked_word);
        println!("Remaining letters to choose from: {}", current_game.remaining_letters);
        println!("Please guess a letter:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.replace("\n", "");
        // Clear the terminal
        print!("{}[2J", 27 as char);

        // Process guess
        if guess.len() > 1 || guess.len() == 0 {
            println!("Invalid guess, enter single character");
        } else if current_game.letters_guessed.contains(&guess) {
            println!("Invalid guess, you've already tried that letter");
        } else if current_game.word_to_guess.contains(&guess){
            println!("Your guess, {}, is in the word", guess);
            current_game.masked_word = String::new();
            current_game.letters_guessed.push_str(&guess);
            
            for c in current_game.word_to_guess.chars() {
                if current_game.letters_guessed.contains(c){
                    current_game.masked_word.push(c);
                } else {
                    current_game.masked_word.push('-');
                }
            }
        } else {
            println!("Your guess, {}, is not in the word", guess);
            current_game.letters_guessed.push_str(&guess);
            current_game.lives -= 1;
        }
        
    }

    print_hangman::print_hangman(current_game.lives);
    if current_game.lives > 0 {
        println!("Congratulations, you guess the word: {}", current_game.word_to_guess);
    
    } else {
        println!("Better luck next time, the word was: {}", current_game.word_to_guess);
    }
    println!("Would you like to play again? (y/n)");
    let mut play_again = String::new();
    io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");
    
    if play_again.contains('y'){
        main();
    } else {
        println!("See you again soon!");
        process::exit(0);
    }

}

