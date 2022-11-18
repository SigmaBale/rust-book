use std::fs;
use std::error::Error;
use std::process;
use std::env;

//Working with Environment Variables

//We’ll improve minigrep by adding an extra feature: an option for case-insensitive searching that the user can turn on via an environment variable.
//We could make this feature a command line option and require that users enter it each time they want it to apply, but instead we’ll use an environment variable.
//Doing so allows our users to set the environment variable once and have all their searches be case insensitive in that terminal session.


//Writing a Failing Test for the Case-Insensitive search Function

//We want to add a new search_case_insensitive function that we’ll call when the environment variable is on.
//We’ll continue to follow the TDD process, so the first step is again to write a failing test.
//We’ll add a new test for the new search_case_insensitive function and rename our old test from one_result to case_sensitive to clarify the differences between the two tests.
//Look down at the end of lib.rs!

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided!");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        //Here, we create a new variable case_sensitive. 
        //To set its value, we call the env::var function and pass it the name of the CASE_INSENSITIVE environment variable. 
        //The env::var function returns a Result that will be the successful Ok variant that contains the value of the environment variable if the environment variable is set. 
        //It will return the Err variant if the environment variable is not set.
        //We’re using the is_err method on the Result to check whether it’s an error and therefore unset, which means it should do a case-sensitive search.
        //If the CASE_INSENSITIVE environment variable is set to anything, is_err will return false and the program will perform a case-insensitive search.
        //We don’t care about the value of the environment variable, just whether it’s set or unset, 
        //so we’re checking is_err rather than using unwrap, expect, or any of the other methods we’ve seen on Result.
        //We pass the value in the case_sensitive variable to the Config instance so the run function can read that value and decide whether to call search or search_case_insensitive
        Ok(Config {query, file_name, case_sensitive})
    }
    //First, we’ll run our program without the environment variable set and with the query 'to', 
    //which should match any line that contains the word “to” in all lowercase.
    //Now, let’s run the program with CASE_INSENSITIVE set to 1 but with the same query to.
    //If you’re using PowerShell, you will need to set the environment variable and run the program as separate commands:
    // $Env:CASE_INSENSITIVE=1

    //This will make CASE_INSENSITIVE persist for the remainder of your shell session. It can be unset with the Remove-Item cmdlet:
    // Remove-Item Env:CASE_INSENSITIVE
    
    //Our minigrep program can now do case-insensitive searching controlled by an environment variable. 
    //Now you know how to manage options set using either command line arguments or environment variables.
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.file_name).unwrap_or_else(|err| {
        println!("Error while trying to open the file: {}", err);
        process::exit(1);
    });

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for lines in contents.lines() {
        if lines.contains(query) {
            results.push(lines);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
        
    }
}

//Note that we’ve edited the old test’s contents too. 
//We’ve added a new line with the text "Duct tape." using a capital D that shouldn’t match the query "duct" when we’re searching in a case-sensitive manner.
//This test should pass now and should continue to pass as we work on the case-insensitive search.
//The new test for the case-insensitive search uses "rUsT" as its query. In the search_case_insensitive function we’re about to add, the query "rUsT" 
//should match the line containing "Rust:" with a capital R and match the line "Trust me." even though both have different casing from the query.
//This is our failing test, and it will fail to compile because we haven’t yet defined the search_case_insensitive function.

//The search_case_insensitive function will be almost same as search function.
//The only difference is that we’ll lowercase the query and each line so whatever the case of the input arguments, 
//they’ll be the same case when we check whether the line contains the query.

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
//First, we lowercase the query string and store it in a shadowed variable with the same name. Calling to_lowercase on the query is necessary 
//so no matter whether the user’s query is "rust", "RUST", "Rust", or "rUsT", we’ll treat the query as if it were "rust" and be insensitive to the case.
//to_lowercase will handle basic Unicode, it won’t be 100% accurate. 
//If we were writing a real application, we’d want to do a bit more work here, but this section is about environment variables, not Unicode, so we’ll leave it at that here.

//Note that query is now a String rather than a string slice, because calling to_lowercase creates new data rather than referencing existing data. 
//Say the query is "rUsT", as an example: that string slice doesn’t contain a lowercase u or t for us to use, 
//so we have to allocate a new String containing "rust". When we pass query as an argument to the contains method now, 
//we need to add an ampersand because the signature of contains is defined to take a string slice.

//Next, we add a call to to_lowercase on each line before we check whether it contains query to lowercase all characters. 
//Now that we’ve converted line and query to lowercase, we’ll find matches no matter what the case of the query is.


//Now, let’s call the new search_case_insensitive function from the run function. 
//First, we’ll add a configuration option to the Config struct to switch between case-sensitive and case-insensitive search.
//Adding this field will cause compiler errors because we aren’t initializing this field anywhere yet:

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

//Note that we added the case_sensitive field that holds a Boolean. 
//Next, we need the run function to check the case_sensitive field’s value and use that to decide whether to call the search function or the search_case_insensitive function.
//check up


//Finally, we need to check for the environment variable. The functions for working with environment variables are in the env module in the standard library, 
//so we want to bring that module into scope with a use std::env; line at the top of src/lib.rs. 
//Then we’ll use the var function from the env module to check for an environment variable named CASE_INSENSITIVE
//check up



//Some programs allow arguments and environment variables for the same configuration. In those cases, the programs decide that one or the other takes precedence.
//For another exercise on your own, try controlling case insensitivity through either a command line argument or an environment variable.
//Decide whether the command line argument or the environment variable should take precedence if the program is run with one set to case sensitive and one set to case insensitive.
//The std::env module contains many more useful features for dealing with environment variables: check out its documentation to see what is available.

