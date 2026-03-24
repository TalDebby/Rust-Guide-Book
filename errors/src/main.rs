use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;
use std::error::Error;



fn intentional_panic() {
    panic!("Something went wrong");
}

fn lib_panic() {
    let vec = vec![1, 2, 3];
    vec[99];
}

fn recoverable_errors() {
    let greeting_file_result = File::open("hello.txt");

    let gretting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => {
                        panic!("There was a problem creating the file: {:?}", e)
                    }
                },
                _ => {panic!("There was a problem opening the file: {:?}", error)}
            }
            
    };
}

fn unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();
}

fn expect() {
    // prefer to use expect over unwrap because it allows clear intention about the code that should always succeed
    let greeting_file = File::open("hello.txt").expect("hello.txt should be present");
}

fn error_propagation() -> Result<File, io::Error>{
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username_file),
        Err(e) => Err(e),
    }
}

fn propagate_error_question_mark() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

struct OurError {
    description: String,
}

impl From<io::Error> for OurError {
    fn from(err: io::Error) -> Self {
        OurError {
            description: err.to_string(),
        }
    }
}

fn propagate_diffrent_error_type() -> Result<String, OurError> {
    // The question mark isn't just a smiple match, it can also convert one type to the other using the From trait
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn chaning() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn question_mark_on_option(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()

}

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

