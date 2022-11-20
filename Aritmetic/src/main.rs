use std::io;

fn main() {
    //literal is a value that is hard coded into the source code
    let x: i64 = 12;
    let y: i8 = 10;

    //addition
    let z = x + (y as i64); // casting y to i64 type to match x type using 'as' keyword
    println!("z = {}", z);

    // user input ot integer
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input_int: i64 = input.trim().parse().unwrap();
    /*
    trim() removes the new line character from the input
    parse() converts the string to integer
    unwrap() returns the value
    */

    println!("You entered: {}", input_int + 2);
}
