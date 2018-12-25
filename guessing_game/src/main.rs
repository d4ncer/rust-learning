extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() -> () {
    println!("Guess the number between 0 & 9!");
    let secret_number = rand::thread_rng().gen_range(0, 10);

    loop {
        let mut guess = String::new();
        println!("Please input your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Yes!!");
                break;
            }
            Ordering::Greater => println!("Too large!"),
        }
    }
}
