use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("---Guess the number---");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("number is {}", secret_number);
    loop {
        println!("Enter your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess: u32 = guess.trim().parse().expect("Please typea number");
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Its less than {}", guess),
            Ordering::Less => println!("its more than {}", guess),
            Ordering::Equal => {
                println!("Correct!!");
                break;
            }
        }
    }
}
