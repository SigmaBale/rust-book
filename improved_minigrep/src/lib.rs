//Improving Our I/O Project

//With this new knowledge about iterators, we can improve the I/O project in Chapter 12 by using iterators to make places in the code clearer and more concise.
//Let’s look at how iterators can improve our implementation of the Config::new function and the search function.

//Removing a clone Using an Iterator
//We added code that took a slice of String values and created an instance of the Config struct by indexing into the slice and cloning the values, 
//allowing the Config struct to own those values. 
//At the time, we said not to worry about the inefficient clone calls because we would remove them in the future. Well, that time is now!

//We needed clone here because we have a slice with String elements in the parameter args, but the new function doesn’t own args. 
//To return ownership of a Config instance, we had to clone the values from the query and filename fields of Config so the Config instance can own its values.

//With our new knowledge about iterators, we can change the new function to take ownership of an iterator as its argument instead of borrowing a slice.
//We’ll use the iterator functionality instead of the code that checks the length of the slice and indexes into specific locations. 
//This will clarify what the Config::new function is doing because the iterator will access the values.

//Once Config::new takes ownership of the iterator and stops using indexing operations that borrow, 
//we can move the String values from the iterator into Config rather than calling clone and making a new allocation.


use std::fs;
use std::error::Error;
use std::env;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    //This still won’t compile because we need to update the function body.
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    //The standard library documentation for the env::args function shows that the type of the iterator it returns is std::env::Args. 
    //We’ve updated the signature of the Config::new function so the parameter args has the type std::env::Args instead of &[String].
    //Because we’re taking ownership of args and we’ll be mutating args by iterating over it, 
    //we can add the mut keyword into the specification of the args parameter to make it mutable.

        //Using Iterator Trait Methods Instead of Indexing
        //The standard library documentation also mentions that std::env::Args implements the Iterator trait, so we know we can call the next method on it! 
        args.next();
        //Remember that the first value in the return value of env::args is the name of the program. 
        //We want to ignore that and get to the next value, so first we call next and do nothing with the return value.

        //Second, we call next to get the value we want to put in the query field of Config. 
        //If next returns a Some, we use a match to extract the value. 
        //If it returns None, it means not enough arguments were given and we return early with an Err value.
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string!"),  
        };
        //We do the same thing for the filename value.
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename!"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        contains_query_sensitive(&config.query, &contents)
    }else {
        contains_query_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

//Making Code Clearer with Iterator Adaptors
//We can also take advantage of iterators in the contains_query_sensitive function in our I/O project

//We can write this code in a more concise way using iterator adaptor methods. Doing so also lets us avoid having a mutable intermediate results vector. 
//The functional programming style prefers to minimize the amount of mutable state to make code clearer. 
//Removing the mutable state might enable a future enhancement to make searching happen in parallel, because we wouldn’t have to manage concurrent access to the results vector.
fn contains_query_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        //This code uses the filter adaptor to keep only the lines that line.contains(query) returns true for.
        .filter(|line| line.contains(query))
        //We then collect the matching lines into another vector with collect.
        .collect()
}

fn contains_query_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}


