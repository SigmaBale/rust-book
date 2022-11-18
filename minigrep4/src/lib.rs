//We’ve made liberal use of the pub keyword: on Config, on its fields and its new method, and on the run function. 
//We now have a library crate that has a public API that we can test!

use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Too little arguments provided!")
        }else {
            let query = args[1].clone();
            let file_name = args[2].clone();
            Ok(Config {query, file_name})
        }
    }
}

//Using the search Function in the run Function (search function is below test config - down)

//Now that the search function is working and tested, we need to call search from our run function. 
//We need to pass the config.query value and the contents that run reads from the file to the search function. Then run will print each line returned from search:

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(&config.file_name)?;
    for line in search(config.query.as_str(), &content) {
        println!("Line: {}", line);
    }
    Ok(())
}

//Developing the Library’s Functionality with Test-Driven Development

//Now that we’ve extracted the logic into src/lib.rs and left the argument collecting and error handling in src/main.rs, 
//it’s much easier to write tests for the core functionality of our code.
//We can call functions directly with various arguments and check return values without having to call our binary from the command line.

//In this section, we’ll add the searching logic to the minigrep program by using the Test-driven development (TDD) process.
//This software development technique follows these steps:
//1. Write a test that fails and run it to make sure it fails for the reason you expect.
//2. Write or modify just enough code to make the new test pass.
//3. Refactor the code you just added or changed and make sure the tests continue to pass.
//4. Repeat from step 1!

//This process is just one of many ways to write software, but TDD can help drive code design as well. 
//Writing the test before you write the code that makes the test pass helps to maintain high test coverage throughout the process.

//We’ll test drive the implementation of the functionality that will actually do the searching for the query string in the file contents and produce 
//a list of lines that match the query. We’ll add this functionality in a function called search.


//Writing a Failing Test

//In src/lib.rs, we’ll add a tests module with a test function
//The test function specifies the behavior we want the search function to have: it will take a query and the text to search for the query in, 
//and it will return only the lines from the text that contain the query.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
//This test searches for the string "duct". The text we’re searching is three lines, only one of which contains "duct" (Note that the backslash after the 
//opening double quote tells Rust not to put a newline character at the beginning of the contents of this string literal).

//We assert that the value returned from the search function contains only the line we expect.
//Now we’ll add just enough code to get the test to compile and run by adding a definition of the search function that always returns an empty vector.
//Then the test should compile and fail because an empty vector doesn’t match a vector containing the line "safe, fast, productive."

pub fn search_test<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
//Notice that we need an explicit lifetime 'a defined in the signature of search and used with the contents argument and the return value.
//Lifetime parameters specify which argument lifetime is connected to the lifetime of the return value.
//In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).
//This is important! The data referenced by a slice needs to be valid for the reference to be valid; 
//if the compiler assumes we’re making string slices of query rather than contents, it will do its safety checking incorrectly.

//Rust can’t possibly know which of the two arguments we need, so we need to tell it. Because contents is the argument that contains all of our text 
//and we want to return the parts of that text that match, we know contents is the argument that should be connected to the return value using the lifetime syntax.
//Test fails as we expected it to fail!




//Writing Code to Pass the Test

//Currently, our test is failing because we always return an empty vector. To fix that and implement search, our program needs to follow these steps:

//Iterate through each line of the contents.
//Check whether the line contains our query string.
//If it does, add it to the list of values we’re returning.
//If it doesn’t, do nothing.
//Return the list of results that match.

//Iterating Through Lines with the lines Method
//Rust has a helpful method to handle line-by-line iteration of strings, conveniently named lines.
//The lines method returns an iterator.

//Storing Matching Lines
//We also need a way to store the lines that contain our query string. 
//For that, we can make a mutable vector before the for loop and call the push method to store a line in the vector. After the for loop, we return the vector.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
//Now the search function should return only the lines that contain query, and our test should pass. 
//Our test passed, so we know it works!
//At this point, we could consider opportunities for refactoring the implementation of the search function while keeping the tests passing to maintain the same functionality.
//The code in the search function isn’t too bad, but it doesn’t take advantage of some useful features of iterators. 
//We’ll return to this example in Chapter 13, where we’ll explore iterators in detail, and look at how to improve it.


//Excellent! We’ve built our own mini version of a classic tool and learned a lot about how to structure applications. 
//We’ve also learned a bit about file input and output, lifetimes, testing, and command line parsing.


