//! Document module

mod cases {
	use intrors_foreignc::classic;
	use intrors_util::util;

	#[test]
	fn test_fact() {
		//let funcs: [unsafe extern fn(i32) -> i64 ; 2] = [classic::fact_i,
		//	classic::fact_lp];
		let funcs: Vec<unsafe extern fn(i32) -> i64> = 
			vec![classic::fact_i, classic::fact_lp];
		
		for f in funcs.iter() {
			assert!(120 == unsafe { f(5) });
			assert!(5040 == unsafe { f(7) });
		}
	}

	#[test]
	fn test_expt() {
		//let funcs: [unsafe extern fn(f32, f32) -> f32 ; 2] = 
		//	[classic::expt_i, classic::expt_lp];
		let funcs: Vec<unsafe extern fn(f32, f32) -> f32> = 
			vec![classic::expt_i, classic::expt_lp];
		let (arr1, arr2) = ([2.0, 10.0, 20.0], [3.0, 6.0, 11.0]);
		let prod_arr = util::cartesian_prod(&arr1, &arr2);
		
		for f in funcs.iter() {
			for (b, n) in prod_arr.iter().cloned() {
				let ans = (b as f64).powf(n);
				assert!(util::in_epsilon(0.001 * ans, ans, 
					unsafe { f(b as f32, n as f32) as f64 }));
			}
		}
	}
}
