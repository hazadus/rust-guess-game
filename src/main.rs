use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");
    println!("Please input your guess: ");

    // By default, variables are immutable.
    // `mut` keyword makes variable mutable.
    let mut guess = String::new();

    // Adds to a string without overwriting it.
    // `&` indicates that this argument is a reference.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    print!("Your guess: {guess}");
    print!("Secret is: {secret_number}")
}
