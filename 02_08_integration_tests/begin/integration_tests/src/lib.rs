
pub mod add_ops;
pub mod mul_ops;

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn ut_add_and_mul_ops() {
        let x = add_ops::add_two(3.14159, 2.71828); // 5.85987
		let y = mul_ops::mul_three(3.0, 4.0, 5.0); // 30.0
		let result = add_ops::add_three(x, y, 123.4); //159.25987
        
		assert_eq!(result, 189.25987);
    }
}
