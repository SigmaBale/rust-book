fn main() {
    //Unrecoverable Errors with panic! macro
    //When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

    //By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. 
    //However, this walking back and cleanup is a lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting, 
    //which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. 
    //If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic 
    //by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file. 
    //For example, if you want to abort on panic in release mode, add this: 

    //[profile.release]
    //panic = 'abort'


    //panic! macro
    
    //panic!("crash and burn");



    //Using a panic! Backtrace
    let v = vec![1, 2, 3];

    v[99]; //program panics here 
    //A backtrace is a list of all the functions that have been called to get to this point. 
    //Let’s try getting a backtrace by setting the RUST_BACKTRACE environment variable to any value except 0
    //cmd command for seting rust_backtrace to 1 = set RUST_BACKTRACE=1

}
