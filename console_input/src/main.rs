// prelude is a module that contains common imports
// a crate is a compilation unit in Rust and has a root module
use std::io;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    // by default is you pass a vatiable to a function, you pass a copy of it
    // to fix that you can pass a reference to the variable by using & in front of it
    // this is called borrowing
    // you can also pass a mutable reference by using &mut
    // a refference is by default immutable
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You typed: {}", input);
    // input is a string, don't make input a integer
}
