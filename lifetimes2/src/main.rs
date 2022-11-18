//Generic Lifetimes in Functions

//We’ll write a function that returns the longer of two string slices. 
//This function will take two string slices and return a single string slice. 
//After we’ve implemented the longest function, the code in Listing 10-20 should print The longest string is abcd.

use lifetimes2::longest;
fn main() {
    let string1 = String::from("long string is long");
    let result;
    let result2: &str;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result2);  
}
//The error shows that for result to be valid for the println! statement, string2 would need to be valid until the end of the outer scope.

