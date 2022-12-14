use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number! 1 - 100");

    let secret_number = rand::thread_rng().gen_range(1..= 100);

    let mut chances: u32 = 0;

    //println!("The secret number is: {secret_number}");
    loop{
        chances += 1;
        println!("Please input your guess:");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nYou guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("\nchances: {chances}");
    if chances >= 10{
        println!("you are a puppy")
    }else if chances >= 4 && chances < 10{
        println!("you are medium")
    }else if chances >= 3{
        println!("you are a master")
    }else if chances < 3{
        println!("YOU A MASTERPIECE!!!")
    }
}  
