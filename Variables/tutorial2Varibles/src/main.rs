fn main() {
    // This is a comment. Comments are ignored by the compiler.
    // Comments are used to explain code and to make it easier to read.
    // Comments are not compiled.

    //create comment over multiple lines
    // by using /* and */ to enclose the comment

    /* normal variables */
    // variables are immutable by default
    let x = 5; // compiler defines x as a variable of type integer
    println!("The value of x is: {}", x);
    //x = 6;
    //println!("The value of x is: {}", x); // compiler error: cannot assign twice to immutable variable

    let mut y = 5; // compiler defines y as a mutable variable of type integer
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);
    // warning: value assigned to `y` is never read
    // value did not need to be mutable (mut) because it changed but did need to change

    /* shadowing example */
    let z = 5;
    println!("The value of z is: {}", z);
    let z = 6;
    println!("The value of z is: {}", z);

    let a = 4;
    println!("The value of a is: {}", a);

    {
        let a = a - 2; // this uses the a from the outer scope (4)
        println!("The value of a is: {}", a);
        let a = 2; // this is a new variable a that shadows the previous variable a in the outer scope (the outer scope is the entire main function)
        println!("The value of a is: {}", a);
    }
    let a = a + 1;
    println!("The value of a is: {}", a);

    //changeing the type of a variable
    let b = 5;
    println!("The value of b is: {}", b); // compiler defines b as a variable of type integer
    let b = "hello"; // compiler defines b as a variable of type string reassigning b to a different type is allowed
    println!("The value of b is: {}", b);
    // if b was mutable, then the code would give an error

    /* constants */
    //constants are always immutable and must be annotated with the const keyword and the type of the value must be annotated as well (the type cannot be inferred) and constants can be declared in any scope, including the global scope
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("There are {} seconds in a minute", SECONDS_IN_MINUTE);
    //if you remove the type annotation, the compiler will give an error
    // you cannot change the value of a constant

    //you cannot reassign a constant to a different value
    /*
    const SECONDS_IN_MINUTE: u32 = 100;
    println!("There are {} seconds in a minute", SECONDS_IN_MINUTE);
    */
}
