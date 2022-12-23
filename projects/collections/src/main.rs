fn count_words(text: &str) -> std::collections::HashMap<String, i32> {
	let mut word_map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
	for word in text.split_whitespace() {
		let count: &mut i32 = word_map.entry(word.to_string()).or_insert(0);
		*count += 1;
	}
	word_map
}

fn main() {
	let v0: Vec<i32> = Vec::new();
    println!("v0 = {:?}", v0);
	let v1: Vec<i32> = vec![1, 2, 3];
    println!("v1 = {:?}", v1);
	let mut v2: Vec<i32> = vec![1,2,3,4];
	v2.push(5);
	v2.push(6);
	v2.push(7);
	v2.push(8);
    println!("v2 = {:?}", v2);
	let third: &i32 = &v2[2];
	println!("third = {}", third);
	match v2.get(2) {
		Some(third) => println!("third = {}", third),
		None => println!("The third doesn't exist."),
	}
	for i in &v2 {
		println!("{}", i);
	}
	for i in &mut v2 {
		*i += 10;
	}
    println!("v2 = {:?}", v2);
	let s0: String = String::new();
	println!("s0 = \"{}\"", s0);
	let s1: String = "initial contents".to_string();
	println!("s1 = \"{}\"", s1);
	let s2: String = String::from("initial contents");
	println!("s2 = \"{}\"", s2);
	let mut s3: String = "foo".to_string();
	let s4: &str = "bar";
	println!("s3 = \"{}\"", s3);
	s3.push_str(s4);
	println!("s3 = \"{}\"", s3);
	let c0: char = 'l';
	s3.push(c0);
	println!("s3 = \"{}\"", s3);
	println!("s4 = \"{}\"", s4);
	let s1: String = s1 + &s2[..];
	println!("s1 = \"{}\"", s1);
	let s5: &str = "tic";
	let s6: &str = "tac";
	let s7: &str = "toe";
	let s8: String = s5.to_string() + "-" + s6 + "-" + s7;
	println!("s8 = \"{}\"", s8);
	let s9: String = format!("{}-{}-{}", s5, s6, s7);
	println!("s9 = \"{}\"", s9);
	for c in "hello".chars() {
		println!("{}", c);
	}
	for c in "hello".bytes() {
		println!("{}", c);
	}
	let mut scores: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
	scores.insert("Blue".to_string(), 10);
	scores.insert("Yellow".to_string(), 50);
	println!("scores = {:#?}", scores);
	let teams: Vec<String> = vec!["Blue".to_string(), "Yellow".to_string()];
	println!("teams = {:#?}", teams);
	let initial_scores: Vec<i32> = vec![10, 50];
	println!("initial_scores = {:#?}", initial_scores);
	let scores: std::collections::HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect();
	println!("scores = {:#?}", scores);
	println!("scores.get(&\"Blue\".to_string()) = {:?}", scores.get(&"Blue".to_string()));
	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}
	let mut scores: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
	scores.insert("Blue".to_string(), 10);
	println!("scores = {:#?}", scores);
	scores.insert("Blue".to_string(), 25);
	println!("scores = {:#?}", scores);
	scores.entry("Yellow".to_string()).or_insert(50);
	scores.entry("Blue".to_string()).or_insert(50);
	println!("scores = {:#?}", scores);
	println!("count_words(\"hello world wonderful world\") = {:#?}", count_words("hello world wonderful world"));
}

