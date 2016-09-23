//! Document module

use std::time::Duration;
use std::thread;

use std::io; //::Error;
use std::io::prelude::*;

use intrors_util::util;

pub fn greeting(greet_path: &str, name: &str) -> String {
	//info!(":greeting() {}:{}:{}", file!(), line!(), column!());
	info!(target: "prac", ":greeting()");
	
	match util::read_file(greet_path) {
		Ok(s) => format!("{} {}", s.trim(), name),
		Err(_) => "HELP, ToDo".to_string()
	}
}

pub fn delay_char(msecs: u64) -> char {
	let mut ch = '\0';
	
	loop {
		thread::sleep(Duration::from_millis(msecs));
		print!("Type any character when ready. ");
		let _ = io::stdout().flush();
		
		match util::get_input() {
			Ok(input) => ch = input.chars().nth(0).unwrap(),
			Err(err) => println!("error: {}", err),
		}
		if '\n' != ch && '\0' != ch {
			break;
		}
	}
	ch
}
