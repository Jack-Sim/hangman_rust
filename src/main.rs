use std::env;
use std::process;
use std::fs;

mod print_hangman;
mod misc;

fn main() {
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
    
    let wordlist_raw = fs::read_to_string(arguments.filename)
        .expect("Something went wrong reading the file");
    let split = wordlist_raw.split("\n");
    let wordlist_vec: Vec<&str> = split.collect();
    println!("{}", wordlist_vec.len());


    print_hangman::print_hangman(5i32);
    //print_hangman::print_hangman(3i32);
    //print_hangman::print_hangman(2i32);
    //print_hangman::print_hangman(1i32);
    //print_hangman::print_hangman(0i32);
    //print_hangman::print_hangman(7i32);

}

