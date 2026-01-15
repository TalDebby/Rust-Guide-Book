use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guees the number!");
    let secret_number = rand::rng().random_range(1..100);
    let mut tries_count = 1;
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! ({tries_count} tries)");
                break;
            }
        }
        tries_count += 1
    }
}
