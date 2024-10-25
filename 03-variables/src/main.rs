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

fn main() {
    let guess: u8 = "255".parse().expect("Not a number!");
    println!("The guess is: {guess}");
}
