// use rand::{thread_rng, Rng};
// use rand::*;
use rand::prelude::*;

fn main() {
	let mut rng = thread_rng();
	let num = rng.gen::<u32>();
	println!("the num is {}", num);
}
