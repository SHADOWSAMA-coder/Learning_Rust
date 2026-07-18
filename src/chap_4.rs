//Memory mangement in rust is taken care of through a system of ownership with a set of rules that the compiler checks.
//none of the features of ownership will slow down your program whiile it's running.

//Before moving forward we must understand the Stack and the Heap

//The Stack stores values in the order they come in and removes them in opposite order.
//All data pushed onto the stack must be of known size. Data with unkiwn size must be stored in the heap instead.

//The memory allocater finds a spot big enough and returns a pointer to that location.
//Because the pointer is of known size we can store it in the stack.

//Pushing to the stack is faster than allocating in the heap.
//Since working with data that is close to each other is usually better for a processer.

//When our code calls a function, the values passsed including pointers to data on heap, local variables get pushed onto the stack.
//When it is done it gets popped off.

//Ownership rules.
//Rule no.1 - Each value in Rust has an owner.
//Rule no.2 - There can only be one owner at a time.
//Rule no.3 - When the owner goes out of scope, the value will be dropped.

fn main() {
    let mut s = String::from("hello");
    s.push_str(", World!");
    //note that string literals are immutable by default but using from we can make it allocated on the heap.

    //consider this 
    let x = 5;
    let y = x;
    //this is perfectly fine code indicating y being set the copied value of x
    //here both x and y continue to live as long as it remains in scope.
    //Since the data is stored on the stack.
    //Rust has a special trait called the Copy Trait implemented on the types that are stored in stack.
    //If a type implements the Copy trait variables that use it do not move rather are trivially copied.


    //now lets take a look at this
    let a = String::from("nigga");
    let b = a;
    //this copies the ointer address instead of the data in the heap hence both these variables now point to the same place on the heap.
    //Therefore there can be double free error since when both go out of scope both try to free the same address.
    //Hence rust considers a to not exist after let b =a; to avoid this nuiance
    //This is exactly the move operation of Rust instead of a shallow copy.

    //now look at this
    let mut d = String::from("hi");
    d = String::from("hey");
    //Rust calls drop since we are assigning a new value to a existing variable it calls drop

    //If you truly want to copy the heap data use clone

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

    //Hence using referneces make much more sense in this context when we want to access the variable without worrying about changing it.
    let mut s = String::from("hello");
    let r1 = &mut s;
    //This is a mutable reference 
    //REMEMBER: There can only exist a single mutable refernce to a value or variable.
    // the reason to do so is to prevent data races.
    //These happen when,
    //1.Two or more pointers access the same data at the same time.
    //2.At least one of the pointers is being used to write to the data.
    //There's no mechanism being used to synchronize access to the data.


}// Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

//Write a function that takes a string of words seperated by spaces and return sthe first word.
fn first_word(s:&String) -> &str{
    let bytes = s.as_bytes();//converts it to bytes 
    for (i,&item) in bytes.iter().enumerate() {//creates an iterator over which we enumerate
        if item == b' ' {
            return &s[0..i];
        }
    }
    s.len()
}