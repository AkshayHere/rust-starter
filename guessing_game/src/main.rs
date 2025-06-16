use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");
    // let mut sample = "Custom";
    let mut guess = String::new();
    // println!("guess. {}::{}", guess, sample);

    let test = io::stdin()
        .read_line(&mut guess).is_ok();
    println!("test. {}", test);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
