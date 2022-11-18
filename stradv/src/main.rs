fn main() {

    //empty String creation
    let mut s = String::new();



    //String creation using to_string method
    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();



    //String creation using String::from() method
    let s = String::from("initial contents");



    //Appending to a String with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");



    //push_str() takes string slice (does not take ownership)
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    


    //push method
    let mut s = String::from("lo");
    s.push('l');



    //Concatention with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    //The + operator uses the add method, whose signature looks something like this:
    //fn add(self, s: &str) -> String {

    

    //format!() macro does not take ownership of any of its arguments because it uses references
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); //s = "tic-tac-toe"
    


    //If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:
    //[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //That’s 18 bytes and is how computers ultimately store this data. 

    //If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:
    //['न', 'म', 'स', '्', 'त', 'े']
    //There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own.

    //Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:
    //["न", "म", "स्", "ते"]

    

    //Slicing Strings - Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:
    let hello = "Здравствуйте";

    let s = &hello[0..4]; //s contains first 4 bytes, bytes at index (0,1,2,3) so s = "Зд"



    //Methods for Iterating Over Strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }



    //Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library.

}
