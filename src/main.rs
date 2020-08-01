use std::env;
use std::process;
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
    println!("{}", arguments.filename);
    println!("{}", arguments.flag);
    print_hangman::print_hangman(5i32);
    //print_hangman::print_hangman(3i32);
    //print_hangman::print_hangman(2i32);
    //print_hangman::print_hangman(1i32);
    //print_hangman::print_hangman(0i32);
    //print_hangman::print_hangman(7i32);

}

