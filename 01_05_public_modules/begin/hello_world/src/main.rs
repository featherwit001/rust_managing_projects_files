fn main() {
	hello::english();
	hello::casual::english();
	// private
	// hello::casual::spanish(); 

}

// hello module is public to items in the main.rs, i.e. binary crate root 
// so fn main() use items in hello, pub is unnecessary to hello module
// the same as hello::english use hello::casual::english (line 14), 
// even if mod casual do not have pub
mod hello {
	pub fn english () {
		println! ("hello");
		spanish();
		casual::english();
		// private
		// casual::spanish();
	}

	fn spanish () {
		println! ("hola");
	}

	pub mod casual {
		pub fn english () {
			println!("hey");
			super::spanish();
		}

		fn spanish () {
			println!("ooohh");
		}
	}
}