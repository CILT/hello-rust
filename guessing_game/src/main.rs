use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Only debug
    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // read_line puts the user input into any string we pass to
        // it, but also returns an Enumeration value (Result).
        // Result can be Ok or Err.
        // Result has expect() method, which handles both cases.
        // If not handled, compiler throws:
        //   warning: unused `Result` that must be used
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Trim eliminates any whitespace at beginning or end
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
