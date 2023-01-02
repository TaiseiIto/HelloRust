fn add_one(x: i32) -> i32 {
	x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
	f(f(arg))
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
	Box::new(|x| x + 1)
}

fn main() {
    println!("do_twice(add_one, 5) = {}", do_twice(add_one, 5));
	let numlist: Vec<i32> = vec![1, 2, 3];
	let strlist: Vec<String> = numlist.iter().map(ToString::to_string).collect::<Vec<String>>();
	println!("strlist = {:?}", strlist);
	println!("returns_closure()(0) = {}", returns_closure()(0));
}

