use std::io; //Standard input-output library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop{
        println!("\nPlease input your guess... ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Not high enough!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("CORRECT! Exiting now..");
                break
            }
        }
    }

    
}
