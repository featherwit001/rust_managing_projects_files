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

	pub fn calc_perimeter(&self) -> f64 {
		2.0 * self.width + 2.0 * self.height
	} 
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn ut_rectange_new () {
		let rect = Rectangle::new(1.2, 3.4);
		assert_eq!(rect.width, 1.2);
		assert_eq!(rect.height, 3.4);
	}

	#[test]
	fn ut_calc_area () {
		let rect = Rectangle::new(4.8, 7.68);
		let area = rect.get_feature(Feature::Area);
		assert_eq!(area, 36.864);
	}

	#[test]
	fn ut_calc_perimeter() {
		let rect = Rectangle::new(4.8, 7.68);
		let perimeter = rect.get_feature(Feature::Perimeter);
		assert_eq!(perimeter, 24.96);
	}

}