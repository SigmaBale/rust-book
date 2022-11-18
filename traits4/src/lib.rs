//Using Trait Bounds to Conditionally Implement Methods

//By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
//For example, the type Pair<T> in Listing 10-16 always implements the new function to return a new instance of Pair<T> 
//Self is a type alias for the type of the impl block, which in this case is Pair<T>
//But in the next impl block, Pair<T> only implements the cmp_display method if its inner type T implements the 
//PartialOrd trait that enables comparison and the Display trait that enables printing.

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}//Listing 10-16



//We can also conditionally implement a trait for any type that implements another trait. 
//Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.
//For example, the standard library implements the ToString trait on any type that implements the Display trait. 
//The impl block in the standard library looks similar to this code:

// impl<T: Display> ToString for T {
//     --snip--
// }

//Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait.
//For example, we can turn integers into their corresponding String values like this because integers implement Display:

// let s = 3.to_string();

//Blanket implementations appear in the documentation for the trait in the “Implementors” section.