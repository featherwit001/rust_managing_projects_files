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