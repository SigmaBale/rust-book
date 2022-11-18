pub struct ImportantExcerpt<'a> {
    part: &'a str,
}
//Lifetime Elision

//You’ve learned that every reference has a lifetime and that you need to specify lifetime parameters for functions or structs that use references.
//However, in Chapter 4 we had a function in Listing 4-9, shown again in Listing 10-26, that compiled without lifetime annotations.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}//Listing 10-26

//The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust, 
//this code wouldn’t have compiled because every reference needed an explicit lifetime.
//At that time, the function signature would have been written like this:
//fn first_word<'a>(s: &'a str) -> &'a str {
//After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotations over and over in particular situations.
//The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

//The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
//These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, 
//and if your code fits these cases, you don’t need to write the lifetimes explicitly.
//The elision rules don’t provide full inference.
//Instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations.

//Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
//The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations.
//The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.
// If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.
//These rules apply to fn definitions as well as impl blocks.

//RULES
//The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. 
//In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); 
//a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

//The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

//The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, 
//the lifetime of self is assigned to all output lifetime parameters.
//This third rule makes methods much nicer to read and write because fewer symbols are necessary.





//Lifetime Annotations in Method Definitions

//When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters.
//Where we declare and use the lifetime parameters depends on whether they’re related to the struct fields or the method parameters and return values.
//Lifetime names for struct fields always need to be declared after the impl keyword and then 
//used after the struct’s name, because those lifetimes are part of the struct’s type.
//In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent.
//In addition, the lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures.
//Let’s look at some examples using the struct named ImportantExcerpt that we defined in Listing 10-25.
//First, we’ll use a method named level whose only parameter is a reference to self and whose return value is an i32, which is not a reference to anything:
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}//The lifetime parameter declaration after impl and its use after the type name are required, 
//but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.

//Here is an example where the third lifetime elision rule applies:
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
//There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes.
//Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.






//The Static Lifetime

//One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program.
//All string literals have the 'static lifetime, which we can annotate as follows: (main.rs)