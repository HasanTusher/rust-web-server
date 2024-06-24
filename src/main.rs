use std::io;
use std::io::BufReader;
use std::mem::forget;
use hello::ThreadPool;


fn main() {
    println!("Guess the number");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Error yay");

    println!("you pressed {guess}");
}

