#[cfg(test)]
mod tests {
	#[test]
	fn iterator_demonstration() {
		let v = vec![1, 2, 3];
		let mut iter = v.iter();
		assert_eq!(iter.next(), Some(&1));
		assert_eq!(iter.next(), Some(&2));
		assert_eq!(iter.next(), Some(&3));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn iterator_sum() {
		assert_eq!(vec![1, 2, 3].iter().sum::<i32>(), 6);
	}
}

