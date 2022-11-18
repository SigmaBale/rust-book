fn main() {
    //match pattern
    /*
    Formally, match expressions are defined as the keyword match, a value to match on, 
    and one or more match arms that consist of a pattern and an expression to run if the value matches that arm’s pattern, 
    like this:

        match VALUE {
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,
        }

    */


    //combination of if let, else if, else if let and else
    //here Some(color), Ok(age) are patterns we are matching our value against
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    

    //while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }


    //for loops
    //In a for loop, the pattern is the value that directly follows the keyword for, so in for x in y the x is the pattern.
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }


    //let Statements
    let x = 5;
    //Throughout this book, we’ve used let like this hundreds of times, and although you might not have realized it, 
    //you were using patterns! More formally, a let statement looks like this:
    //let PATTERN = EXPRESSION;

    
    //Multiple Patterns
    //In match expressions, you can match multiple patterns using the | syntax, which means or.
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //Matching Ranges of Values with ..=
    //The ..= syntax allows us to match to an inclusive range of values.
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    struct Point { 
        x: i32, 
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    

    //Match guard
    //A match guard is an additional if condition specified after the pattern in a match arm that must also match, along with the pattern matching, for that arm to be chosen.
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }


    //@ Bindings
    //The at operator (@) lets us create a variable that holds a value at the same time we’re testing that value to see whether it matches a pattern.
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
