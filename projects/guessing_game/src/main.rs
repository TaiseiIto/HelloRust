use rand::Rng;
use std::io::Write;

fn main() {
    let mut guess:String = String::new();
    let secret_number:u8 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {}", secret_number);
    loop {
        println!("Guess the number!");
        print!("Please input your guess : ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess:u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        println!("You guessed : {}", guess);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

