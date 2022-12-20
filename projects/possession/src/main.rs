fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn first_word(s: &String) -> &str {
	let mut begin_index: usize = 0;
	let mut begin_index_found: bool = false;
	let mut end_index: usize = s.len();
	let mut end_index_found: bool = false;
	for (i, &byte) in s.as_bytes().iter().enumerate() {
		match (begin_index_found, end_index_found, byte == b' ') {
			(false, false, false) => {
				begin_index = i;
				begin_index_found = true;
			},
			(false, false, true ) => {},
			(false, true , false) => {},
			(false, true , true ) => {},
			(true , false, false) => {},
			(true , false, true ) => {
				end_index = i;
				end_index_found = true;
				break;
			},
			(true , true , false) => {},
			(true , true , true ) => {},
		}
	}
	match (begin_index_found, end_index_found) {
		(false, false) => &s[..0],
		(false, true ) => &s[..0],
		(true , false) => &s[begin_index..],
		(true , true ) => &s[begin_index..end_index],
	}
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
    let len: usize = calculate_length(&s1);
    println!("The length of \"{}\" is {}.", s1, len);
    let mut s: String = String::from("hello");
    {
        let r1: &mut String = &mut s;
        change(r1);
    }
    let r2: &mut String = &mut s;
    change(r2);
    println!("s = {}", s);
	let s: String = String::from("  hello,  world!  hey hoo!");
	println!("The first word is {}.", first_word(&s));
	println!("The second word is {}.", second_word(&s));
}

fn makes_copy(some_integer: i32) {
    println!("some_integer = {}", some_integer);
}

fn second_word(s: &String) -> &str {
	let mut word_counter: u32 = 0;
	let mut begin_index: usize = 0;
	let mut begin_index_found: bool = false;
	let mut end_index: usize = s.len();
	let mut end_index_found: bool = false;
	for (i, &byte) in s.as_bytes().iter().enumerate() {
		match (begin_index_found, end_index_found, byte == b' ') {
			(false, false, false) => {
				begin_index = i;
				begin_index_found = true;
			},
			(false, false, true ) => {},
			(false, true , false) => {},
			(false, true , true ) => {},
			(true , false, false) => {},
			(true , false, true ) => {
				end_index = i;
				end_index_found = true;
				if word_counter == 1 {
					break;
				} else {
					word_counter += 1;
					begin_index_found = false;
					end_index_found = false;
				}
			},
			(true , true , false) => {},
			(true , true , true ) => {},
		}
	}
	match (begin_index_found, end_index_found) {
		(false, false) => &s[..0],
		(false, true ) => &s[..0],
		(true , false) => &s[begin_index..],
		(true , true ) => &s[begin_index..end_index],
	}
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("some_string = {}", some_string);
}

