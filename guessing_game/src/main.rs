use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number (between 0 - 10)!");
    let secret_number = thread_rng().gen_range(0..10);

    loop {
        println!("Please input your guess");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                println!("you guessed {}", input);
            }
            Err(_) => continue,
        }

        let input = match input.trim_end().parse::<u32>() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match input.cmp(&secret_number) {
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => {
                println!("mm, that's the good stuff");
                break;
            }
        }
    }
}
