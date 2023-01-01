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

