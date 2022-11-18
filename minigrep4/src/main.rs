//Splitting Code into a Library Crate

//Our minigrep project is looking good so far! Now we’ll split the src/main.rs file and 
//put some code into the src/lib.rs file so we can test it and have a src/main.rs file with fewer responsibilities.

//Let’s move all the code that isn’t the main function from src/main.rs to src/lib.rs:¸

//The run function definition
//The relevant use statements
//The definition of Config
//The Config::new function definition

use std::env;
use std::process;
use minigrep4::Config;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error while parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep4::run(&config) {
        println!("Error while trying to read the file: {}", e);
        process::exit(1);
    }
    
}
