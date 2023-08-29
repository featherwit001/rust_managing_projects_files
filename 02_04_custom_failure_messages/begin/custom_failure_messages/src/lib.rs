pub fn add(left: usize, _right: usize) -> usize {
    left + 5
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn tests_with_assert () {
		let result = add(2, 2);
		assert! (result == 4, "Expect 4; result was {}", result);
	}

	#[test]
	fn test_with_equal() {
		let result = add(2, 2);
		assert_eq!(result, 4, "Expect 4; result was {}", result);
	}

	#[test]
	fn test_with_ne () {
		let result = add(2, 2);
		assert_ne!(result, 3, "Expect not 3, result was {}", result);
	}
}
