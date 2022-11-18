#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle { //Associated Functions
    fn area(&self) -> i32 {
        self.width*self.height
    }
    fn can_fit(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 5,
        height: 5
    };

    let rectangle2 = Rectangle {
        width: 2,
        height: 6
    };

    let rectangle3 = Rectangle {
        width: 4,
        height: 5
    };

    let rectangle4 = Rectangle::square(4);

    println!("a = {}", rectangle4.width);

    println!("Area of rectangle = {}", rectangle1.area());
    println!("{:#?}", rectangle1);
    println!("Can rect1 hold rect2? {} ", rectangle1.can_fit(&rectangle2));
    println!("Can rect1 hold rect3? {} ", rectangle1.can_fit(&rectangle3));
}


//In C and C++, two different operators are used for calling methods: you use . 
//if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object 
//and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to (*object).something().

//Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic 
//referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.