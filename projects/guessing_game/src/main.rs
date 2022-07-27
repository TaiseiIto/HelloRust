use std::io::Write;

fn main() {
    let mut guess:String = String::new();
    println!("Guess the number!");
    print!("Please input your guess : ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed : {}", guess);
}

