fn main() {
    let x = 5;
    //x = 6;
    //this code gives error since x has been declared as immutable.
    // we can change this by making x as a mutable variable.

    //mut can not be used with constants
    //you declare them with const instead of let
    //you can not set the constant to something at runtime it is fully a compile time declaration however, we can set expressions to it.
    const PI:f64 = 3.24;

    //we can shadow variables as well by using the same name again
    let x = x+1;

    // scope creation is done by creating curly braces and putting stuff in it.

    //we must add type annotations while using parse.
    let y:f64 = 0.2;
    let x:f64 = 0.1;
    println!("x+y = {}",x+y);
    //floating point arithmetic is done by the IEEE-754 standard.
    // we can group data as well using tuples and arrays.
    let tup:(i32,f64,char) = (900,9.876,'h');//must specify each type while declaring.
    //to get elements out of the tuple we can use pattern matching to destructure it.
    let (x,y,z) = tup;
    println!("the value of y is {y}");
    //this is destructuring.

    //we can also accesss it by using the period fllowed by the index we want to use.
    let var_1 = tup.0;
    let var_2 = tup.1;

    //another way to store compounded data is to use arrays.
    //Unlike tuples arrays, demand you to have the same type in it.
    let arr:[i32; 5] = [1,2,3,4,5];

    let new_arr = [3; 5];
    // this creates [3,3,3,3,3]

    //array accessing is done like this,
    let frst = new_arr[0];
    let second = new_arr[1];

    //if you try to access beyond the array length obviously it throws an error in runtime.
    //other languages like c++ provide you with invalid memory rather than throwing an error.

    //consider this
    let y = {
        let x = 3;
        x+1//as you can see this is an expression to which y binds to 
    };
    println!("The value of y is {y}");

    //Conditionals 
    if x > 5 {
        println!("{x} is greater than 5");
    }else {
        println!("{x} was not gretaer than 5");
    }

    //you can declare variables using the if expression 
    let number = if 5>4 {4} else {7};
    //Note:- The valus that have the potential to be results must have the same type in an if expression.

    //One use of loop is to retry an operation you know you might fail.
    let mut count = 0;
    let num = loop {
        count+=1;
        if count == 10 {
            break count*2;//Notice how this loop returns a value that the num variable can bind to.
            //Anything to be returned can be added after the break statement as shown.
        }
    };
    //Also please look up the documentation for learning more about loop labels.
    //Lets look at how to loop through a collection and using a range.
    let numarr = [1,2,3,4,5,6];
    for number in numarr{
        println!("HELLO {number}");
    }

    for i in (1..10){//number from 1 till 9
        println!("{i**2}");
    }

}

//To define new functions we make use of the fn keyword followed by the name and parameters.
//Rust does not care where you define your function, only that they should be in a scope the caller can see.
//You must declare the type of each parameter the function is using.

//An important distinction to understand is between statement and expression.
//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value.
//Functions definiitions are also statements

//Functions with return value must be declared with the type.
fn hello() -> String{
    "Hello"
}

