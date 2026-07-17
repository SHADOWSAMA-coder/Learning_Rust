use std::io; //library to deal with input and output 

use rand::Rng;
//Rng is a trait that defines methods, random number generators implement. Therefore it must be in scope to use it.
//thread_rng is a random geneartor local to the current thread and is in the os. 
//gen_range creates a number in the given range.

use std::cmp::Ordering;
//Ordering type is another enum and has the variants Less Equal and Greater.



fn main(){
    loop{//creates an infinite loop
        println!("Enter your guess.");

        let secret_num = rand::thread_rng().gen_range(1,,=100);

        let mut guess = String::new(); //Creates an empty instance of String type.

        //let is used to declare a variable
        let x = 5;

        // mut indicates it being mutable
        //Rust variables are immutable by default
        let y = 4;//immutable
        let mut z = 0;//mutable

        //String is a growable(on heap in the memory) UTF-8 encoded bit of text.
        //::new where new is a associated function which are implemented for a particular type in this case String.

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
        //This returns a std::io::Stdin which is a type that represents a handle to the standard input.
        //read_line is then called on the handle and appended in the variable guess.
        //& indicates it being a reference which is a way to access one piece of data without needing to copy multiple times.
        //referneces are also immutable by default hence you need to give mut keyword before it as well.

        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,//create a temporary variable and pass it immeditaely to guess.
            Err(_)=>continue,// _ is a catch all value 
        }
        // we used shadowing to overwrite the previous variable value to the new one.
        // trim eliminates any white spaces at the beginning and the end.
        //parse is used to type cast from string to integer
        // the colon tells to annotate the type of the variable we are dealing with.

        //The output is a enumeration called Result, which is a type that can be inn one of multiple states.
        //Result's variants are Ok and Err. Ok is for when was succesful and Err for when it is not.

        println!("You guessed: {guess}");
        //{} acts as a placeholder for the value in the variable.

        println!("x = {x} and y+2 = {}",y+2);
        //you get it.

        match guess.cmp(&secret_num) { // cmp method compares any two values that are comparable
            // Then returns a Ordering Type enum which we use match to check its state.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("Yay! Magic number found."),
                break;//to exit out of the loop
            }
        }
    }

}