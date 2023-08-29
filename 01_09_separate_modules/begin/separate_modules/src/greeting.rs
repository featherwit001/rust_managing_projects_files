pub fn description () {
	println!("greeting messages");
}

pub mod formal {
	pub fn english () {
		println! ("hello");
	}

	pub fn spanish () {
		println! ("hola");
	}
} 

// submodule in the greeting/casual
pub mod casual;

pub mod coco;
