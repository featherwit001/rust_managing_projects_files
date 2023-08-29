pub struct Person {
	pub name: String
}

impl Person {
	pub fn new (name :String) -> Person {
		if name.is_empty() {
			panic!("A Person needs a name.");
		}
		Person {name}
	} 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
		let new_person = Person::new("Steve".to_string());
        assert_eq!(new_person.name, "Steve".to_string());
    }

	#[test]
	#[should_panic(expected = "A Person needs a name")]
	fn it_still_panics() {
		let new_person = Person::new("Barron".to_string());
        assert_eq!(new_person.name, "Steve".to_string());
    }

	#[test]
	#[should_panic(expected = "needs a name")]
	fn person_test() {
		let new_person = Person::new("".to_string());
        assert_eq!(new_person.name, "Steve".to_string());
    }
}
