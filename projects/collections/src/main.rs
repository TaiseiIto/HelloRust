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
}

