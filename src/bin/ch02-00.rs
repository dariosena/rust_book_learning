use rand::Rng;
use std::cmp::Ordering;
use std::io;

/* Programming a Guessing Game */
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..11);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess.eq("quit") {
            println!("You quit!");
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(my_guess) => my_guess,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => println!("Too small!"),
        }
    }
}
