#[derive(Debug)]
//Structs give you a way of grouping related fields and data.
//Enums give you a way of saying a value is one of the possible sets of values.

enum IpvAddr {
    V4(String),//this says that both have String values associated.
    V6(String),
    //We can use the name of the function to construct instances of enum.
    //We automatically get a constructor defined as a result of defining the enum.
    //Another advantage of using enums is we can give custom types to each type in a enum
}

enum ipvaddr{
    v4(u8,u8,u8,u8),
    v6(String),
    //This goes to show how we can put anything inside a enum.
    //Also the standard library has a enum for Ipv address so we can use that if we want.
}

enum Message {
    Quit,//Has no data
    Move{ x:i32,y:i32},//Has named fields, like a struct does
    Write(String),//Includes a string
    ChangeColor(i32,i32,i32),//Includes three i32 values

    //Notice how if we would have defined individual structs for all we couldn't define a function that works for all these variants of the enum which defining using the enum allows.
}

impl message{
    fn call(&self) {
        println!("hello");
    }
}

//The option<T>(it is a generic type parameter) enum
//Tells whether it could be something or may not be something.
//Rust doesnot have the null type hence we can encode this in a enum using the option enum.
//It has two variants Some and None

//IpvAddr is now a custom data type
//we can create instances of each of the two variiants like so,

//Now lets look at the usage of match control workflow

enum state {
    Andhra,
    Kerala,
    Tamil Nadu,
}
enum Coin {
    Nickel,
    Penny,
    Dime,
    Quarter(state),
}
fn main() {
    let four = IpvAddr::V4;
    let six = IpvAddr::V6;

    let m = Message::Write(String::from("hello"));
    m.call();//we can imolement this in any tyoe of variants of the enum.

    let some = Some(5);//The type of some here is Option<i32>
    let absent_number:Option<i32> = None;
    //we can not combine types such as i32 and Option<i32> unless we are sure of the presence.
    //That is one of the advantages of using an Option<T> enum rather than a null value.
    //In other words you need to convert Option<T> to T before doing any operation on it.

    //Consider this
    let config = Some(3u8);
    match config {
        Some(max) => println!("The maximum is {max}"),
        _ => (),
    }//Notice how we need to write the extra _ => ()
    //We can avoid that using th eif let syntax

    if let Some(max) => config {
        println!("The max is {max}");
    }//This is a more concise way of doing the same.
    
}

fn value_cents(coin:&Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State is {state:?}");
            25//As you can see we can now extract the internal state of the quarter using the match expression.
            //The same can be done using an Option<T> as well.
        },
        other => {
            println!("Gay");
            0
        },//This here is a catch all other case match arm making it exhaustive

        //Rust also has a catch all pattern _ that matches any value and does not bind to that value.
    }//Here we are using match statements to return values of individual coins.
    //The only condition regarding match is that you need to exhaust it especially in case of using Option<T>
}

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x {
        Some(num) => Some(num+1),
        None => None,
    }
}

fn route(ip_kind:IpvAddr) {
    println!("This prints any type.");
}
//We can call this function with either of the two types.

