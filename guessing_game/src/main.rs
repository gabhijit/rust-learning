extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secret Number is {}", secret_number);

    loop {
        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {},
            Err(_) => {println!("Error in readline")} ,
        };

        println!("Guess is {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error in Guess a number! {:?}", err);
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Little!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Equal! You Win!!");
                break;
            },
        };
    }
}
