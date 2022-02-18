use std::io;

// #[macro_use]
// extern crate text_io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    // let (s, i, j): (String, i32, f32);
    // scan!("{} {} {}\n", s, i, j);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
