use std::io;
use rand::Rng;

fn main() {
    println!("guess number");
    println!("input your guess...");

    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("you guessed: {guess}");
    println!("secret number: {secret}");
}
