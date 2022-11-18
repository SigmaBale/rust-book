struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

//Advanced Traits

//Specifying Placeholder Types in Trait Definitions with Associated Types

/*
Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.
The implementor of a trait will specify the concrete type to be used in this type’s place for the particular implementation. 
That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.

The definition of the Iterator trait is as shown in Listing 19-12.

    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    Listing 19-12


Associated types might seem like a similar concept to generics, in that the latter allow us to define a function without specifying what types it can handle. 
So why use associated types?

Let’s examine the difference between the two concepts with an example that implements the Iterator trait on a Counter struct. 
This implementation specifies the Item type is u32:
*/
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/*
This syntax seems comparable to that of generics. So why not just define the Iterator trait with generics, as shown in Listing 19-13?

    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }

    Listing 19-13


The difference is that when using generics, as in Listing 19-13, we must annotate the types in each implementation; 
because we can also implement Iterator<String> for Counter or any other type, we could have multiple implementations of Iterator for Counter.
With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times. 
In Listing 19-12 with the definition that uses associated types, we can only choose what the type of Item will be once, 
because there can only be one impl Iterator for Counter.
*/



//Default Generic Type Parameters and Operator Overloading

/*
When we use generic type parameters, we can specify a default concrete type for the generic type.
The syntax for specifying a default type for a generic type is <PlaceholderType=ConcreteType> when declaring the generic type.

A great example of a situation where this technique is useful is with operator overloading. 
Operator overloading is customizing the behavior of an operator (such as +) in particular situations.

Rust doesn’t allow you to create your own operators or overload arbitrary operators. 
But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator. 
Check main.rs!
*/


//Changing default type parametert from Self to Meters.
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}



//Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

/*
Nothing in Rust prevents a trait from having a method with the same name as another trait’s method, nor does Rust prevent you from implementing both traits on one type. 
It’s also possible to implement a method directly on the type with the same name as methods from traits.

When calling methods with the same name, you’ll need to tell Rust which one you want to use.
*/

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

/*
When we call fly on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type:

    fn main() {
        let person = Human;
        person.fly();
    }

Running this code will print *waving arms furiously*, showing that Rust called the fly method implemented on Human directly.
To call the fly methods from either the Pilot trait or the Wizard trait, we need to use more explicit syntax to specify which fly method we mean.

    fn main() {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
    }

Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call.
Associated functions that are not methods don’t have a self parameter. 
When there are multiple types or traits that define non-method functions with the same function name, 
Rust doesn't always know which type you mean unless you use fully qualified syntax.
*/
pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
/*
In main, we call the Dog::baby_name function, which calls the associated function defined on Dog directly. 
This code prints the following:

    $ cargo run
    Compiling traits-example v0.1.0 (file:///projects/traits-example)
      Finished dev [unoptimized + debuginfo] target(s) in 0.54s
        Running `target/debug/traits-example`
    A baby dog is called a Spot


To disambiguate and tell Rust that we want to use the implementation of Animal for Dog
as opposed to the implementation of Animal for some other type, we need to use fully qualified syntax.

    fn main() {
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }

We’re providing Rust with a type annotation within the angle brackets, which indicates we want to call the baby_name method from the Animal trait as implemented on Dog 
by saying that we want to treat the Dog type as an Animal for this function call.

In general, fully qualified syntax is defined as follows:

    <Type as Trait>::function(receiver_if_method, next_arg, ...);


For associated functions that aren’t methods, there would not be a receiver: there would only be the list of other arguments. 
You could use fully qualified syntax everywhere that you call functions or methods. 
You only need to use this more verbose syntax in cases where there are multiple implementations 
that use the same name and Rust needs help to identify which implementation you want to call.
*/



//Using Supertraits to Require One Trait’s Functionality Within Another Trait

/*
Sometimes, you might need one trait to use another trait’s functionality. 
In this case, you need to rely on the dependent trait also being implemented. 
The trait you rely on is a supertrait of the trait you’re implementing.

We need to specify that the OutlinePrint trait will work only for types that also implement Display 
and provide the functionality that OutlinePrint needs.
We can do that in the trait definition by specifying OutlinePrint: Display.
*/
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
/*
Because we’ve specified that OutlinePrint requires the Display trait, 
we can use the to_string function that is automatically implemented for any type that implements Display.
If we tried to use to_string without adding a colon and specifying the Display trait after the trait name, 
we’d get an error saying that no method named to_string was found for the type &Self in the current scope.
*/
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}




//Using the Newtype Pattern to Implement External Traits on External Types

/*
We mentioned the orphan rule that states we’re allowed to implement a trait on a type as long as either the trait or the type are local to our crate. 
It’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct. 
*/
pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}