//Separation of Concerns for Binary Projects

//The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects.
//As a result, the Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. 

//The process has the following steps:
//Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
//As long as your command line parsing logic is small, it can remain in main.rs. 
//When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

//The responsibilities that remain in the main function after this process should be limited to the following:
//Calling the command line parsing logic with the argument values
//Setting up any other configuration
//Calling a run function in lib.rs
//Handling the error if run returns an error

//This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand.
//Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs.
//The only code that remains in main.rs will be small enough to verify its correctness by reading it.
//Let’s rework our program by following this process.


//Extracting the Argument Parser
//We’ll extract the functionality for parsing arguments into a function that main will call to prepare for moving the command line parsing logic to src/lib.rs.


//New start of main that calls a new function parse_config, which we’ll define in src/main.rs for the moment.
use std::env;
use std::fs;
use std::process;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    //We’ve used a method we haven’t covered in detail yet: unwrap_or_else, which is defined on Result<T, E> by the standard library.
    //Using unwrap_or_else allows us to define some custom, non-panic! error handling.
    //If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping.
    //However, if the value is an Err value, this method calls the code in the closure, which is an anonymous function we define and pass as an argument to unwrap_or_else.
    //For now, you just need to know that unwrap_or_else will pass the inner value of the Err, which in this case is the static string "not enough arguments" that we added 
    //to our closure in the argument err that appears between the vertical pipes.
    let config = Config::new(&args).unwrap_or_else(|err|{ 
        //The code in the closure can then use the err value when it runs.
        //We’ve added a new use line to bring process from the standard library into scope. 
        //The code in the closure that will be run in the error case is only two lines: we print the err value and then call process::exit.
        println!("Problem parsing arguments: {}", err);
        process::exit(1);//The process::exit function will stop the program immediately and return the number that was passed as the exit status code.
    });

    let contents = fs::read_to_string(config.file_name).expect("Error");
}

struct Config {
    query: String,
    file_name: String,
}
//We’ve updated main where we were calling parse_config to instead call Config::new. 
//We’ve changed the name of parse_config to new and moved it within an impl block, which associates the new function with Config.
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Not enough arguments provided!")
        }else {
            let query = args[1].clone();
            let file_name = args[2].clone();
            Ok(Config { query: query, file_name: file_name })
        }  
        //Our new function now returns a Result with a Config instance in the success case and a &'static str in the error case.
        //Our error values will always be string literals that have the 'static lifetime.
    }
}
//We’re still collecting the command line arguments into a vector, but instead of assigning the argument value at index 1 
//to the variable query and the argument value at index 2 to the variable filename within the main function, we pass the whole vector to the parse_config function.
//The parse_config function then holds the logic that determines which argument goes in which variable and passes the values back to main.
// We still create the query and filename variables in main, but main no longer has the responsibility of determining how the command line arguments and variables correspond.
//This rework may seem like overkill for our small program, but we’re refactoring in small, incremental steps.


//Grouping Configuration Values
//We can take another small step to improve the parse_config function further. 
//At the moment, we’re returning a tuple, but then we immediately break that tuple into individual parts again. 
//This is a sign that perhaps we don’t have the right abstraction yet.

//Another indicator that shows there’s room for improvement is the config part of parse_config, 
//which implies that the two values we return are related and are both part of one configuration value. 
//We’re not currently conveying this meaning in the structure of the data other than by grouping the two values into a tuple; 
//we could put the two values into one struct and give each of the struct fields a meaningful name.
//Doing so will make it easier for future maintainers of this code to understand how the different values relate to each other and what their purpose is.

//We’ve added a struct named Config defined to have fields named query and filename. The signature of parse_config now indicates that it returns a Config value. 
//In the body of parse_config, where we used to return string slices that reference String values in args, we now define Config to contain owned String values.
//The args variable in main is the owner of the argument values and is only letting the parse_config function borrow them, 
//which means we’d violate Rust’s borrowing rules if Config tried to take ownership of the values in args.

//We could manage the String data in a number of different ways, but the easiest, though somewhat inefficient, 
//route is to call the clone method on the values. This will make a full copy of the data for the Config instance to own, 
//which takes more time and memory than storing a reference to the string data. 
//However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references; 
//in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.

//We’ve updated main so it places the instance of Config returned by parse_config into a variable named config, 
//and we updated the code that previously used the separate query and filename variables so it now uses the fields on the Config struct instead.
//Now our code more clearly conveys that query and filename are related and that their purpose is to configure how the program will work.



//Creating a Constructor for Config

//So now that the purpose of the parse_config function is to create a Config instance, we can change parse_config from a plain function 
//to a function named new that is associated with the Config struct. 
//Making this change will make the code more idiomatic. 
//We can create instances of types in the standard library, such as String, by calling String::new. 
//Similarly, by changing parse_config into a new function associated with Config, we’ll be able to create instances of Config by calling Config::new.



//Fixing the Error Handling

//Recall that attempting to access the values in the args vector at index 1 or index 2 will cause the program to panic if the vector contains fewer than three items.
//The line index out of bounds: the len is 1 but the index is 1 is an error message intended for programmers. 
//It won’t help our end users understand what happened and what they should do instead. Let’s fix that now.

//Improving the Error Message
//We add a check in the new function that will verify that the slice is long enough before accessing index 1 and 2. 
//If the slice isn’t long enough, the program panics and displays a better error message than the index out of bounds message.
//This output is better: we now have a reasonable error message. However, we also have extraneous information we don’t want to give to our users. 
//A call to panic! is more appropriate for a programming problem than a usage problem.
//Instead, we can use the other technique, a Result that indicates either success or an error.

//Returning a Result from new Instead of Calling panic!
//We can instead return a Result value that will contain a Config instance in the successful case and will describe the problem in the error case. 
//When Config::new is communicating to main, we can use the Result type to signal there was a problem. 
//Then we can change main to convert an Err variant into a more practical error for our users without the 
//surrounding text about thread 'main' and RUST_BACKTRACE that a call to panic! causes.
//Returning an Err value from Config::new allows the main function to handle the Result value returned from the new function and exit the process more cleanly in the error case.


//Calling Config::new and Handling Errors

//To handle the error case and print a user-friendly message, we need to update main to handle the Result being returned by Config::new.
//We’ll also take the responsibility of exiting the command line tool with a nonzero error code from panic! and implement it by hand. 
//A nonzero exit status is a convention to signal to the process that called our program that the program exited with an error state.






