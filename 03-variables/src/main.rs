// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6; // This will cause a compile-time error because x is immutable
//     println!("The value of x is: {x}");

//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
// }

// Shadowing

// fn main() {
//     // let x = 5;

//     // let x = x + 1;

//     // {
//     //     let x = x * 2;
//     //     println!("The value of x in the inner scope is: {x}");
//     // }

//     // println!("The value of x is: {x}");

//     let spaces = "     ";
//     let spaces = spaces.len();
//     println!("The number of spaces is: {spaces}");
// }

// Data Types
/*
* When we converted a String to numeric type using parse, we must add a type annotation to let Rust know what type we want to convert to.
 */

// fn main() {
//     let guess: u8 = "255".parse().expect("Not a number!");
//     println!("The guess is: {guess}");
// }


// Floating Point Types

// fn main() {
//     let x  = 2.0; // f64 Default
//     let y: f32 = 3.0; // f32

//     println!("x: {x}, y: {y}");
// }

// Numeric Operations

// fn main() {
//     // adition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncate = -5 / 3; // Results in -1

//     let remainder = 43 % 5;

//     println!("sum: {sum}, \ndifference: {difference}, \nproduct: {product}, \nquotient: {quotient}, \ntruncate: {truncate}, \nremainder: {remainder}");
// }

// Boolean Type

/*
* The main way to use Boolean values is through conditionals, such as an
* if expression.
 */

// fn main() {
//     let t = true;
//     let f: bool = false; // with explicit type annotation
// }

// Character Type

// fn main() {
//     let c = 'z;
//     let z: char = 'Z'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// Compound Types
// Tuple Type

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     let (x, y, z) = tup; // Destructuring
//     // You can also access the elements of a tuple using a period followed by the index of the value you want to access.
//     // For example, tup.0 will give you the first element of the tuple.
//     // tup.0; // 500
//     println!("The value of y is: {y}");
// }


// Array Type

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     /*
//     * An array is not flexible as the vector type, though. Arrays are more
//     * useful when you know the number of elements will not need to change.
//     * For example, months in a year or days in a week.
//      */

//     let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

//     let a: [i32; 5] = [1, 2, 3, 4, 5]; // Type annotation

//     let a = [3; 5]; // [3, 3, 3, 3, 3] - All elements initialized to 3

//     let a = [1, 2, 3, 4, 5];

//     let first = a[0]; // Accessing the first element
//     let second = a[1]; // Accessing the second element

    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    // .read_line(&mut index)
    // .expect("Failed to read line");

    // let index: usize = index.trim().parse().expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");
// }

// Functions

fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_parameter(5);
    print_labeled_measurement(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

