//Generic Type Parameters, Trait Bounds, and Lifetimes Together

//Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!

use std::fmt::Display;

fn longest_with_an_annoucement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    }else{
        y
    }
}
//This is the longest function from Listing 10-22 that returns the longer of two string slices. 
//But now it has an extra parameter named ann of the generic type T, which can be filled 
//in by any type that implements the Display trait as specified by the where clause.
