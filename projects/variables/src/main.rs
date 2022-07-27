fn main() {
    // 3.1
    // Immutable value
    let x = 5;
    // Shadowing
    let x = x + 1;
    // Inner scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {}", x);
    }
    println!("The value of x in the outer scope is : {}", x);
    // Create variables with same name and different types
    let string:&str = "Hello, World!";
    println!("string = {}", string);
    let string:usize = string.len();
    println!("string = {}", string);
    // Constant value
    const CONSTANT_VALUE:u8 = 0x10;
    println!("CONSTANT_VALUE = {:#02x}", CONSTANT_VALUE);

    // 3.2
    // &str to u32
    let guess:u32 = "42".parse().expect("Not a number!");
    println!("guess = {}", guess);
    // Double
    let sum:i32 = 5 + 10;
    let difference:f64 = 95.5 - 4.3;
    let product:i32 = 4 * 30;
    let quotient:f64 = 56.7 / 32.2;
    let floored:i32 = 2 / 3;
    let remainder:i32 = 43 % 5;
    println!("sum = {}", sum);
    println!("difference = {}", difference);
    println!("product = {}", product);
    println!("quotient = {}", quotient);
    println!("floored = {}", floored);
    println!("remainder = {}", remainder);
}

