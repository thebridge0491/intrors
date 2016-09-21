//! Document module

mod cases {
	pub fn in_epsilon(tolerance: f64, flt1: f64, flt2: f64) -> bool {
		let delta = tolerance.abs();
		//(flt1 - delta) <= flt2 && (flt1 + delta) >= flt2
		!((flt1 + delta) < flt2) && !((flt2 + delta) < flt1)
	}
	
	#[test]
	fn test_method1() {
		assert!(4 == 2 * 2);
	}

	#[test]
	fn test_method_double() {
		//assert!(4.0 == 4.0);
		assert!(in_epsilon(0.001 * 4.0, 4.0, 4.0));
	}

	#[test]
	fn test_method_string() {
		assert!("Hello" == "Hello");
	}

	#[test] #[should_panic]
	fn test_fail() {
		assert!(false);
	}

	#[test] #[ignore]
	fn test_ignore() {
		assert!(false);
	}
}
