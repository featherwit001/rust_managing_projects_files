use math_func:: *;
use rand::prelude::*;

mod test_utils;

#[test]
fn it_add_rand() {
	let mut rng = thread_rng();
	for i in 0..1000 {
		let x = rng.gen_range(0..100);
		let y = rng.gen_range(0..100);
		let z = rng.gen_range(0..100);
		assert_eq!(add_ops::add_two(x, y), x + y);
		assert_eq!(add_ops::add_three(x, y, z), x + y + z);
		test_utils::print_test();
	}
}


#[test]
fn it_mul_rand(){
	let mut rng = thread_rng();
	for i in 0..1000 {
		let x = rng.gen_range(0..100);
		let y = rng.gen_range(0..100);
		let z = rng.gen_range(0..100);
		assert_eq!(mul_ops::mul_two(x, y), x * y);
		assert_eq!(mul_ops::mul_three(x, y, z), x * y * z);
		test_utils::print_test();
	}
}