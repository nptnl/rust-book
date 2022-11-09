use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("game beginning.");
    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    println!("solution is {secret}");
    
    loop {
        println!("enter guess");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read");
        println!("you guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("{guess} is too small"),
            Ordering::Greater => println!("{guess} is too large"),
            Ordering::Equal => {
                println!("{guess} wins!");
                break;
            }
        }
    }
}
