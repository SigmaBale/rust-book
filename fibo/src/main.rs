use std::io;

fn main() {
    
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error");

    let input = input.trim();

    let input = {
        match input.parse::<u32>() {
            Ok(x) => x,
            Err(_) => 0,
        }
    };

    let mut prev: u32 = 0;
    let mut current: u32 = 1;

    for _ in 0..=input {
        let output = prev + current; 
        println!("{}", output);
        prev = current;
        current = output;
    }
}
