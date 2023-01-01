fn main() {
	let mut num: i32 = 5;
	let immutable_pointer = &num as *const i32;
	let mutable_pointer = &mut num as *mut i32;
	unsafe {
		println!("num = {}", *immutable_pointer);
		println!("num = {}", *mutable_pointer);
	}
}

