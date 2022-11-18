//Here’s an example of how a binary crate could use our aggregator library crate
use traits::{NewsArticle,Summary,Tweet};

fn main() {
    
    let fritz_news = NewsArticle {
        headline: "Covijek usrao karbonski naslon za ruku u Cinestaru".to_string(),
        location: "Albanija".to_string(),
        author: "Črni".to_string(),
        content: "Čovijek posrao 80 tona govana u Cinestar kinu prekjučer preksutra. Šok i zaprepaščenje, lijep pozdrav.".to_string()
    };

    println!("{}", fritz_news.summarize());

    let alban_influencer = Tweet {
        username: "Alban69".to_string(),
        content: "Jeli ko vidio žutog miloševića prekjučer preksutra? Jeli on se posrao u cinestar? Javite mi esemesom".to_string(),
        reply: false,
        retweet: true
    };

    println!("{}", alban_influencer.summarize());
}

//Other crates that depend on the traits crate can also bring the Summary trait into scope to implement Summary on their own types.
//One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate.
//For example, we can implement standard library traits like Display on a custom type like Tweet as part of our traits crate functionality,
//because the type Tweet is local to our traits crate.

//But we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our traits crate, 
//because Display and Vec<T> are both defined in the standard library and aren’t local to our traits crate.
//This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present.
//This rule ensures that other people’s code can’t break your code and vice versa. 
