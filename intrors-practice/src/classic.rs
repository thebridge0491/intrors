//! Document module

pub fn fact_lp(n: i64) -> i64 {
	let mut acc: i64 = 1;
	info!(target: "prac", ":fact_lp()");
	
	//for i in range(2, n + 1) {
	for i in 2..(n + 1) {
		acc *= i as i64;
	}
	acc
}

pub fn fact_i(n: i64) -> i64 {
	fn iter(m: i64, acc: i64) -> i64 {
		/*if m > 1 {
			iter(m - 1, acc * m as u64)
		} else {
			acc
		}*/
		match m > 1 {
			true => iter(m - 1, acc * m as i64),
			_ => acc
		}
	}
	iter(n, 1)
}

pub fn expt_lp(b: f64, n: f64) -> f64 {
	let mut acc = 1.0;
	
	for _ in 1..(n as u32 + 1) {
		acc *= b;
	}
	acc
}

pub fn expt_i(b: f64, n: f64) -> f64 {
	fn iter(b: f64, m: f64, acc: f64) -> f64 {
		match m > 0.0 {
			true => iter(b, m - 1.0, acc * b),
			_ => acc
		}
	}
	iter(b, n, 1.0)
}
