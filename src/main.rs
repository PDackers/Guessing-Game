use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number between 1 and 100.");
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess as a number:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
