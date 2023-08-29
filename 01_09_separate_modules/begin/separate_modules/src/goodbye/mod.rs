
pub fn description() {
	println!("goodbye messages");
}


pub mod formal {
    use crate::goodbye::casual;

	pub fn english () {
		println! ("goodbye");
	}

	pub fn spanish () {
		println! ("adios");
		casual::english();
		casual::spanish();
	}
} 

mod casual;


// new added 
pub mod doom;