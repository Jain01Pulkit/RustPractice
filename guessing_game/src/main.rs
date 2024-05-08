use rand::Rng;
use std::cmp::Ordering;
use std::io; //To obtain user input and then print the result as output: io library'
fn main() {
    println!("Guess the number!");

    //In rust by default variables are immutable. To make a variable mutable mut keyword is added
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        /*String is a string type provided by the standard library that is a growable,UTF-8 encoded bit of text.
        The :: syntax in the ::new line indicates that new is an associated function of the String type.
        An associated function is a function that’s implemented on a type, in this case String.
         */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        /*
        calling the stdin function from the io library and then calling read_line method to handle the input from the user.
        the & sign indicates that this argument is a reference, so that any other line of code can access this variable.
        */
        //Diff b/w shadowing and mutable is with mutable we cannot change the type of the variable.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //parse returns a result type which is Ok or Err.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
        println!("You guessed: {guess}");
        //{guess} will print the value stored in guess variable.
    }
}
