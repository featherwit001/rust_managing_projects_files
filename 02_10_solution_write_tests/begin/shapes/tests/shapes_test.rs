use myshapes::shapes::{*, rectangle::Rectangle, circle::Circle};
use rand::prelude::*;

mod test_utils;


#[test]
fn it_rand_rect () {
	let mut rng = rand::thread_rng();
	for n in 0..1000 {
		let a = rng.gen_range(0.0..100.0);
		let b = rng.gen_range(0.0..100.0);
		let rect = Rectangle::new(a, b);
		let area = rect.get_feature(Feature::Area);
		let preimeter = rect.get_feature(Feature::Perimeter);
		assert_eq!(area, a * b);
		assert_eq!(preimeter, (a + b) * 2.0);
	}
}


use std::f64::consts::PI;

#[test]
fn it_rand_cric () {
	let mut rng = rand::thread_rng();
	for n in 0..1000 {
		let a = rng.gen_range(0.0..100.0);
		let cric = Circle::new(a);
		let area = cric.get_feature(Feature::Area);
		let preimeter = cric.get_feature(Feature::Perimeter);
		assert_eq!(area, PI * a.powi(2));
		assert_eq!(preimeter, 2.0 * PI * a );
	}
}

