//Validating References with Lifetimes

//Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.
//Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
//Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We only must annotate types when multiple types are possible.
//In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.
//Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.
//Although we won’t cover lifetimes in their entirety in this chapter, we’ll discuss common ways you might encounter lifetime syntax so you can get comfortable with the concept.




//Preventing Dangling References with Lifetimes

//The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference. 
//Consider the program in Listing 10-17, which has an outer scope and an inner scope.

{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}//Listing 10-17
//Note: The examples in Listings 10-17, 10-18, and 10-24 declare variables without giving them an initial value, so the variable name exists in the outer scope.
//At first glance, this might appear to be in conflict with Rust’s having no null values. However, if we try to use a variable before giving it a value, we’ll get a compile-time error, 
//which shows that Rust indeed does not allow null values.

//Listing 10-17 we will get an error.
//The variable x doesn’t “live long enough.” The reason is that x will be out of scope when the inner scope ends.
//But r is still valid for the outer scope; because its scope is larger, we say that it “lives longer.”
//If Rust allowed this code to work, r would be referencing memory that was deallocated when x went out of scope, and anything we tried to do with r wouldn’t work correctly. 
//So how does Rust determine that this code is invalid? It uses a borrow checker.




//The Borrow Checker

//The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
//Listing 10-18 shows the same code as Listing 10-17 but with annotations showing the lifetimes of the variables.

{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}//Listing 10-18

//Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b. As you can see, the inner 'b block is much smaller than the outer 'a lifetime block. 
//At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b. 
//The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.


//Listing 10-19 fixes the code so it doesn’t have a dangling reference and compiles without any errors.
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
//Listing 10-19
//Here, x has the lifetime 'b, which in this case is larger than 'a. 
//This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.
