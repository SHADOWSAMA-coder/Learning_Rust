// rust's  standard library includes a number of very useful data structures known as collections.
//We will discuss the usage of vector string and hashmap.

//Vector(Vec<T>)
//The elements are stored contigously in memory fo rfast access.

use std::collections::HashMap;

fn main(){
    let mut v:Vec<i32> = Vec::new();
    //This is how we create vectors.

    let mut v = vec![1,2,3]; //This is to initialize a vector without type annotation.

    //To update a vector we have,
    //push
    v.push(5);
    v.push(6);

    //To read a element we can do this in two ways.
    let thrid: &i32 = &v[2];
    println!("third element is {third}");

    let first: Option<&i32> = v.get(0);
    match first {
        Some(first) => println!("third is {first}"),
        None => println!("no number"),
    }
    //Oh and by the way you can not have a mutable reference and a immutable refernce in the same scope.
    //This is to stop data races which are common in other programming languages

    //What if we add an element to a vector it may need to resize itself then the compiler must allocate new bigger memory block somewhere else.
    //This causes to free the already existing memory and copy it elsewhere mamking the pointer point to random junk.

    for i in &mut v {
        *i += 50; //You need to derefrence here unlike cpp 
        //in cpp there is implicit dereferencing everywhere but only in specialized scenarios in rust.
    }

    let mut a = vec![1,2,3,4,5];

    //Lets say you want to remove or add elements while looping through and checking a condition then,

    //for adding an element
    //case1: use a temporary vector
    let mut elements_to_add = Vec::new();
    for item in &a {
        if *item  == 2 {
            elements_to_add.push(99);
        }
    }
    a.extend(elements_to_add);
    //case2: inserting an element
    a.insert(1,100);//inserts at index 1 and the value next to it.Takes O(n) time.

    //case3 : extending using an iterable
    a.extend(vec![9,8,7,6]);
    a.extend(6..9)

    //case 4: appending
    let mut a2 = vec![11,22,33,44,55];
    a.append(&mut a2);

    //for removing an element
    //case 1:filtering elements using retain
    a.retain(|item| item%2 == 0);

    //case2: removing at a index and returning that value
    let removed = a.remove(1);//This takes O(n) time to happen.

    //case3: an alternative to remove it takes the last element and puts it in place of the removed elements.
    let swap_removed = a.swap_remove(1);

    //case4: shortening the vector to a certain length
    a.truncate(1);

    //case5: clearing it out
    a.clear()

    //Vectors can only store values of the same type which can be inconvenient.
    //Instead we can create an enum that stores all these differnet values as its variants

    enum Row{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        row::Int(3),
        row::Float(5.6),
        row::Text(String::from("hello")),
    ]
    //Now we can store different values in the same vector.

    //Now we discuss about another type the String type.

    //Defining Strings
    //Rust has only one string type which is str usually seen as &str. 
    //string slices are references to some UTF-8 encoded string data elsewhere.
    //The reference can be either to the heap, stack or the direct binary of program(string literals).


    //Creating a new String.
    let mut s = String::new();//creates an empty string
    let data = "initial data";
    let s = data.to_string();//to convert already existing literals to Strings.
    //Note: to_string is aapplicable to any type having the Display trait

    let s = String::from("Initial contents");//works the same as to_string

    //Updating a String
    //You can use the + operator or the format! macro unlike the vector to concatenate String values.
    
    //We can grow a string using the push_str method to append a string slice
    //push_str doesn't take ownership of the string slice being appended 
    //we can use push to add a charcter to a string
    let mut c = String::from("foo");
    c.push_str("bar");

    //concatenating with + or format!
    let s1 = String::from("hello");
    let s2 = String::from(" ,World.");
    let s3 = s1 + &s2;
    //Note: we can not use s1 here again since it has been passed by value hence ownership is moved into additon function.
    //The add function takes in &String and a String to concatenate.
    //add uses a deref coercion
    //Secondly, we see that add takes ownership of self meaning s1 will be moved into the add call.

    //We can use the format macro to combine strings in a more efficient way.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    //The format macro works like println but instead of printing an output to the screen it returns a String.

    //Rust strings don't support indexing becuase a string ia vector of charcters that are utf-8 encoded.
    //Meaning there are charcters which require more than 1 byte to be stored.
    //Hence we don't the retrn type whether it will be a byte value, a character, a grapheme cluster, or a string slice.

    //However we can use a range to make string slices using byte values.
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    //since each character is of 2 bytes meaning s will be Зд.


    //Iterating over Strings
    //Best way to iterate over strings is to be explicit whetehr you want charcters or bytes.

    //For individual unicode scalar values we use chars method to seperate and return the type char.
    for c in "hello".chars() {
        println!("{c}");
    }

    //Alternatively for using bytes we can use the bytes method.
    for b in "hello".bytes() {
        println!("{b}");
    }

    //NOW LETS LOOK AT HASHMAPS
    //HashMap<K,V> stores a mapping of keys type K to values of type V using a hashing function. 

    //creating a new hashmap
    let mut scores = hashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    //This allows insertion of a key value pair.

    //HashMaps are homogeneous meaning all the keys have to be of the same type and all the keys must be of the same type.
    let team = String::from("Blue");
    let score = scores.get(&team).copied().unwrap_or(0);
    //score will have the value associated with Blue team and the result will be 10.
    //get returns an Option<&V> if there is no value to that key get returns None.

    //This program handles the Option by calling the copied to get an Option<i32> rather than an Option<&i32>.
    //Then unwrap_or to set score to zero if scores doesn't have an entry for the key.

    for (key,value) in &scores{
        println!("{key}: {value}");
        //This will print each pair in arbitrary order.
    }

    //For types implemeting Copy trait values are copied into the hashmap.
    //For owned values like String the values will be moved 

    //Each unique key can only have one value associated with it at a time

    //Updating a HashMap value.

    //Overwriting a Value.
    //If we insert a key again then the value gets overwritten.

    //Adding a Key and Value Only if a Key isn't present
    //We use entry that returns an Entry enum that represents a value might exist or not.

    scores.entry(String::from("Red")).or_insert(50);
    //The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that exists.
    //If not insert and returns a mutable reference to the value

    //Updating a value based on the old value.
    let text  = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count+=1;
    }
    //The split_whitespace method returns an iterator over subslices seperated by whitespace.

    //HashMap uses a hash function called SipHash
}