use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
fn main() {
    let rand_num = rand::thread_rng().gen_range(1..10);

    println!("Guess a number!");
    loop {
        println!("----------------");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        print!("You Guessed: {guess}");
        let guess: i32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Cold"),
            Ordering::Greater => println!("Hot"),
            Ordering::Equal => {
                println!("You guessed {rand_num} and WON!");
                break;
            }
        }
    }
}
