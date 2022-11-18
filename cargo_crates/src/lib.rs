//! # Cargo Crates
//!
//! `cargo_crates` is a collection of utilities to make performing certain
//! calculations more convenient.


//Customizing Builds with Release Profiles

//Cargo has two main profiles: the dev profile Cargo uses when you run cargo build and the release profile Cargo uses when you run cargo build --release. 
//The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.
//These profile names might be familiar from the output of your builds:

/*
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
*/

//The dev and release shown in this build output indicate that the compiler is using different profiles.

//Cargo has default settings for each of the profiles that apply when there aren’t any [profile.*] sections in the project’s Cargo.toml file. 
//By adding [profile.*] sections for any profile you want to customize, you can override any subset of the default settings. 
//For example, here are the default values for the opt-level setting for the dev and release profiles:

//Filename: Cargo.toml
/*
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
*/

//The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3. 
//Applying more optimizations extends compiling time, so if you’re in development and compiling your code often, 
//you’ll want faster compiling even if the resulting code runs slower. That is the reason the default opt-level for dev is 0.

//When you’re ready to release your code, it’s best to spend more time compiling. 
//You’ll only compile in release mode once, but you’ll run the compiled program many times, so release mode trades longer compile time for code that runs faster. 
//That is why the default opt-level for the release profile is 3.

//You can override any default setting by adding a different value for it in Cargo.toml. 
//For example, if we want to use optimization level 1 in the development profile, we can add these two lines to our project’s Cargo.toml file:
/*
[profile.dev]
opt-level = 1
*/

//This code overrides the default setting of 0. Now when we run cargo build, Cargo will use the defaults for the dev profile plus our customization to opt-level. 
//Because we set opt-level to 1, Cargo will apply more optimizations than the default, but not as many as in a release build.



//Publishing a Crate to Crates.io

//We’ve used packages from crates.io as dependencies of our project, but you can also share your code with other people by publishing your own packages. 
//The crate registry at crates.io distributes the source code of your packages, so it primarily hosts code that is open source.

//Making Useful Documentation Comments

//Accurately documenting your packages will help other users know how and when to use them, so it’s worth investing the time to write documentation.
//Rust also has a particular kind of comment for documentation, known conveniently as a documentation comment, that will generate HTML documentation. 
//The HTML displays the contents of documentation comments for public API items intended for programmers interested in knowing how to use your crate as opposed 
//to how your crate is implemented.

//Documentation comments use three slashes, ///, instead of two and support Markdown notation for formatting the text. 
//Place documentation comments just before the item they’re documenting. 
//Example:

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_crates::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

//Here, we give a description of what the add_one function does, start a section with the heading Examples, and then provide code that demonstrates how to use the add_one function. 
//We can generate the HTML documentation from this documentation comment by running cargo doc. 
//This command runs the rustdoc tool distributed with Rust and puts the generated HTML documentation in the target/doc directory.
//For convenience, running cargo doc --open will build the HTML for your current crate’s documentation 
//(as well as the documentation for all of your crate’s dependencies) and open the result in a web browser. 
//Navigate to the add_one function and you’ll see how the text in the documentation comments is rendered


//Commonly Used Sections

//We used the # Examples Markdown heading to create a section in the HTML with the title “Examples.” 
//Here are some other sections that crate authors commonly use in their documentation:
/*

    Panics: The scenarios in which the function being documented could panic. 
            Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.

    Errors: If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors 
            to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways.

    Safety: If the function is unsafe to call (we discuss unsafety in Chapter 19), there should be a section explaining why the function is 
            unsafe and covering the invariants that the function expects callers to uphold.

*/
//Most documentation comments don’t need all of these sections, but this is a good checklist to remind you of the aspects of your 
//code that people calling your code will be interested in knowing about.



//Documentation Comments as Tests

//Adding example code blocks in your documentation comments can help demonstrate how to use your library, and doing so has an additional bonus: running cargo test 
//will run the code examples in your documentation as tests! 
//Nothing is better than documentation with examples. 
//But nothing is worse than examples that don’t work because the code has changed since the documentation was written. 
//If we run cargo test with the documentation for the add_one function, we will see a section in the test results like this:
/*
        Doc-tests cargo_crates

    running 1 test
    test src/lib.rs - add_one (line 5) ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
*/
//Now if we change either the function or the example so the assert_eq! in the example panics and run cargo test again, 
//we’ll see that the doc tests catch that the example and the code are out of sync with each other!


//Commenting Contained Items

//Another style of doc comment, //!, adds documentation to the item that contains the comments rather than adding documentation 
//to the items following the comments. We typically use these doc comments inside the crate root file (src/lib.rs by convention) 
//or inside a module to document the crate or the module as a whole.

//For example, if we want to add documentation that describes the purpose of the my_crate crate that contains the add_one function, 
//we can add documentation comments that start with //! to the beginning of the src/lib.rs file
//check at the start of src.lib

//Notice there isn’t any code after the last line that begins with //!. 
//Because we started the comments with //! instead of ///, we’re documenting the item that contains this comment rather than an item that follows this comment. 
//In this case, the item that contains this comment is the src/lib.rs file, which is the crate root. These comments describe the entire crate.

//When we run cargo doc --open, these comments will display on the front page of the documentation for cargo_crates above the list of public items in the crate.

//Documentation comments within items are useful for describing crates and modules especially. 
//Use them to explain the overall purpose of the container to help your users understand the crate’s organization.



