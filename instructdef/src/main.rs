//We can also define structs to use a generic type parameter in one or more fields using the <> syntax.
//Defining a Point<T> struct to hold x and y coordinate values of any type.
struct Point<T> {
    x: T,
    y: T,
}



//To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters.
struct Point_2<T, U> {
    x: T,
    y: U,
}

fn main() {
    
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //If we create an instance of a Point<T> that has values of different types, as in Listing 10-7, our code won’t compile.
    //The fields x and y must be the same type because both have the same generic data type T.



    let both_integer = Point_2 { x: 5, y: 10 };
    let both_float = Point_2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point_2 { x: 5, y: 4.0 };

    //If you're finding you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces.
    
}
