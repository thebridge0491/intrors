//! Document module

mod props {
	use quickcheck::quickcheck;
	use intrors_util::util;
	use intrors_practice::sequenceops;

	#[test]
	fn testprop_find_index() {
		fn prop_find_index<T: Eq + Clone>(data: T, xs: Vec<T>) -> bool {
			let ans = xs.iter().position(|e| &data == e);
			(|| {	// puzzling error: use || { ... } closure form
				let funcs: Vec<fn(T, &[T]) -> Option<usize>> = 
					vec![sequenceops::find_index_lp];
				/*let mut res = true;
				for f in funcs {
					res = res && ans == f(data.clone(), &xs);
				}
				res*/
				funcs.iter().fold(true, |acc, &f| {
					acc && ans == f(data.clone(), &xs) })
			})()
		}
		quickcheck(prop_find_index as fn(i32, Vec<i32>) -> bool);
	}

	#[test]
	fn testprop_reverse() {
		fn prop_reverse<T: Clone + PartialEq>(xs: Vec<T>) -> bool {
			let mut ans = xs.to_vec(); //xs.clone();
			ans.reverse();
			(|| {	// puzzling error: use || { ... } closure form
				let funcs_mut_imm: Vec<(fn(&mut [T]) -> (), 
						fn(&[T]) -> Vec<T>)> = 
					vec![(sequenceops::reverse_lp, sequenceops::reverse_i)];
				/*let mut res = true;
				for (fn_mut, fn_imm) in funcs_mut_imm {
					let mut tmp = sequenceops::copy_of_lp(&xs).to_owned();
					fn_mut(tmp.as_mut_slice());
					res = res && ans == tmp && ans == fn_imm(&xs);
				}
				res*/
				funcs_mut_imm.iter().fold(true, |acc, &(fn_mut, fn_imm)| {
					let mut tmp = sequenceops::copy_of_lp(&xs).to_owned();
					fn_mut(tmp.as_mut_slice());
					acc && ans == tmp && ans == fn_imm(&xs) })
			})()
		}
		quickcheck(prop_reverse as fn(Vec<i32>) -> bool);
	}
}
