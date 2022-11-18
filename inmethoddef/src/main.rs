//In Method Definitions
//We can implement methods on structs and enums and use generic types in their definitions, too.

//Point<T> struct with a method named x implemented on it.
struct Point<T> {
    x: T,
    y: T,
}


//By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


//We can also specify constraints on generic types when defining methods on the type. 
//We could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}
//X1 and Y1 for the Point struct and X2 Y2 for the mixup method signature
//The method creates a new Point instance
//with the x value from the -- self Point struct (of type X1)
//and the y value from the -- passed-in Point struct (of type Y2).
impl<X1, Y1> Point2<X1, Y1> {
    //The generic parameters X2 and Y2 are declared after fn mixup, because they’re only relevant to the method.
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}



fn main() {
    
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
