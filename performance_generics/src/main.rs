//Performance of Code Using Generics
//Generic types won't make your run any slower than it would with concrete types.
//Rust accomplishes this by performing monomorphization of the code using generics at compile time.
//Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

fn main() {

    //Let’s look at how this works by using the standard library’s generic Option<T> enum
    let integer = Some(5);
    let float = Some(5.0);
    //When Rust compiles this code, it performs monomorphization. During that process, the compiler 
    //reads the values that have been used in Option<T> instances and identifies two kinds of 
    //Option<T>: one is i32 and the other is f64. As such, it expands the generic definition of Option<T> 
    //into Option_i32 and Option_f64, thereby replacing the generic definition with the specific ones.
    
}


//The monomorphized version of the code looks like the following:

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}