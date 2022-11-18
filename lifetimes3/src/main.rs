//Thinking in Terms of Lifetimes

//The way in which you need to specify lifetime parameters depends on what your function is doing. 
//For example, if we changed the implementation of the longest function to always return the first 
//parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter.
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
//We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, 
//because the lifetime of y does not have any relationship with the lifetime of x or the return value.
//When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.

//If the reference returned does not refer to one of the parameters, it must refer to a value created within this function. 
//However, this would be a dangling reference because the value will go out of scope at the end of the function. 
//Consider this attempted implementation of the longest function that won’t compile:
fn longest2<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
//Here, even though we’ve specified a lifetime parameter 'a for the return type, 
//this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all.

//The problem is that result goes out of scope and gets cleaned up at the end of the longest function.
//We’re also trying to return a reference to result from the function.
//There is no way we can specify lifetime parameters that would change the dangling reference, and Rust won’t let us create a dangling reference.
//In this case, the best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value.

//Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. 
//Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create 
//dangling pointers or otherwise violate memory safety.





//Lifetime Annotations in Struct Definitions

//We can define structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition. 
struct ImportantExcerpt2<'a> {
    part: &'a str,
}
//This struct has one field, part, that holds a string slice, which is a reference.
//As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name 
//of the struct so we can use the lifetime parameter in the body of the struct definition.
//This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    //The main function here creates an instance of the ImportantExcerpt struct that holds a reference to the first sentence of the String owned by the variable novel. 
    //The data in novel exists before the ImportantExcerpt instance is created. 
    //In addition, novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is valid.
    

    //The 'static lifetime
    let s: &'static str = "I have a static lifetime.";
    //The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.
    //efore specifying 'static as the lifetime for a reference, think about whether the reference 
    //you have actually lives the entire lifetime of your program or not, and whether you want it to.
    //Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes.
    //In such cases, the solution is fixing those problems, not specifying the 'static lifetime.
}






