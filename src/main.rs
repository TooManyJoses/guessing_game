use std::io;

fn main() {
    println!("Welcome to the guess the number game!");

    println!("Please enter your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("An error has occured.");

    println!("You guessed: {}", guess);
}