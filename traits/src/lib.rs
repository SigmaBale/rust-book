//Traits: Defining Shared Behavior
//A trait defines functionality a particular type has and can share with other types.
//We can use traits to define shared behavior in an abstract way.
//We can use trait bounds to specify that a generic type can be any type that has certain behavior.


//Defining a Trait
//A type’s behavior consists of the methods we can call on that type.
//Different types share the same behavior if we can call the same methods on all of those types.
//Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

//We want to make a media aggregator library crate named aggregator that can display summaries of data that might be stored in a NewsArticle or Tweet instance.
//To do this, we need a summary from each type, and we’ll request that summary by calling a summarize method on an instance. 
//Definition of a public Summary trait that expresses this behavior.
pub trait Summary {
    fn summarize(&self) -> String;
}
//Here, we declare a trait using the trait keyword and then the trait’s name, which is Summary in this case.
//We’ve also declared the trait as pub so that crates depending on this crate can make use of this trait too
//Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this trait
//Each type implementing this trait must provide its own custom behavior for the body of the method.
//The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.
//A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.


//Implementing a Trait on a Type
//Implementation of the Summary trait on the NewsArticle struct that uses the headline, the author, and the location to create the return value of summarize.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


//For the Tweet struct, we define summarize as the username followed by the entire text of the tweet, 
//assuming that tweet content is already limited to 280 characters.
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


//Implementing a trait on a type is similar to implementing regular methods. The difference is that after impl, 
//we put the trait name we want to implement, then use the for keyword, and then specify the name of the type we want to implement the trait for.
//Within the impl block, we put the method signatures that the trait definition has defined. Instead of adding a semicolon after each signature, 
//we use curly brackets and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.

//Now that the library has implemented the Summary trait on NewsArticle and Tweet, 
//users of the crate can call the trait methods on instances of NewsArticle and Tweet in the same way we call regular methods.
//check main.rs