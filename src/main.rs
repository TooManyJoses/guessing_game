use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guess the number game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter your guess");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("An error has occured.");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("Correct");
                break;
            },
            Ordering::Greater => println!("Too high"),
        }
    }

}