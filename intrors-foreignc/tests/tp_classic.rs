//! Document module

mod props {
	use quickcheck::quickcheck;
	use intrors_foreignc::classic;
	use intrors_util::util;
	
	#[test]
	fn testprop_fact() {
		fn prop_fact(x: u32) -> bool {
			let n = x % 19;
			let ans: i64 = (1..(1 + n as i64)).product();
			(|| {	// puzzling error: use || { ... } closure form
				let funcs: Vec<unsafe extern fn(i32) -> i64> = 
					vec![classic::fact_lp, classic::fact_i];
				/*let mut res = true;
				for f in funcs {
					res = res && ans == unsafe { f(n) };
				}
				res*/
				funcs.iter().fold(true, |acc, &f| acc && ans == unsafe { f(n as i32) })
			})()
		}
		quickcheck(prop_fact as fn(u32) -> bool);
	}
	
	#[test]
	fn testprop_expt() {
		fn prop_expt(x: u32, y: u32) -> bool {
			let (b, n) = ((x % 19 + 2) as f64, (y % 9 + 2) as f64);
			let ans = (b as f64).powf(n as f64);
			(|| {	// puzzling error: use || { ... } closure form
				let funcs: Vec<unsafe extern fn(f32, f32) -> f32> = 
					vec![classic::expt_lp, classic::expt_i];
				/*let mut res = true;
				for f in funcs {
					res = res && util::in_epsilon(0.001 * ans, ans, 
					 	unsafe { f(b as f32, n as f32) as f64 });
				}
				res*/
				funcs.iter().fold(true, |acc, &f| 
					acc && util::in_epsilon(0.001 * ans, ans,
					unsafe { f(b as f32, n as f32) as f64 }))
			})()
		}
		quickcheck(prop_expt as fn(u32, u32) -> bool);
	}
}
