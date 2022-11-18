//Using the Returned Iterator Directly

//We’ll change the start of the main function!

use std::env;
use improved_minigrep::{Config, run};
use std::process;

fn main() {

    //The env::args function returns an iterator! 
    //Rather than collecting the iterator values into a vector and then passing a slice to Config::new, 
    //now we’re passing ownership of the iterator returned from env::args to Config::new directly.
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Error while reading arguments: {}", e);
        process::exit(1);
    });
    //Next, we need to update the definition of Config::new. In your I/O project’s src/lib.rs file, let’s change the signature of Config::new

    if let Err(e) = run(&config) {
        eprintln!("Error while trying to read the file: {}", e);
        process::exit(1);
    }
}
