fn main() {
    crate::hello::english(); // hello
	crate::hello::spanish(); // hola
	hello::spasnish();
	crate::hello::casual::english(); // hey
}

mod hello {
	fn english () {
		println! ("hello");
		self::spanish();
		spanish();
		casual::english();
	}

	fn spanish () {
		println! ("hola");
	}

	mod casual {
		fn english () {
			println!("hey");
			super::spanish();
		}
	}
}