//! Document module

mod cases {
	use intrors_util::util;
	
	#[test]
	fn test_method1() {
		assert!(4 == 2 * 2);
	}

	#[test]
	fn test_method_double() {
		//assert!(4.0 == 4.0);
		assert!(util::in_epsilon(0.001 * 4.0, 4.0, 4.0));
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
