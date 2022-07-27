fn main() {
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
}

