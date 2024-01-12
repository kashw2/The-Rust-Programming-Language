use std::io;

fn main() {
    println!("Guessing game!");

    println!("Please input your number");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading user input");

    println!("Your guess was {input}");
}
