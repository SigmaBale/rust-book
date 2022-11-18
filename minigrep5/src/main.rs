use std::env;
use std::process;
use minigrep5::{Config, run};


fn main() {
    
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error while reading arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("Error while trying to open a file: {}", e);
        process::exit(1);
    }
}

//Writing Error Messages to Standard Error Instead of Standard Output

//At the moment, we’re writing all of our output to the terminal using the println! macro. In most terminals, there are two kinds of output: 
//standard output (stdout) for general information and standard error (stderr) for error messages.

//This distinction enables users to choose to direct the successful output of a program to a file but still print error messages to the screen.
//The println! macro is only capable of printing to standard output, so we have to use something else to print to standard error.


//Checking Where Errors Are Written

//First, let’s observe how the content printed by minigrep is currently being written to standard output, including any error messages we want to write to standard error instead. 
//We’ll do that by redirecting the standard output stream to a file while also intentionally causing an error.
//We won’t redirect the standard error stream, so any content sent to standard error will continue to display on the screen.

//Command line programs are expected to send error messages to the standard error stream so we can still see error messages on the screen even 
//if we redirect the standard output stream to a file. 
//Our program is not currently well-behaved: we’re about to see that it saves the error message output to a file instead!

//The way to demonstrate this behavior is by running the program with > and the filename, output.txt, that we want to redirect the standard output stream to. 
//We won’t pass any arguments, which should cause an error:
// cargo run > output.txt

//The > syntax tells the shell to write the contents of standard output to output.txt instead of the screen. 
//We didn’t see the error message we were expecting printed to the screen, so that means it must have ended up in the file. This is what output.txt contains:
// Problem parsing arguments: not enough arguments
//Yup, our error message is being printed to standard output. 
//It’s much more useful for error messages like this to be printed to standard error so only data from a successful run ends up in the file. We’ll change that.


//Printing Errors to Standard Error

//Because of the refactoring we did earlier in this chapter, all the code that prints error messages is in one function, main.
//The standard library provides the eprintln! macro that prints to the standard error stream, 
//so let’s change the two places we were calling println! to print errors to use eprintln! instead.

//This demonstrates that we’re now using standard output for successful output and standard error for error output as appropriate.