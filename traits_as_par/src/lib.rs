use std::fmt::Display;
//Traits as parameters

//We'll use the Summary trait we implemented on the NewsArticle and Tweet types
//to define a notify function that calls the summarize method on its item parameter, which is of some type that implements the Summary trait.

//To do this, we use the impl Trait syntax
pub fn notify(item: &impl Summary) { //Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name.
    println!("Breaking news! {}", item.summarize()); //This parameter accepts any type that implements the specified trait.
    //In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize.
}
//We can call notify and pass in any instance of NewsArticle or Tweet.
//Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.


//Trait Bound Syntax
//The impl Trait syntax works for straightforward cases but is actually syntax sugar 
//for a longer form known as a trait bound; it looks like this:
pub fn notify2<T: Summary>(item: &T) { //We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.
    println!("Breaking news! {}", item.summarize());
}

//The impl Trait syntax is convenient and makes for more concise code in simple cases, 
//while the fuller trait bound syntax can express more complexity in other cases.
//We can have two parameters that implement Summary. Doing so with the impl Trait syntax looks like this:
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} - {}", item1.summarize(), item2.summarize());
}//Using impl Trait is appropriate if we want this function to allow item1 and item2 to have different types (as long as both types implement Summary).

//If we want to force both parameters to have the same type, however, we must use a trait bound, like this:
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} - {}", item1.summarize(), item2.summarize());
}//The generic type T specified as the type of the item1 and item2 parameters constrains the function such that the concrete type 
//of the value passed as an argument for item1 and item2 must be the same.


pub trait Summary {

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

}

pub trait Summary2 {

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Summary for NewsArticle {}




//Specifying Multiple Trait Bounds with the + Syntax

//We can also specify more than one trait bound.
//Say we wanted notify to use display formatting as well as summarize on item: 
//we specify in the notify definition that item must implement both Display and Summary.
//We can do so using the + syntax:
pub fn notify5(item: &(impl Summary + Display)) {
    println!()
}

//The + syntax is also valid with trait bounds on generic types:
//  pub fn notify<T: Summary + Display>(item: &T) {...
//With the two trait bounds specified, the body of notify can call summarize and use {} to format item.



use std::fmt::Debug;
//Clearer Trait Bounds with where Clauses
//Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait 
//bound information between the function’s name and its parameter list, making the function signature hard to read.
//For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. 

//So instead of writing this:
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

//We can use a where clause, like this:
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    0
}
//This function’s signature is less cluttered: the function name, parameter list, and return type are close together, similar to a function without lots of trait bounds.







