#![allow(unused, unused_variables, dead_code)]
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::panic;
pub fn start() {
    // Errors in rust are of two types: Recoverable and Unrecoverable. There are NO exceptions in rust.

    // Unrecoverable:-
    // panic!("Crash and burn!");
    // The above line will crash the program, with the message inside the `panic!` macro. We can call it when we want to stop the program and we know that it is an unrecoverable error, like accessing indices of a vector beyond its `length-1`.

    // Recoverable:- They are handled with the Result enum in rust.
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}")
            }
        },
    };

    // using closures(will be discussed more, later) to use less "matches"
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // If the Result value is the Ok variant, "unwrap" will return the value inside the Ok. If the Result is the Err variant, "unwrap" will call the panic! macro for us. Similarly, the "expect" method lets us also choose the panic! error message.

    let username = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                _ => panic!("Problems opening the file"),
            },
            _ => panic!("Problems opening the file"),
        },
    };
    // somewhat simpler way written down below
    let username = match File::open("hello.txt") {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                match File::create("hello.txt") {
                    Ok(file) => file,
                    _ => panic!("Couldn't open the file."),
                }
            } else {
                panic!("Couldn't open the file.")
            }
        }
    };
    // using closures
    let username = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|err| panic!("Couldn't open the file: {err:?}"))
        } else {
            panic!("Couldn't open the file.")
        }
    });
}
// Using Result to "propagate" errors to the calling function, so that the caller can decide what to do with the error
fn read_username_from_file() -> Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// better and concise way to return the Ok/Err value from the function
fn read_username_from_file_concise() -> Result<String, std::io::Error> {
    let mut username_file = File::open("hello.txt")?; //  If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so that the error value gets propagated to the calling code.
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // There is a difference between what the match expression from above does and what the "?" operator does: Error values that have the "?" operator called on them go through the "from" function, defined in the "From" trait in the standard library, which is used to convert values from one type into another. When the "?" operator calls the "from" function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

    // For example, we could change the read_username_from_file function in the above function to return a custom error type named OurError that we define. If we also define impl From<io::Error> for OurError to construct an instance of OurError from an io::Error, then the ? operator calls in the body of read_username_from_file will call from and convert the error types without needing to add any more code to the function.

    // More concise way will be to "chain" the methods together using "?" and the dot operator
    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)
}

fn read_username_from_file_more_concise() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Using the ? operator
fn some_fn() {
    // Its return type is the unit type()
    // let greeting_file = File::open("hello.txt")?;
    // the above line is illegal! Because the '?' works only and only when the return type of the function matches Result/Option type

    // So if the return type was Result/Option type depending on the return value of the thing with which we are using '?' then it would have worked just like the following example below
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// * Also, we can return errors from fn main() -> Result<(),  E>.
