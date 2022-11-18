use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

fn main() {

    //Shortcuts for Panic on Error: unwrap and expect
    //If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
    //If the Result is the Err variant, unwrap will call the panic! macro for us.

    // let f = File::open("hello.txt").unwrap();




    //Similarly, the expect method lets us also choose the panic! error message. 
    //Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.

    // let f = File::open("hello.txt").expect("Failed to open hello.txt");



    
    //Propagating Errors
    //function that reads a username from a file. If the file doesn’t exist or can’t be read, 
    //this function will return those errors to the code that called the function.

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
    
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    
        let mut s = String::new();
    
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }




    //A Shortcut for Propagating Errors: the ? Operator
    //The ? placed after a Result value is defined to work in almost the same way as the match expressions 
    //we defined to handle the Result values in Listing 9-6. If the value of the Result is an Ok, 
    //the value inside the Ok will get returned from this expression, and the program will continue. 
    //If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword 
    //so the error value gets propagated to the calling code.

    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    //When the ? operator calls the 'from' function, the error type received is converted into the error type defined in the return type of the current function.
    //As long as there’s an impl From<OtherError> for ReturnedError to define the conversion in the trait’s from function, 
    //the ? operator takes care of calling the from function automatically.




    
    //Chaining method calls
    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut s = String::new();
    
        File::open("hello.txt")?.read_to_string(&mut s)?;
    
        Ok(s)
    }




    //Shorter way
    fn read_username_from_file_4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }




    //The error message also mentioned that ? can be used with Option<T> values as well. 
    //As with using ? on Result, you can only use ? on Option in a function that returns an Option. 
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }




    //main can also return a Result<(), E>.
    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;
    
        Ok(())
    }
    //You can read Box<dyn Error> to mean “any kind of error.” 
    //Using ? on a Result value in a main function with the error type Box<dyn Error> is allowed, 
    //because it allows any Err value to be returned early.
    //The main function may return any types that implement the std::process::Termination trait. 
    //As of this writing, the Termination trait is an unstable feature only available in Nightly Rust, 
    //so you can’t yet implement it for your own types in Stable Rust, but you might be able to someday!




    //if it was absolutely critical that the program only operated on values between 1 and 100, 
    //and it had many functions with this requirement, having a check like this in every function 
    //would be tedious (and might impact performance).
    //Instead, we can make a new type and put the validations in a function to create an instance of the type 
    //rather than repeating the validations everywhere.
    
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }



}
