//extra
//Note that std::env::args will panic if any argument contains invalid Unicode. 
//If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead.
//That function returns an iterator that produces OsString values instead of String values. 

fn main() {

    //Allow your minigrep program to read any command line arguments passed to it and then collect the values into a vector.
    //First, we bring the std::env module into scope with a use statement so we can use its args function.
    use std::env; //Notice that the std::env::args function is nested in two levels of modules.
    //it’s conventional to bring the parent module into scope rather than the function.
    //By doing so, we can easily use other functions from std::env.

    use std::fs;

    //On the first line of main, we call env::args, and we immediately use collect to turn the iterator into a vector containing all the values produced by the iterator.
    let args: Vec<String> = env::args().collect(); //We can use the collect function to create many kinds of collections, so we explicitly annotate the type 
    //of args to specify that we want a vector of strings.

    //Notice that the first value in the vector is "target/debug/minigrep", which is the name of our binary.
    println!("{:?}", args);//This matches the behavior of the arguments list in C, letting programs use the name by which they were invoked in their execution.
    //It’s often convenient to have access to the program name in case you want to print it in messages or change behavior of the program 
    //based on what command line alias was used to invoke the program.


    //Saving the Argument Values in Variables

    //Now we need to save the values of the two arguments in variables so we can use the values throughout the rest of the program.
    //As we saw when we printed the vector, the program’s name takes up the first value in the vector at args[0], so we’re starting at index 1.
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for query {query}");
    println!("In file {filename}");
    //Great, the program is working! The values of the arguments we need are being saved into the right variables.


    //Reading a File

    //Now we’ll add functionality to read the file that is specified in the filename command line argument.
    //First, we need a sample file to test it with: the best kind of file to use to make sure minigrep is working is 
    //one with a small amount of text over multiple lines with some repeated words.
    //Create a file called poem.txt at the root level of your project

    //With the text in place, edit src/main.rs and add code to read the file

    //First, we add another use statement to bring in a relevant part of the standard library: we need std::fs to handle files.
    //In main, we’ve added a new statement: fs::read_to_string takes the filename, opens that file, and returns a Result<String> of the file’s contents.
    
    let file_contents = fs::read_to_string("poem.txt").expect("File not found!");
    println!("File contents:\n{}", file_contents);

    //Great! The code read and then printed the contents of the file. But the code has a few flaws.


    //Refactoring to Improve Modularity and Error Handling

    //To improve our program, we’ll fix four problems that have to do with the program’s structure and how it’s handling potential errors.
    //First, our main function now performs two tasks: it parses arguments and reads files. For such a small function, this isn’t a major problem. 
    //However, if we continue to grow our program inside main, the number of separate tasks the main function handles will increase.
    //As a function gains responsibilities, it becomes more difficult to reason about, harder to test, and harder to change without breaking one of its parts. 
    //It’s best to separate functionality so each function is responsible for one task.

    //The longer main becomes, the more variables we’ll need to bring into scope; the more variables we have in scope, 
    //the harder it will be to keep track of the purpose of each.
    //It’s best to group the configuration variables into one structure to make their purpose clear.

    //The third problem is that we’ve used expect to print an error message when reading the file fails, but the error message just prints Something went wrong reading the file.
    //Reading a file can fail in a number of ways: for example, the file could be missing, or we might not have permission to open it.
    //Fourth, we use expect repeatedly to handle different errors, and if the user runs our program without specifying enough arguments, 
    //they’ll get an index out of bounds error from Rust that doesn’t clearly explain the problem.

    //It would be best if all the error-handling code were in one place so future maintainers had only one place to consult in the code if the error-handling logic needed to change.
    //Let’s address these four problems by refactoring our project.
    //check minigrep2


}
