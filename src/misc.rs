pub struct Arguments {
    pub flag: String,
    pub filename: String,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 3 {
            return Err("Too many arguments");
        }
        let f = args[1].clone();
        //let mut filename = "";
        if f.contains("-h") || f.contains("-help") && args.len() == 2 {
            println!("Usage:
            \n\r -h or -help to show this help menu
            \n\r -g or -game to play the game and specify wordlist file
            \n\r ==========================================
            \n\rExamples:
            \n\r hangman -g simple.txt
            \n\r hangman -g advanced.txt");
            return Err("help");
        } else if f.contains("-h") || f.contains("-help") {
            return Err("Too many arguments, use -h or -help to display usage guide");
        } else if args.len() == 3 {
            if f.contains("-g") || f.contains("-game"){
                if args[2].ends_with(".txt") {
                    let filename = &args[2].clone();
                    return Ok(Arguments {flag: f, filename: filename.to_string()});
                }
                else {
                    return Err("Invalid filename; must end with .txt");
                }
            } else {
                return Err("Invalid syntax, use -h or -help to display usage guide")
            }
            
        } else {
            return Err("Invalid Syntax");
        }
    }
}
