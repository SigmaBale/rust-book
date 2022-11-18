//Default Implementations
//Sometimes it’s useful to have default behavior for some or all of the methods in a trait 
//instead of requiring implementations for all methods on every type.


//We specify a default string for the summarize method of the Summary trait instead of only defining the method signature.
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

//To use a default implementation to summarize instances of NewsArticle, we specify an empty impl block with impl Summary for NewsArticle {}.
//Even though we’re no longer defining the summarize method on NewsArticle directly, we’ve provided 
//a default implementation and specified that NewsArticle implements the Summary trait.
//As a result, we can still call the summarize method on an instance of NewsArticle - check main.rs


