//We will learn error handling in this chapter
//Rust groups errors into two major categories recoverable and unrecoverable errors.

//for a recoverble error such as  a file not found error we just report the problem to the user.
//for an unrecoverable error like a bug accessing an out of bounds index we would like to immedieatly stop the program.

//Rust does not have an exception.
//Insteaf it has Result<T,E> for recoverable errors and panic! that stops execution.

//There are two ways to cause a panic in practice:
//1.by taking an action that causes our code to panic 
//2.explicitly calling the panic! macro

//By default when a panic occurs the program starts unwinding meaning rust walks back up the stckand cleans up the data from each function it encounters.
//However we can immediatley abort without cleaning up as well
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self,Read};
fn main() {
    panic!("crash and gay");
    //consider this 
    let mut v = vec![1,2,3];
    v[99];//This will also panic 
    // In languages like C++ this would be undefined behaviour caused due to buffer overread 

    //Not all processes require your program to stop entirely 
    //example being the creation of a new file if it does not exist.
    //In that case we can use Result<T,E> where T is the type of value returned in case of success, and E the type of the error.
    let greet = File::open("hello.txt");
    //The return type of this is a result where T is std::fs::File and E is std::io::Error
    //Now we can look into it deeply using the match statements.
    let greet_file = match greet {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc)=>fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file {error:?}");
            }
        },
    };
    //The above code looks to repetitive using match we can use closures and the unwrap_or_else method

    let hi = File::open("hi.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hi.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn read_username() -> Result<String, io::Error> {
    let user = File::open("hello.txt");

    let mut username_file = match user {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    //Instead of this match statement we could use ? after the file::open statement.

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
    //Also ? only works on result types for the values you call it on.
    
}