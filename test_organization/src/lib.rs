//Test Organization

//The Rust community thinks about tests in terms of two main categories: unit tests and integration tests.
//Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.
//Integration tests are entirely external to your library and use your code in the same way any other external code would, 
//using only the public interface and potentially exercising multiple modules per test.

//Writing both kinds of tests is important to ensure that the pieces of your library are doing what you expect them to, separately and together.




//UNIT TESTS

//The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected.
//You’ll put unit tests in the src directory in each file with the code that they’re testing.
//The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).


//The Tests Module and #[cfg(test)]

//The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.
//This saves compile time when you only want to build the library and saves space in the resulting compiled artifact because the tests are not included.
//You’ll see that because integration tests go in a different directory, they don’t need the #[cfg(test)] annotation. However, because unit tests go in the same files 
//as the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.

//Recall that when we generated the new lib project, cargo generated this for us:
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
//The attribute cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option.
//In this case, the configuration option is test, which is provided by Rust for compiling and running tests.
//By using the cfg attribute, Cargo compiles our test code only if we actively run the tests with cargo test.
//This includes any helper functions that might be within this module, in addition to the functions annotated with #[test].


//Testing Private Functions

//Rust’s privacy rules do allow you to test private functions.
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 { //private function
    a + b
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }//tested here
}







//INTEGRATION TESTS

//In Rust, integration tests are entirely external to your library. 
//They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API.
//Their purpose is to test whether many parts of your library work together correctly.

//The tests Directory

//We create a tests directory at the top level of our project directory, next to src. Cargo knows to look for integration test files in this directory. 
//We can then make as many test files as we want to in this directory, and Cargo will compile each of the files as an individual crate.
//Let’s create an integration test.
//With the code in Listing 11-12 still in the src/lib.rs file, make a tests directory, 
//create a new file named tests/integration_test.rs, and enter the code in Listing 11-13.

//check tests/integration_test.rs





//Integration Tests for Binary Crates

//If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, 
//we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement.

//Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

//This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file.
//Using that structure, integration tests can test the library crate with use to make the important functionality available.
//If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.