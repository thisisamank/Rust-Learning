use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("The secret number is {}", secret_number);
    loop {
        println!("Type a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read string");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too less"),
            Ordering::Equal => {
                println!("You guessed it right");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
