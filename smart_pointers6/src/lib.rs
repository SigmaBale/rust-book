//Reference Cycles Can Leak Memory

/*
Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak). 
Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust. 
We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other in a cycle. 
This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.
*/

//Creating a Reference Cycle

/*
Let’s look at how a reference cycle might happen and how to prevent it, starting with the definition of the List enum and a tail method in Listing 15-25:

    use crate::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    fn main() {}

    Listing 15-25


We’re using another variation of the List definition from Listing 15-5. 
The second element in the Cons variant is now RefCell<Rc<List>>, meaning that instead of having the ability to modify the i32 value as we did in Listing 15-24, 
we want to modify which List value a Cons variant is pointing to. 
We’re also adding a tail method to make it convenient for us to access the second item if we have a Cons variant.

In Listing 15-26, we’re adding a main function that uses the definitions in Listing 15-25. 
This code creates a list in a and a list in b that points to the list in a. 
Then it modifies the list in a to point to b, creating a reference cycle. 
There are println! statements along the way to show what the reference counts are at various points in this process.

    fn main() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }
    Listing 15-26



We create an Rc<List> instance holding a List value in the variable a with an initial list of 5, Nil. 
We then create an Rc<List> instance holding another List value in the variable b that contains the value 10 and points to the list in a.

We modify a so it points to b instead of Nil, creating a cycle. 
We do that by using the tail method to get a reference to the RefCell<Rc<List>> in a, which we put in the variable link. 
Then we use the borrow_mut method on the RefCell<Rc<List>> to change the value inside from an Rc<List> that holds a Nil value to the Rc<List> in b.

When we run this code, keeping the last println! commented out for the moment, we’ll get this output:

    $ cargo run
    Compiling cons-list v0.1.0 (file:///projects/cons-list)
        Finished dev [unoptimized + debuginfo] target(s) in 0.53s
        Running `target/debug/cons-list`
    a initial rc count = 1
    a next item = Some(RefCell { value: Nil })
    a rc count after b creation = 2
    b initial rc count = 1
    b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
    b rc count after changing a = 2
    a rc count after changing a = 2


The reference count of the Rc<List> instances in both a and b are 2 after we change the list in a to point to b. 
At the end of main, Rust drops the variable b, which decreases the reference count of the Rc<List> instance from 2 to 1. 
The memory that Rc<List> has on the heap won’t be dropped at this point, because its reference count is 1, not 0. 
Then Rust drops a, which decreases the reference count of the a Rc<List> instance from 2 to 1 as well. 
This instance’s memory can’t be dropped either, because the other Rc<List> instance still refers to it. 
The memory allocated to the list will remain uncollected forever.

If you uncomment the last println! and run the program, Rust will try to print this cycle with a pointing to b pointing to a and so forth until it overflows the stack.
In this case, right after we create the reference cycle, the program ends. 
The consequences of this cycle aren’t very dire. 
However, if a more complex program allocated lots of memory in a cycle and held onto it for a long time, 
the program would use more memory than it needed and might overwhelm the system, causing it to run out of available memory.

Creating reference cycles is not easily done, but it’s not impossible either. 
If you have RefCell<T> values that contain Rc<T> values or similar nested combinations of types with interior mutability and reference counting, 
you must ensure that you don’t create cycles; you can’t rely on Rust to catch them. 
Creating a reference cycle would be a logic bug in your program that you should use automated tests, code reviews, and other software development practices to minimize.

Another solution for avoiding reference cycles is reorganizing your data structures so that some references express ownership and some references don’t. 
As a result, you can have cycles made up of some ownership relationships and some non-ownership relationships, 
and only the ownership relationships affect whether or not a value can be dropped. 
In Listing 15-25, we always want Cons variants to own their list, so reorganizing the data structure isn’t possible. 
Let’s look at an example using graphs made up of parent nodes and child nodes to see when non-ownership relationships are an appropriate way to prevent reference cycles.
*/


//Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

/*
So far, we’ve demonstrated that calling Rc::clone increases the strong_count of an Rc<T> instance, and an Rc<T> instance is only cleaned up if its strong_count is 0. 
You can also create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade and passing a reference to the Rc<T>. 
When you call Rc::downgrade, you get a smart pointer of type Weak<T>. 
Instead of increasing the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count by 1. 
The Rc<T> type uses weak_count to keep track of how many Weak<T> references exist, similar to strong_count. 
The difference is the weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.

Strong references are how you can share ownership of an Rc<T> instance. 
Weak references don’t express an ownership relationship. 
They won’t cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.

Because the value that Weak<T> references might have been dropped, to do anything with the value that a Weak<T> is pointing to, you must make sure the value still exists. 
Do this by calling the upgrade method on a Weak<T> instance, which will return an Option<Rc<T>>. 
You’ll get a result of Some if the Rc<T> value has not been dropped yet and a result of None if the Rc<T> value has been dropped. 
Because upgrade returns an Option<Rc<T>>, Rust will ensure that the Some case and the None case are handled, and there won’t be an invalid pointer.

As an example, rather than using a list whose items know only about the next item, we’ll create a tree whose items know about their children items and their parent items.
check smart_pointers7
*/