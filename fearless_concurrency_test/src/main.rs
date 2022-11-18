use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let result = Arc::new(Mutex::new(0));
    let result_thread1 = Arc::clone(&result);
    let result_thread2 = Arc::clone(&result);

    let handle1 = thread::spawn(move || {
        let mut number = result_thread1.lock().unwrap();
        *number = 3*3; 
        println!("Result = {}", *number);
    });

    handle1.join().unwrap();

    let handle2 = thread::spawn(move || {
        let mut number = result_thread2.lock().unwrap();
        *number *= 10;
        println!("Result = {}", *number);
    });
    
    handle2.join().unwrap();
}
