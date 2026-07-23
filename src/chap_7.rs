//We will cover cwhat crates are in this chapter.

//Crate is the smallest amount of code that the compiler considers at a time. 
//Crates contain modules, which may be defined in other files that get compiled with the crate.

//There are two forms of a crate.

//Binary crate: programs we can compile to an executable that we can run, such as command line program or a server.
//Each must have a main function called main that defines what happens when the executable runs.

//Library Crate: They don;;t have main function and they don't compile as executable.
//They define functionality intended to be shared with multiple projects.

//A package is a bundle of one or more crates that provide a set of functionality.
//A package contains a cargo.toml file that describes how t build those crates.
//Cargo is a package that contains the binary crate fo rthe command line tool.
//A package must contain atleast one library crate 

//When compiling a crate, the compiler first looks in the crate root file.
//In the crate root file, we can declare new modules say using mod garden;
//The compiler then look for that modules code in,
//    -> Inline, within the curly braces that replace the semicolon following mod garden.
//    -> in the file src/garden.rs
//    -> In the file src/garden/mod.rs

