use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your Guess");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Faled to read line");
    
    println!("You guessed: {guess} ");
    
}
