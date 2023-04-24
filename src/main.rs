use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    // The kind of range expression we’re using here takes the form start..=end and
    // is inclusive on the lower and upper bounds, so we need to specify 1..=100
    // to request a number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
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
    
        // Rust allows us to shadow the previous value of guess with a new one.
        // Shadowing lets us reuse the guess variable name rather than forcing us to create
        // two unique variables, such as guess_str and guess, for example.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // The underscore, _, is a catchall value; in this example, we’re saying we
            // want to match all Err values, no matter what information they have inside them.
            Err(_) => continue,
        };
    
        println!("Your guess: {guess}");
    
            // A match expression is made up of arms. An arm consists of a pattern to match against,
        // and the code that should be run if the value given to match fits that arm’s pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
