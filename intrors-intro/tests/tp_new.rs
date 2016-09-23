//! Document module

mod props {
	use num::Num;
	use std::cmp;
	use quickcheck::quickcheck;
	use intrors_util::util;
	
	#[test]
	fn testprop_commutadd() {
		fn prop_commutadd<T: Num + Copy>(a: T, b: T) -> bool {
			a + b == b + a && a.add(b) == b.add(a)
		}
		quickcheck(prop_commutadd::<i32> as fn(i32, i32) -> bool);
	}
	
	#[test]
	fn testprop_assocadd() {
		fn prop_assocadd<T: Into<f64> + Num + Copy>(a: T, b: T, c: T) -> bool {
			//(a + b) + c == a + (b + c)
			util::in_epsilon(0.001 * ((a + b) + c).into(), 
				((a + b) + c).into(), (a + (b + c)).into())
		}
		quickcheck(prop_assocadd::<f64> as fn(f64, f64, f64) -> bool);
	}

	#[test]
	fn testprop_rev_reverse() {
		fn prop_rev_reverse<T: Clone + PartialEq>(xs: Vec<T>) -> bool {
			let mut tmp = xs.to_vec();
			tmp.reverse(); tmp.reverse();
			xs == tmp
		}
		quickcheck(prop_rev_reverse::<i32> as fn(Vec<i32>) -> bool);
	}

	#[test]
	fn testprop_id_reverse() {
		fn prop_id_reverse<T: Clone + PartialEq>(xs: Vec<T>) -> bool {
			let mut tmp = xs.to_vec();
			tmp.reverse();
			xs == tmp
		}
		quickcheck(prop_id_reverse::<f64> as fn(Vec<f64>) -> bool);
	}

	#[test]
	fn testprop_sort_reverse() {
		fn prop_sort_reverse<T: Clone + PartialOrd>(xs: Vec<T>) -> bool {
			let (mut tmp0, mut tmp1) = (xs.to_vec(), xs.to_vec());
			tmp1.reverse();
			tmp1.sort_by(|x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Less));
			tmp0.sort_by(|x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Less));
			tmp0 == tmp1
		}
		quickcheck(prop_sort_reverse::<f64> as fn(Vec<f64>) -> bool);
	}
	
	#[test]
	fn testprop_min_sort_head() {
		fn prop_min_sort_head<T: Into<f64> + Clone + PartialOrd>(ys: Vec<T>) -> bool {
			if 0 < ys.len() {
				let xs: Vec<util::OrdF64> = ys.into_iter().map(|e| 
					util::OrdF64::new(e.into())).collect();
				let mut tmp = xs.to_vec();
				tmp.sort();
				xs.iter().min() == Some(tmp.get(0).unwrap())
			} else {
				true
			}
		}
		quickcheck(prop_min_sort_head::<f64> as fn(Vec<f64>) -> bool);
	}
}
