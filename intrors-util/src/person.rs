//! Document module

#[derive(Debug, Eq, Hash, Clone)]
pub struct Person {
	name: String, pub age: i32
}

impl Person {
	pub fn new(name: String, age: i32) -> Person {
		Person { name: name, age: age }
	}
	
	pub fn get_name(&self) -> &String {
		&self.name
	}
	
	pub fn get_age(&self) -> i32 {
		self.age
	}
	
	pub fn set_name(&mut self, new_name: String) {
		self.name = new_name;
	}
	
	pub fn set_age(&mut self, new_age: i32) {
		self.age = new_age;
	}
}

impl PartialEq for Person {
	fn eq(&self, other: &Self) -> bool {
		let mut is_eq = true;
		is_eq &= self.name == other.name;
		is_eq &= self.age == other.age;
		is_eq
	}
}
