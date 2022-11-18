use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //Recoverable errors with Result
    //Result enum is defined as having two variants, Ok and Err, as follows:
    //enum Result<T, E> {
    //    Ok(T),
    //    Err(E),
    //}
    //The T and E are generic type parameters
    //T represents the type of the value that will be returned in a success case within the Ok variant, 
    //and E represents the type of the error that will be returned in a failure case within the Err variant

    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(file_created) => file_created,
                Err(e) => panic!("Could not create the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        }
    };

    //This return type means the call to File::open might succeed and return a file handle that we can read from or write to. 
    //The function call also might fail: for example, the file might not exist, or we might not have permission to access the file.


}
