fn main() {
    let number: i32 = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number: i32 = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if number != 0 {
        println!("number was something other than zero");
    }
    let number: i32 = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 };
    println!("The value of number is : {}", number);
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining: i32 = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
    let mut number: i32 = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    for element in a {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

