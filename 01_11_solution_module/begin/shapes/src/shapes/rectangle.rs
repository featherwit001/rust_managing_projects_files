use crate::shapes::Feature;

pub struct Rectangle {
	pub width: f64,
	pub height: f64
}

impl Rectangle {
	pub fn new(width: f64, height: f64) -> Rectangle {
		Rectangle { width, height }
	}

	pub fn get_feature(&self, feature: Feature) -> f64 {
		match feature {
			Feature::Area => self.calc_area(),
			Feature::Perimeter => self.calc_perimeter()
		}
	}

	pub fn calc_area(&self) -> f64 {
		self.width * self.height
	}

	fn calc_perimeter(&self) -> f64 {
		2.0 * self.width + 2.0 * self.height
	} 
}
