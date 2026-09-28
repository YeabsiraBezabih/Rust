use std::io;
use rand::RngExt;
use std::cmp::Ordering;



fn main() {
    let mut guess_count: u8 = 3;
    let secret_number = rand::rng().random_range(0..=100);
    let mut guess = String::new();
        
    
    println!("Rust Guessing Game\n\n");

    loop{
        guess.clear();

    
    println!("please enter your guess {} left", guess_count);

    println!("{}", secret_number);

    let stdin = io::stdin();
    let _ = stdin.read_line(&mut guess);
    

    let guess: u16 = match guess.trim().parse() {
           Ok(num) => num,
           Err(_) => {
               println!("please enter a valid number 0-100");
               continue;
           }, // Default fallback value
       };

    match guess.cmp(&secret_number) {
            Ordering::Greater => {
            println!("too high");
            guess_count -= 1;
                
            }
            Ordering::Less => {
            println!("too low");
            guess_count -= 1;
                
            }
            Ordering::Equal => {
                println!("you got it");
                break;
            },
    }
    if guess_count == 0 {
        println!("out of guesses");
        break;
    }
    
}
    
}
