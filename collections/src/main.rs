fn main() {
    //creating a vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    //updating a vector
    let mut v = Vec::new();

    //adding elements
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    //acessing vector variables using indexing and get
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),

    }

    //borrowing, push
    let mut v = vec![1, 2, 3, 4, 5];

    //let first = &v[0];

    v.push(6);

    let sixth = &v[5];

    println!("The sixth element is: {}", sixth);

    //iteration using for loop
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    //iteration over mutable refrence
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{}", v[0]);
    
}
