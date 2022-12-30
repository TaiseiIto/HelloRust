struct CustomSmartPointer {
	data: String,
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data {}", self.data);
	}
}

fn main() {
	let c = CustomSmartPointer {
		data: "my stuff".to_string(),
	};
	let _d = CustomSmartPointer {
		data: "other stuff".to_string(),
	};
	drop(c);
	println!("CustomSmartPointers created.");
}

