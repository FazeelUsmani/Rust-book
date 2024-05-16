use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number: ");
    
    let secret_number = rand::thread_rng().gen_range(1..=10);
    // println!("The secret number is: {}", secret_number);
    
    let mut chances = 0;
    while chances <= 3 {
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");      // crashing on an error
    println!("You guessed: {guess}");

        // Shadowing guess variable with u32 datatype.
        // let guess: u32 = match guess.trim().parse()
        //                        .expect("Please type a number!");

        // expect is crashing on an error. So, let's handle using Ok, Err types enums.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,     // here _ is a catall value will catch all types of Exceptions.
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        chances += 1;
        if chances >= 3 {
            println!("You lost! The secret number was: {}", secret_number);
            break;
        }
    }
    
}
