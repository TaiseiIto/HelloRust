fn another_function(x: i32) {
    println!("The value of x is {}", x);
}

fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };
    println!("The value of y is {}", y);
    let x = five();
    println!("The value of x is {}", x);
    let x = plus_one(5);
    println!("The value of x is {}", x);
    let number: i32 = 3;
    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false.");
    }
    let number: i32 = 6;
    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3 and 2.");
    }
    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining: i32 = 10;
        loop {
            println!("remaining {}", remaining);
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
    let mut index = 0;
    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }
    for element in a {
        println!("the value is {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {}{}", value, unit_label);
}

