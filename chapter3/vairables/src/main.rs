// 3.1. Immutable --------------------------------
// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     let x = 6;
//     println!("The value of x is: {x}");
// }

// 3.1. Constant --------------------------------
// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("{THREE_HOURS_IN_SECONDS}")
// }

// 3.1. Shadowing --------------------------------
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//     println!("The value of x is: {x}");
// }

fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();
}