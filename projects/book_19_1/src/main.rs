extern "C" {
	fn abs(i: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;

fn main() {
	let mut num: i32 = 5;
	let immutable_pointer = &num as *const i32;
	let mutable_pointer = &mut num as *mut i32;
	unsafe {
		println!("num = {}", *immutable_pointer);
		*mutable_pointer += 1;
		println!("num = {}", *immutable_pointer);
	}
	let mut v = [1, 2, 3, 4, 5, 6];
	let (a, b) = split_at_mut(&mut v[..], 3);
	println!("a = {:?}", a);
	println!("b = {:?}", b);
	unsafe {
		println!("abs(3) = {}", abs(3));
		println!("abs(-3) = {}", abs(-3));
	}
	println!("{}", HELLO_WORLD);
	unsafe {
		COUNTER += 1;
		println!("COUNTER = {}", COUNTER);
	}
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
	let len: usize = slice.len();
	let ptr: *mut i32 = slice.as_mut_ptr();
	assert!(mid <= len);
	unsafe {
		(
			std::slice::from_raw_parts_mut(ptr, mid),
			std::slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
		)
	}
}

