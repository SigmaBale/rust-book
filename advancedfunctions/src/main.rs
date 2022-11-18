/*
The fn type is called a function pointer. 
The syntax for specifying that a parameter is a function pointer is similar to that of closures.
*/

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

/*
Unlike closures, fn is a type rather than a trait, so we specify fn as the parameter type 
directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.

Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), 
so you can always pass a function pointer as an argument for a function that expects a closure. 
It’s best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures.

An example of where you would want to only accept fn and not closures is when interfacing with external code that doesn’t have closures: 
C functions can accept functions as arguments, but C doesn’t have closures.

As an example of where you could use either a closure defined inline or a named function, let’s look at a use of map. 
To use the map function to turn a vector of numbers into a vector of strings, we could use a closure, like this:

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();

Or we could name a function as the argument to map instead of the closure, like this:

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();




Each enum variant that we define also becomes an initializer function.
We can use these initializer functions as function pointers that implement the closure traits, 
which means we can specify the initializer functions as arguments for methods that take closures, like so:

    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

Here we create Status::Value instances using each u32 value in the range that map is called on by using the initializer function of Status::Value.
*/



//Returning Closures

/*
Closures are represented by traits, which means you can’t return closures directly. 
In most cases where you might want to return a trait, you can instead use the concrete type that implements the trait as the return value of the function.
But you can’t do that with closures because they don’t have a concrete type that is returnable; 
you’re not allowed to use the function pointer fn as a return type, for example.

The following code tries to return a closure directly, but it won’t compile:
    
    fn returns_closure() -> dyn Fn(i32) -> i32 {
        |x| x + 1
    }

The compiler error is as follows:

    $ cargo build
    Compiling functions-example v0.1.0 (file:///projects/functions-example)
    error[E0746]: return type cannot have an unboxed trait object
    --> src/lib.rs:1:25
    |
    1 | fn returns_closure() -> dyn Fn(i32) -> i32 {
    |                         ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
    help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@src/lib.rs:2:5: 2:14]`, which implements `Fn(i32) -> i32`
    |
    1 | fn returns_closure() -> impl Fn(i32) -> i32 {
    |                         ~~~~~~~~~~~~~~~~~~~

    For more information about this error, try `rustc --explain E0746`.
    error: could not compile `functions-example` due to previous error


The error references the Sized trait again! Rust doesn’t know how much space it will need to store the closure. 
We saw a solution to this problem earlier. We can use a trait object:

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

This code will compile just fine.
*/