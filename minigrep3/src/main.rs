use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    //We use if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does. 
    //The run function doesn’t return a value that we want to unwrap in the same way that Config::new returns the Config instance. 
    //Because run returns () in the success case, we only care about detecting an error, 
    //so we don’t need unwrap_or_else to return the unwrapped value because it would only be ().
    if let Err(e) = run(config) {
        println!("Error while trying to read the file: {}", e);
        process::exit(1);
    }
    //The bodies of the if let and the unwrap_or_else functions are the same in both cases: we print the error and exit.
}

//The run function now contains all the remaining logic from main, starting from reading the file. 
//The run function takes the Config instance as an argument.
fn run(config: Config) -> Result<(), Box<dyn Error>> { //For the error type, we used the trait object Box<dyn Error> 
    //(and we’ve brought std::error::Error into scope with a use statement at the top).
    //For now, just know that Box<dyn Error> means the function will return a type that implements the Error trait, 
    //but we don’t have to specify what particular type the return value will be.
    //This gives us flexibility to return error values that may be of different types in different error cases. The dyn keyword is short for “dynamic.”
    let content = fs::read_to_string(config.file_name)?; //Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.
    println!("With text:\n{}", content);
    Ok(())//This Ok(()) syntax might look a bit strange at first, but using () like this is the idiomatic way 
    //to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.
}
//We’ve made three significant changes here. First, we changed the return type of the run function to Result<(), Box<dyn Error>>. 
//This function previously returned the unit type, (), and we keep that as the value returned in the Ok case.

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Not enough arguments provided!")
        }else {
            let query = args[1].clone();
            let file_name = args[2].clone();
            Ok(Config { query, file_name })
        }
    }
}

//Extracting Logic from main

//Now that we’ve finished refactoring the configuration parsing, let’s turn to the program’s logic. 
//As we stated in “Separation of Concerns for Binary Projects”, we’ll extract a function named run 
//that will hold all the logic currently in the main function that isn’t involved with setting up configuration or handling errors.

//When we’re done, main will be concise and easy to verify by inspection, and we’ll be able to write tests for all the other logic.
//For now, we’re just making the small, incremental improvement of extracting the function. We’re still defining the function in src/main.rs. check up



//Returning Errors from the run Function

//With the remaining program logic separated into the run function, we can improve the error handling, as we did with Config::new
//Instead of allowing the program to panic by calling expect, the run function will return a Result<T, E> when something goes wrong.
//Rust tells us that our code ignored the Result value and the Result value might indicate that an error occurred. 
//But we’re not checking to see whether or not there was an error, and the compiler reminds us 
//that we probably meant to have some error-handling code here! Let’s rectify that problem now.

//Handling Errors Returned from run in main
//We’ll check for errors and handle them using a technique similar to one we used with Config::new but with a slight difference