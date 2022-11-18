fn main() {
    
    //As we did with structs, we can define enums to hold generic data types in their variants.
    enum Option<T> {
        Some(T),
        None,
    }


    //Enums can use multiple generic types as well.
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

}
