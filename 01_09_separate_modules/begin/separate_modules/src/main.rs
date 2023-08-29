mod greeting;
mod goodbye;

// mod mod;

// mod mainmod;
 
fn main() {
	// function form greeting.rs
	greeting::description(); // greeting messages

	// functions form inline submodule within greeting.rs
	greeting::formal::english(); // hello
	greeting::formal::spanish(); // hola

	// functions form separate submodule within greeting.rs
	greeting::casual::english(); //  hey
	greeting::casual::spanish(); // oye
	
	// greeting::casual::doom::curse();
	use greeting::casual::doom;
	doom::curse();

	greeting::coco::test();

	// functions from goodbye/mod.rs
	goodbye::description(); // goodbye messages

	// functions form inline submodule within goodbye/mod.rs
	goodbye::formal::english(); // goodbye
	goodbye::formal::spanish(); // adios

	//functions from separate submodule greeting/casual.rs
	// goodbye::casual::english(); // see you later
	// goodbye::casual::spanish(); // hasta luego

	goodbye::doom::curse();

	// exception
	// mod::test();
}