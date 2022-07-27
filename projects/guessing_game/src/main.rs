use rand::Rng;
use std::io::Write;

fn main() {
    let mut guess:String = String::new();
    let secret_number:i8 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {}", secret_number);
    println!("Guess the number!");
    print!("Please input your guess : ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed : {}", guess);
}

