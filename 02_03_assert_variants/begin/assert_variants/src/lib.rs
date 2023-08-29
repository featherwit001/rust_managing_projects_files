pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn tests_with_assert () {
		assert! (add(2, 2) == 4)
	}

	#[test]
	fn test_with_equal() {
		assert_eq!(add(2,2), 4)
	}

	#[test]
	fn test_with_ne () {
		assert_ne!(add(2,2), 3)
	}
}
