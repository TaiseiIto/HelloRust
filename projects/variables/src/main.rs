fn main() {
    // 3.1
    // Immutable value
    let x: i32 = 5;
    // Shadowing
    let x: i32 = x + 1;
    // Inner scope
    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is : {}", x);
    }
    println!("The value of x in the outer scope is : {}", x);
    // Create variables with same name and different types
    let string: &str = "Hello, World!";
    println!("string = {}", string);
    let string: usize = string.len();
    println!("string = {}", string);
    // Constant value
    const CONSTANT_VALUE: u8 = 0x10;
    println!("CONSTANT_VALUE = {:#02x}", CONSTANT_VALUE);

    // 3.2
    // &str to u32
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess = {}", guess);
    // Double
    let sum: i32 = 5 + 10;
    let difference: f64 = 95.5 - 4.3;
    let product: i32 = 4 * 30;
    let quotient: f64 = 56.7 / 32.2;
    let floored: i32 = 2 / 3;
    let remainder: i32 = 43 % 5;
    println!("sum = {}", sum);
    println!("difference = {}", difference);
    println!("product = {}", product);
    println!("quotient = {}", quotient);
    println!("floored = {}", floored);
    println!("remainder = {}", remainder);
    let t: bool = true;
    let f: bool = false;
    println!("t = {}", t);
    println!("f = {}", f);
    let c: char = 'z';
    println!("c = {}", c);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);
    let (x, y, z): (i32, f64, u8) = tup;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0] = {}", a[0]);
    println!("a[1] = {}", a[1]);
    println!("a[2] = {}", a[2]);
    println!("a[3] = {}", a[3]);
    println!("a[4] = {}", a[4]);
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("months[0] = {}", months[0]);
    println!("months[1] = {}", months[1]);
    println!("months[2] = {}", months[2]);
    println!("months[3] = {}", months[3]);
    println!("months[4] = {}", months[4]);
    println!("months[5] = {}", months[5]);
    println!("months[6] = {}", months[6]);
    println!("months[7] = {}", months[7]);
    println!("months[8] = {}", months[8]);
    println!("months[9] = {}", months[9]);
    println!("months[10] = {}", months[10]);
    println!("months[11] = {}", months[11]);
}

