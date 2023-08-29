use crate::shapes::Feature;
use std::f64::consts::PI;

pub struct Circle {
	pub radius: f64
}

impl Circle {
	pub fn new(radius: f64) -> Circle {
		Circle { radius }
	}

	pub fn get_feature(&self, feature: Feature) -> f64 {
		match feature {
			Feature::Area => self.calc_area(),
			Feature::Perimeter => self.calc_circumference()
		}
	}

	fn calc_area(&self) -> f64 {
		PI * self.radius.powi(2)
	}

	pub fn calc_circumference(&self) -> f64 {
		2.0 * PI * self.radius
	}
}


#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn ut_rectange_new () {
		let circ = Circle::new(3.4);
		assert_eq!(circ.radius, 3.4_);
	}

	#[test]
	fn ut_calc_area () {
		let circ = Circle::new(6.3);
		let area = circ.get_feature(Feature::Area);
		assert_eq!(area, PI * (6.3_f64).powi(2));
	}

	#[test]
	fn ut_calc_perimeter() {
		let circ = Circle::new(7.68);
		let perimeter = circ.get_feature(Feature::Perimeter);
		assert_eq!(perimeter, PI * 7.68 * 2.);
	}

}