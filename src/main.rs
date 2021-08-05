use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to my first Rust program!");
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100); // 1(include), 100(exclude), =100(include)
    println!("The secret number is {}", secret_number);

    // add number of try
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "You input {} which is not a number, please type a number",
                    guess
                );
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("You got it!!");
                break;
            }
        }
    }
}
