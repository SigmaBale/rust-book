use test_organization;

mod common;
//Note that the mod common; declaration is the same as the module declaration we demonstrated in Listing 7-21. 

//Then in the test function, we can call the common::setup() function.
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}

//We’ve added use test_organization at the top of the code, which we didn’t need in the unit tests. 
//The reason is that each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope.
//We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. 
//Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.

//Run cargo test now:
//The three sections of output include the unit tests, the integration test, and the doc tests.
//The first section for the unit tests is the same as we’ve been seeing: one line for each unit test and then a summary line for the unit tests.
//The integration tests section starts with the line Running target/debug/deps/integration_test-1082c4b063a8fbe6 (the hash at the end of your output will be different). 
//Next, there is a line for each test function in that integration test and a summary line 
//for the results of the integration test just before the Doc-tests test_organization section starts.

//Similarly to how adding more unit test functions adds more result lines to the unit tests section, 
//adding more test functions to the integration test file adds more result lines to this integration test file’s section.
//Each integration test file has its own section, so if we add more files in the tests directory, there will be more integration test sections.

//We can still run a particular integration test function by specifying the test function’s name as an argument to cargo test.
//To run all the tests in a particular integration test file, use the --test argument of cargo test followed by the name of the file.



//Submodules in Integration Tests

//As you add more integration tests, you might want to make more than one file in the tests directory to help organize them; 
//for example, you can group the test functions by the functionality they’re testing.
//As mentioned earlier, each file in the tests directory is compiled as its own separate crate.

//Treating each integration test file as its own crate is useful to create separate scopes that are more like the way end users will be using your crate.
//However, this means files in the tests directory don’t share the same behavior as files in src do
//The different behavior of files in the tests directory is most noticeable when you have a set of helper functions that would be useful in multiple 
//integration test files and you try to follow the steps in the “Separating Modules into Different Files” section of Chapter 7 to extract them into a common module.

//For example, if we create tests/common.rs and place a function named setup in it, 
//we can add some code to setup that we want to call from multiple test functions in multiple test files: check tests/common.rs