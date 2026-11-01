use std::io;  // This line brings input/output libraray into scope. 

use rand::prelude::*;

use std::cmp::Ordering; //The Ordering type is another 
// enum and has the variants Less, Greater, and Equal for compare two values. 

fn main(){
    println!("Guess the number!");  // println! is a macro that prints a string to the screen:
    
    let secret_number = rand::rng().random_range(1..=100); 

    println!("The secret number is: {secret_number}");

    loop {

     println!("Please input your guess."); 

    let mut guess = String::new(); 

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess : u32 = guess.trim().parse().expect("Please type a number"); // guess.trim() -> method is use will elimitate any whitespace
        //parse() -> method will converts a string to another type. Here, we use it to convert from a string to a number.
    
        
            println!("You guessed: {guess}");

    // compare two numbers
      match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win");
            break; 
        }
       }
    }
}