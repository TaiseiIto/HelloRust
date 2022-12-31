#[derive(Debug)]
pub struct AveragedCollection {
	list: Vec<i32>,
	average: f64,
}

impl AveragedCollection {
	fn update_average(&mut self) {
		self.average = self.list.iter().sum::<i32>() as f64 / self.list.len() as f64;
	}

	pub fn new() -> Self {
		Self {
			list: vec![],
			average: 0.0,
		}
	}

	pub fn average(&self) -> f64 {
		self.average
	}

	pub fn add(&mut self, value: i32) {
		self.list.push(value);
		self.update_average();
	}

	pub fn remove(&mut self) -> Option<i32> {
		let result = self.list.pop();
		if let Some(_) = result {
			self.update_average();
		}
		result
	}
}

