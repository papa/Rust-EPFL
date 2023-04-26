use std::fs::File;
use std::io::{self, Read};

fn main() {
    let v = vec![1,2,3];
    
    //v[99]; // code will panic
    //run with RUN_BACKTRACE=1 to see backtrace

    //return value is Result<T,E>
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    //trying to create a file if there is not one
    let greeting_file_result = File::open("hello.txt");

    //altervnative to writing these many matches is unwrap_or_else (chapter 13)
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    //returns the file, or panics if file can't be opened
    // let greeting_file_result = File::open("hello.txt").unwrap();

    //custom panic error message
    let file = File::open("hello.txt").expect("ERROR OPENING FILE");

    //read examples on when to panic and when not to do so
}
//? is shortcut for propagating errors,
//means if there is an error return it as functions result
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
