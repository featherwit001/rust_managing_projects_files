// use crate::greeting::formal::spanish;

// use crate::greeting::formal;
// use crate::greeting::casual;

use crate::greeting::{formal, casual};

fn main() {
	greeting::formal::english();
	formal::spanish();
	// spanish();
	casual::spanish();
}

mod greeting {
	pub mod formal {
		pub fn english () {
			println! ("hello");
		}
	
		pub fn spanish () {
			println! ("hola");
		}
	} 

	pub mod casual {
		fn english () {
			println!("hey");
		}

		pub fn spanish () {
			println! ("oye");
		}
	}
}