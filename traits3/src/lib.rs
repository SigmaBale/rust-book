//Fixing the largest Function with Trait Bounds

//n the body of largest we wanted to compare two values of type T using the greater than (>) operator. 
//Because that operator is defined as a default method on the standard library trait std::cmp::PartialOrd, 
//we need to specify PartialOrd in the trait bounds for T so the largest function can work on slices of any type that we can compare.

//We don’t need to bring PartialOrd into scope because it’s in the prelude.
//Trying to compile that we would get another set of errors.
//The key line in this error is cannot move out of type [T], a non-copy slice.
//Types like i32 and char that have a known size can be stored on the stack, so they implement the Copy trait. 
//But when we made the largest function generic, it became possible for the list parameter to have types in it that don’t implement the Copy trait. 
//Consequently, we wouldn’t be able to move the value out of list[0] and into the largest variable, resulting in this error.
//To call this code with only those types that implement the Copy trait, we can add Copy to the trait bounds of T!
//It will compile as long as the types of the values in the slice that we pass into the function implement the PartialOrd and Copy traits

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//If we don’t want to restrict the largest function to the types that implement the Copy trait, 
//we could specify that T has the trait bound Clone instead of Copy.
//Then we could clone each value in the slice when we want the largest function to have ownership.
//Using the clone function means we’re potentially making more heap allocations in the case of types that own heap data like String, 
//and heap allocations can be slow if we’re working with large amounts of data.
//We could also implement largest by having the function return a reference to a T value in the slice. If we change the return type to &T instead of T, 
//thereby changing the body of the function to return a reference, we wouldn’t need the Clone or Copy trait bounds and we could avoid heap allocations.