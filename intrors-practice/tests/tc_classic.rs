//! Document module

mod cases {
	use intrors_util::util;
	use intrors_practice::classic;

	#[test]
	fn test_fact() {
		//let funcs: [fn(i64) -> i64 ; 2] = [classic::fact_i,
		//	classic::fact_lp];
		let funcs: Vec<fn(i64) -> i64> = vec![classic::fact_i,
			classic::fact_lp];
		
		for f in funcs.iter() {
			assert!(120 == f(5));
			assert!(5040 == f(7));
		}
	}

	#[test]
	fn test_expt() {
		//let funcs: [fn(f64, f64) -> f64 ; 2] = [classic::expt_i,
		//	classic::expt_lp];
		let funcs: Vec<fn(f64, f64) -> f64> = vec![classic::expt_i,
			classic::expt_lp];
		let (arr1, arr2) = ([2.0 as f64, 10.0, 20.0], [3.0 as f64, 6.0, 11.0]);
		let prod_arr = util::cartesian_prod(&arr1, &arr2);
		
		for f in funcs.iter() {
			for (b, n) in prod_arr.iter().cloned() {
				let ans = b.powf(n);
				assert!(util::in_epsilon(0.001 * ans, ans, f(b, n)));
			}
		}
	}
}
