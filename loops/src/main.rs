use std::io;

fn main() {
    //Convert temperatures between Fahrenheit and Celsius.
    //32 F = 0 C

    //Create empty string, take input from user & assign value to the empty string
    println!("Chose your input, Fahrenheit or Celsius (F or C):");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Error");

    //Convert F/C value depending on input
    let out: i32 = {
            if inp.trim() == "F" || inp.trim() == "f" {
                loop {
                    println!("Enter Fahrenheit amount:  ");
                    let mut inp = String::new();
                    io::stdin().read_line(&mut inp).expect("Error");
                    let inp: i32 = match inp.trim().parse(){
                        Ok(x) => x,
                        Err(_) => continue,
                    };
                    println!("Celsius:");
                    break inp-32
                }
            }
            else if inp.trim() == "C" || inp.trim() == "c" {
                loop {
                    println!("Enter Celsius amount:  ");
                    let mut inp = String::new();
                    io::stdin().read_line(&mut inp).expect("Error");
                    let inp: i32 = match inp.trim().parse(){
                        Ok(x) => x,
                        Err(_) => continue,
                    };
                    println!("Fahrenheit:");
                    break inp+32
                }
            } else {
                println!("Error, input F or C to start conversion, restart the program!");
                0
            }
    };

    println!("{}", out);

}
