use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing game!");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your number");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading user input");

        let input: u32 = match input.trim().parse() {
            Ok(number) => {
                println!("Your guess was {number}");
                number
            },
            Err(_) => continue
        };

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Your guess of {input} was too low"),
            Ordering::Greater => println!("Your guess of {input} was too high"),
            Ordering::Equal => {
                println!("Your guess of {input} was right");
                break;
            }
        }
    }
}
