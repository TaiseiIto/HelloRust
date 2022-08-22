fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn main() {
    let mut s: String = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    let s1: String = String::from("hello");
    let s2: String = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    let s: String = String::from("hello");
    takes_ownership(s);
    let x: i32 = 5;
    makes_copy(x);
    println!("x = {}", x);
    let s1: String = gives_ownership();
    let s2: String = String::from("hello");
    let s3: String = takes_and_gives_back(s2);
    println!("s1 = {}", s1);
    println!("s3 = {}", s3);
    let s1: String = String::from("hello");
    let (s2, len): (String, usize) = calculate_length(s1);
    println!("The length of \"{}\" is {}.", s2, len);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer = {}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("some_string = {}", some_string);
}

