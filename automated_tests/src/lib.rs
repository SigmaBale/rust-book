//How to Write Tests

//Tests are Rust functions that verify that the non-test code is functioning in the expected manner.
//The bodies of test functions typically perform these three actions:
// 1¸ Set up any needed data or state.
// 2. Run the code you want to test. 
// 3. Assert the results are what you expect.



//THE ANATOMY OF A TEST FUNCTION

//At its simplest, a test in Rust is a function that’s annotated with the test attribute.
//Attributes are metadata about pieces of Rust code; one example is the derive attribute.
//To change a function into a test function, add #[test] on the line before fn.
// When you run your tests with the cargo test command, Rust builds a test runner binary that runs the functions 
//annotated with the test attribute and reports on whether each test function passes or fails.

//When we make a new library project with Cargo, a test module with a test function in it is automatically generated for us.
//You can add as many additional test functions and as many test modules as you want!

//Let’s create a new library project!
// cargo new project_name --lib

//The contents of the src/lib.rs file in your project library should look like this:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another() {
        panic!("Make this test fail");
    
    }//Tests fail when something in the test function panics.
    //Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 6,
        };

        let smaller = Rectangle {
            width: 6,
            height: 5,
        };
        
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 6,
        };

        let smaller = Rectangle {
            width: 6,
            height: 5,
        };
        
        assert!(!smaller.can_hold(&larger));
    }
}
//Note the #[test] annotation before the fn line: this attribute indicates this is a test function, so the test runner knows to treat this function as a test.
//We could also have non-test functions in the tests module to help set up common scenarios or perform common operations, 
//so we need to indicate which functions are tests by using the #[test] attribute.

//The function body uses the assert_eq! macro to assert that 2 + 2 equals 4.
//This assertion serves as an example of the format for a typical test.
//The cargo test command runs all tests in our project.

//Let’s change the name of our test to see how that changes the test output.






//CHECKING RESULT WITH THE assert! MACRO

//The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true.
//We give the assert! macro an argument that evaluates to a Boolean.
//If the value is true, assert! does nothing and the test passes. If the value is false, the assert! macro calls the panic! macro, which causes the test to fail. 

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
//We test function implementing this struct using assert! macro in the code above.
//Note that we’ve added a new line inside the tests module: use super::*;. The tests module is a regular module that follows the usual visibility rules
//Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module.
//We use a glob here so anything we define in the outer module is available to this tests module.






//TESTING EQUALITY WITH THE assert_eq! AND assert_ne! MACROS

//A common way to test functionality is to compare the result of the code under test to the value you expect the code to return to make sure they’re equal.
//Standard library provides a pair of macros—assert_eq! and assert_ne!
//These macros compare two arguments for equality or inequality, respectively. 

//We write a function named add_two that adds 2 to its parameter and returns the result. Then we test this function using the assert_eq! macro.
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
//Note that in some languages and test frameworks, the parameters to the functions that assert two values are equal are called 
//expected and actual, and the order in which we specify the arguments matters.
//However, in Rust, they’re called left and right, and the order in which we specify the value we expect and the value that the code under test produces doesn’t matter.
//The assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal. This macro is most useful for cases when we’re not sure what a value will be, 
//but we know what the value definitely won’t be if our code is functioning as we intend.
//Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively.
//When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits.
//All the primitive types and most of the standard library types implement these traits.

//For structs and enums that you define, you’ll need to implement PartialEq to assert that values of those types are equal or not equal.
//You’ll need to implement Debug to print the values when the assertion fails.
//Because both traits are derivable traits this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition.





//Adding Custom Failure Messages

//You can also add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros.
//Any arguments specified after the one required argument to assert! or the two required arguments to assert_eq! and assert_ne! are passed along to the format! macro
//so you can pass a format string that contains {} placeholders and values to go in those placeholders.
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Mile"), "Greeting did not contain name, value was `{}`",result);
    }
}





//Checking for Panics with should_panic

//In addition to checking that our code returns the correct values we expect, it’s also important to check that our code handles error conditions as we expect.
//For example, consider the Guess type that we created
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
}
//Other code that uses Guess depends on the guarantee that Guess instances will contain only values between 1 and 100.
//We can write a test that ensures that attempting to create a Guess instance with a value outside that range panics.
//We do this by adding another attribute, should_panic, to our test function. This attribute makes a test pass if the code 
//inside the function panics; the test will fail if the code inside the function doesn’t panic.

#[cfg(test)]
mod tests4 {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

//We place the #[should_panic] attribute after the #[test] attribute and before the test function it applies to.
//Tests that use should_panic can be imprecise because they only indicate that the code has caused some panic. 
//A should_panic test would pass even if the test panics for a different reason from the one we were expecting to happen.
//To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute.

// #[should_panic(expected = "Guess value must be less than or equal to 100")]





//Using Result<T, E> in Tests

//Here’s the test rewritten to use Result<T, E> and return an Err instead of panicking:
#[cfg(test)]
mod tests5 {
    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
//Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, 
//which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.
//You can’t use the #[should_panic] annotation on tests that use Result<T, E>. To assert that an operation returns an Err variant, 
//don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
