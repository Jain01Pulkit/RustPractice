use std::io;

// fn main() {
//     let arr = [1, 2, 3, 4, 5];
//     let mut guess = String::new();
//     println!("Entere a number");
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Enter a valid number");
//     let guess: usize = guess.trim().parse().expect("Index was not a number");
//     let element: i32 = arr[guess];
//     println!("YOu enetred index is {guess} and value is {element}");
// }

fn five() -> i32 {
    5
}

fn main() {
    let newstring = String::from("Hello");
    let newstring2 = newstring.clone();
    println!("yo lo  {newstring}");
}
//cargo new create a new project
//cargo run build and run the project
//cargo build build the project
//cargo check just compile the project and does not make an executable binary file
