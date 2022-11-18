fn main() {

    //Creating a new hash map
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);



    //Constructing hashmap using iterators and collect()
    let teams = vec![String::from("Blue"), String::from("Yellow")];

    let initial_scores = vec![10, 50];

    //we need to specify the type we want to collect() and store our data
    let scores: HashMap<String, i32> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();


    
    //Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!



    //Accessing Values in a Hash Map
    //We can get a value out of the hash map by providing its key to the get method
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); //score value is wrapped in Option<T> in case value is null (None)



    //Iteration over hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }//it will print key/values in arbitrary order



    //Updating HashMap
    //Overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);//Overwrites values previously inserted

    println!("{:?}", scores);



    //Only Inserting a Value If the Key Has No Value using entry()
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    //The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, 
    //and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.

    println!("{:?}", scores);



    //Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() { //split_whitespace() method divides string slice into sub slices divided based on whitespaces
        let count = map.entry(word).or_insert(0); //count is actually here equal to "value" that is linked to "key" in hashmap
        *count += 1; //increasing "value" by 1 and not some random i32, but first we dereference it using *
    }//count goes out of scope and mutable reference is dropped (so its safe and allowed by borrowing rules)

    println!("{:?}", map);



    //By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables
    //If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher.
    //A hasher is a type that implements the BuildHasher trait.

}