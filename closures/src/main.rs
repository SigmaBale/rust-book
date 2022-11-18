use closures::{generate_workout};
fn main() {
    let intensity = 10;
    let random_number = 5;
    generate_workout(intensity, random_number);


    //example
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
    //Here, even though x is not one of the parameters of equal_to_x, the equal_to_x closure 
    //is allowed to use the x variable that’s defined in the same scope that equal_to_x is defined in.
    //We can’t do the same with functions!

    //If you want to force the closure to take ownership of the values it uses in the environment, you can use the move keyword before the parameter list. 
    //This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread.
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

    //move closures may still implement Fn or FnMut, even though they capture variables by move. 
    //This is because the traits implemented by a closure type are determined by what the closure does with captured values, 
    //not how it captures them. The move keyword only specifies the latter.

}
