use std::thread;
use std::time::Duration;
use std::collections::HashMap;

//CLOSURES

//Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. 
//You can create the closure in one place and then call the closure to evaluate it in a different context. 
//Unlike functions, closures can capture values from the scope in which they’re defined.

//To define a closure, we start with a pair of vertical pipes (|), inside which we specify the parameters to the closure; 
//this syntax was chosen because of its similarity to closure definitions in Smalltalk and Ruby. 
//If we had more than one parameter, we would separate them with commas, like |param1, param2|.

//After the parameters, we place curly brackets that hold the body of the closure—these are optional if the closure body is a single expression.


pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        //The struct will execute the closure only if we need the resulting value, 
        //and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. 
        //You may know this pattern as memoization or lazy evaluation.
        if !self.value.contains_key(&arg) {
            self.value.insert(arg, (self.calculation)(arg));
            self.value[&arg]
        }else {
            self.value[&arg]
        }
    }
}

//To make a struct that holds a closure, we need to specify the type of the closure, 
//because a struct definition needs to know the types of each of its fields. 
//Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature, their types are still considered different. 
//To define structs, enums, or function parameters that use closures, we use generics and trait bounds.
struct Cacher<T> 
where 
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}


//Limitations of the Cacher Implementation (closures)
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(assert_eq!(v2, 2), assert_eq!(_v1, 1));
}

//Try modifying Cacher to hold a hash map rather than a single value. 
//The keys of the hash map will be the arg values that are passed in, and the values of the hash map will be the result of calling the closure on that key.
//Instead of looking at whether self.value directly has a Some or a None value, the value function will look up the arg in the hash map and return the value if it’s present.
//If it’s not present, the Cacher will call the closure and save the resulting value in the hash map associated with its arg value.

//The second problem with the current Cacher implementation is that it only accepts closures that take one parameter of type u32 and return a u32. 
//We might want to cache the results of closures that take a string slice and return usize values, for example. 
//To fix this issue, try introducing more generic parameters to increase the flexibility of the Cacher functionality.


//Capturing the Environment with Closures

//In the workout generator example, we only used closures as inline anonymous functions. 
//However, closures have an additional capability that functions don’t have: 
//they can capture their environment and access variables from the scope in which they’re defined.

//An example of a closure stored in the equal_to_x variable that uses the x variable from the closure’s surrounding environment.
//check main.rs