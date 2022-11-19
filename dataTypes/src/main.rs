fn main() {
    // primitive data types
    //scalar types
    //scalar types represent a single value (not a collection of values) and are either integer, floating-point, boolean, or character
    //rust is statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it
    let x: i32 = 5;
    /*
    compiler defines x as a variable of type integer (i32 is a 32-bit signed integer)
    8-bit signed integer the range is -128 to 127 (inclusive) and the default is i8
    16-bit signed integer the range is -32,768 to 32,767 (inclusive) and the default is i16
    32-bit signed integer the raange is -2,147,483,648 to 2,147,483,647 (inclusive) and the default is i32
    64-bit signed integer the range is -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 (inclusive) and the default is i64
    */
    let x: u32 = 972;
    /*
    compiler defines x as a variable of type unsigned integer (u32 is a 32-bit unsigned integer)
    8-bit unsigned integer the range is 0 to 255 (inclusive) and the default is u8
    16-bit unsigned integer the range is 0 to 65,535 (inclusive) and the default is u16
    32-bit unsigned integer the range is 0 to 4,294,967,295 (inclusive) and the default is u32
    64-bit unsigned integer the range is 0 to 18,446,744,073,709,551,615 (inclusive) and the default is u64
    128-bit unsigned integer the range is 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455 (inclusive) and the default is u128
    */

    let x: f32 = 2.0; //compiler defines x as a variable of type float (f32 is a 32-bit floating point number) and the default is f64
    let x: f64 = 3.0; //compiler defines x as a variable of type float (f64 is a 64-bit floating point number)

    let x: bool = true; //compiler defines x as a variable of type boolean (bool is a boolean type) and the default is false

    let x: char = 'z'; //compiler defines x as a variable of type character (char is a character type) and the default is 'a' (single quotes)

    //compound types
    //compound types can group multiple values into one type
    //rust has two primitive compound types: tuples and arrays
    //tuples are a fixed length list of values of different types (each position in the tuple has a type) and are created using parentheses () and commas , to separate the values
    //tuples are immutable by default
    let tup: (i32, bool, char) = (500, true, 'z');
    // can use the mut keyword to make a tuple mutable
    //accessing tuple values
    println!("{}", tup.0); //prints 500
    println!("{}", tup.1); //prints true ..

    // make a tuple mutable
    let mut tup2: (i32, bool, char) = (500, true, 'z');
    tup2.0 = 600;
    println!("{}", tup2.0); //prints 600

    //destructuring a tuple
    let tup3: (i32, bool, char) = (500, true, 'z');
    let (x, y, z) = tup3;
    println!("{}", x); //prints 500

    //arrays are a fixed length list of values of the same type and are created using square brackets [] and commas , to separate the values
    //arrays are immutable by default
    let arr = [1, 2, 3, 4, 5];
    //accessing array values
    println!("{}", arr[0]); //prints 1
                            // can use the mut keyword to make an array mutable
                            // make an array mutable
    let mut arr2 = [1, 2, 3, 4, 5];
    arr2[0] = 6;
    println!("{}", arr2[0]); //prints 6

    //you cannot innitialize an array with a value and a length
    //let arr3 = [3; 5]; //this is not allowed
    //you can innitialize an array to an empty array
    //let arr4: [i32; 5] = []; //this is allowed

    //you can innitialize an array with a value and a length
    let arr5 = [3; 5]; //this is allowed
    println!("{}", arr5[0]); //prints 3

    // you cannot reassign a value to a different type
    //let x = 5;
    //x = "hello"; //this is not allowed
    // or
    //let x: u8 = 5;
    //let y: u16 = 6; //this is not allowed
}
