use std::io;  // This line brings input/output libraray into scope. 



fn main(){
    println!("Guess the number!");  // println! is a macro that prints a string to the screen:
    

    println!("Please input your guess."); 

    let mut guess = String::new(); 

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

}