fn main() {
    
    //Generic Data Types
    //We use generics to create definitions for items like function signatures or structs, 
    //which we can then use with many different concrete data types.

    //In Function Definitions
    //When defining a function that uses generics, we place the generics in the signature of the function 
    //where we would usually specify the data types of the parameters and return value.

    //Two functions that both find the largest value in a slice. 
    //We'll then combine these into a single function that uses generics.

    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
    
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }
    
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
    
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    //let’s eliminate the duplication by introducing a generic type parameter in a single function.
    //To parameterize the types in a new single function, we need to name the type parameter, 
    //just as we do for the value parameters to a function. You can use any identifier as a type parameter name.
    //Short for “type,” T is the default choice of most Rust programmers.

    //When we use a type parameter name in a function signature, we have to declare the type parameter name before we use it.
    //To define the generic largest function, place type name declarations inside angle brackets, <>, between the name of the function and the parameter list.
    //Like this:
    //  largest<T>(list: &[T]) -> T {

    
    //combined largest function definition using the generic data type in its signature.
    
    fn largest<T>(input: &[T]) -> T {
        let mut largest = input[0];

        for &item in input{
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    //If we compile this code right now, we’ll get error
    //The note mentions std::cmp::PartialOrd, which is a trait.
    //This error states that the body of largest won’t work for all possible types that T could be.
    //Because we want to compare values of type T in the body, we can only use types whose values can be ordered. 
    //To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types
    //this will be fixed later

}
