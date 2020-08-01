pub fn print_hangman(lives_left: i32) {
    match lives_left {
        0i32 => 
        {
            println!(" _______  ");
            println!("|       |  ");
            println!("|       XO  ");
            println!("|      /|\\ ");
            println!("|      / \\ ");
            println!("|           ");
            println!("|           ");
            println!("____________");
        }
        1i32 => 
        {
            println!(" _______  ");
            println!("|       | ");
            println!("|       O ");
            println!("|      /|\\ ");
            println!("|      / \\ ");
            println!("|      |||  ");
            println!("|      |||  ");
            println!("____________");
        }
        2i32 => 
        {
            println!(" _______ ");
            println!("|        ");
            println!("|       O ");
            println!("|      /|\\ ");
            println!("|      / \\ ");
            println!("|      |||  ");
            println!("|      |||  ");
            println!("____________");
        }
        3i32 => 
        {
            println!(" _______  ");
            println!("|         ");
            println!("|         ");
            println!("|       O ");
            println!("|      /|\\ ");
            println!("|      / \\  ");
            println!("|      |||  ");
            println!("____________");
        }
        4i32 => 
        {
            println!(" _______  ");
            println!("|         ");
            println!("|         ");
            println!("|         ");
            println!("|       O    ");
            println!("|      /|\\  ");
            println!("|      / \\  ");
            println!("____________");
        }
        5i32 => 
        {
            println!("          ");
            println!("|         ");
            println!("|         ");
            println!("|         ");
            println!("|       O    ");
            println!("|      /|\\  ");
            println!("|      / \\  ");
            println!("____________");
        }
        _ => 
        {
            println!("          ");
            println!("          ");
            println!("          ");
            println!("            ");
            println!("        O   ");
            println!("       /|\\ ");
            println!("       / \\"); 
            println!("____________");
        }

    }
}