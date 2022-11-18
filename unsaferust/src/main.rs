//Using extern Functions to Call External Code
/*
Sometimes, your Rust code might need to interact with code written in another language. For this, Rust has a keyword, 
extern, that facilitates the creation and use of a Foreign Function Interface (FFI).
An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.
Functions declared within extern blocks are always unsafe to call from Rust code.
*/
extern "C" {
    fn abs(input: i32) -> i32;
}
//The "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level. 
//The "C" ABI is the most common and follows the C programming language’s ABI.

//Accessing or Modifying a Mutable Static Variable
//In Rust, global variables are called static variables.
static HELLO_WORLD: &str = "Hello, world!";
/*
Static variables can only store references with the 'static lifetime, which means the Rust compiler 
can figure out the lifetime and we aren’t required to annotate it explicitly. 
Accessing an immutable static variable is safe.
Constants and immutable static variables might seem similar, but a subtle difference is that values in a static variable have a fixed address in memory.
Using the value will always access the same data. 
Constants, on the other hand, are allowed to duplicate their data whenever they’re used.
Another difference between constants and static variables is that static variables can be mutable.
*/
static mut _COUNTER: u32 = 0;

fn _add_to_count(inc: u32) {
    unsafe {
        _COUNTER += inc;
    }
}

fn main() {

    //extern
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    //global variable
    println!("name is: {}", HELLO_WORLD);

    let mut num = 5;

    //Immutable and a mutable raw pointer from references.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    //Notice that we don’t include the unsafe keyword in this code. 
    //We can create raw pointers in safe code; we just can’t dereference raw pointers outside an unsafe block, as you’ll see in a bit.


    //Creating a raw pointer to an arbitrary memory address
    let address = 0x012345usize;
    let _r = address as *const i32;


    //We use the dereference operator * on a raw pointer from example at the start, this requires an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }


    //Calling an Unsafe Function or Method
    //Here is an unsafe function named dangerous that doesn’t do anything in its body
    unsafe fn dangerous() {}

    //We must call the dangerous function within a separate unsafe block. If we try to call dangerous without the unsafe block, we’ll get an error.
    unsafe {
        dangerous();
    }


    //Creating a Safe Abstraction over Unsafe Code
    //Wrapping unsafe code in a safe function is a common abstraction.
    //As an example, let’s study a function from the standard library, split_at_mut, that requires some unsafe code and explore how we might implement it.
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (a, b) = v.split_at_mut(3);

  

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);


}

//unsafe block, a raw pointer, and some calls to unsafe functions to make the implementation of split_at_mut work
use std::{slice, vec};

fn _split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            //The function slice::from_raw_parts_mut is unsafe because it takes a raw pointer and must trust that this pointer is valid.
            slice::from_raw_parts_mut(ptr, mid),
            //The add method on raw pointers is also unsafe, because it must trust that the offset location is also a valid pointer.
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
//We’ve created a safe abstraction to the unsafe code with an implementation of the function that uses unsafe code in a safe way, 
//because it creates only valid pointers from the data this function has access to.