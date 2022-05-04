use std::io;

fn main() {
    println!("Guessing the number!");

    println!("Please input your guess.");

    /* The :: syntax in the ::new line indicates that 
       new is an associated function of the String type */
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
