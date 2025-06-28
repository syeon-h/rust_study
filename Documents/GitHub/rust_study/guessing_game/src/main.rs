use std::io; 
use std::cmp::Ordering;
use rand::Rng; 

fn main() {
    println!("Guess the number!");

    // Generate a random number ranges btw 1 to 100 
    let secret_number = rand::thread_rng().gen_range(1..=100); 

    // println!("The secret number is: {secret_number}"); 

    loop {
        println!("Please input your guess: "); 

        // storing values with variables (mutable) 
        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess) 
            .expect("Fail to read line"); 

        let guess: u32 = match guess.trim().parse() {
            // if parse successfully return a number 
            Ok(num) => num, 
            // handling invalid input: go to the next iteration of the loop 
            Err(_) => continue,
        }; 

        println!("You guessed: {guess}"); 

        // comparing guess to secret_number 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"), 
            Ordering::Greater => println!("Too big"), 
            Ordering::Equal => {  
                println!("You win"); 
                // makes the program exit the loop 
                break; 
            }
        }
    }
}

