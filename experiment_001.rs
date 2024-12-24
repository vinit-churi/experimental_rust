use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Start the guessing game");
    loop {
        println!("Enter your guess");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error in regriving the guess");
        // let guess : u32 = guess.trim().parse().expect("Please enter a valid number");
        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        println!("you have guessed the number = {}", guess);
        println!("the secret number was {}", secret_number);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Equal, You Win!");
                break;
            }
        }
    }
}