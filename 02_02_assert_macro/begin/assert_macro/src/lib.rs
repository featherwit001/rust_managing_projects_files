fn is_even(number: i32) -> bool {
	match number % 2 {
		0 => true,
		_ => false,
	}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() { 
		assert!(is_even(42));
        assert!(!is_even(21));
    }
}
