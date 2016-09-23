//! Document module

mod props {
	use quickcheck::quickcheck;
	use intrors_util::util;
	use intrors_practice::classic;
	
	#[test]
	fn testprop_fact() {
		fn prop_fact(x: i64) -> bool {
			let n = x.abs() % 19;
			let ans: i64 = (1..(1 + n as i64)).product();
			(|| {	// puzzling error: use || { ... } closure form
				let funcs: Vec<fn(i64) -> i64> = vec![classic::fact_lp, 
					classic::fact_i];
				/*let mut res = true;
				for f in funcs {
					res = res && ans == f(n);
				}
				res*/
				funcs.iter().fold(true, |acc, &f| acc && ans == f(n))
			})()
		}
		quickcheck(prop_fact as fn(i64) -> bool);
	}
	
	#[test]
	fn testprop_expt() {
		fn prop_expt(x: u32, y: u32) -> bool {
			let (b, n) = ((x % 19 + 2) as f64, (y % 9 + 2) as f64);
			let ans = b.powf(n);
			(|| {	// puzzling error: use || { ... } closure form
				let funcs: Vec<fn(f64, f64) -> f64> = 
					vec![classic::expt_lp, classic::expt_i];
				/*let mut res = true;
				for f in funcs {
					res = res && util::in_epsilon(0.001 * ans, ans, f(b, n));
				}
				res*/
				funcs.iter().fold(true, |acc, &f| 
					acc && util::in_epsilon(0.001 * ans, ans, f(b, n)))
			})()
		}
		quickcheck(prop_expt as fn(u32, u32) -> bool);
	}
}
