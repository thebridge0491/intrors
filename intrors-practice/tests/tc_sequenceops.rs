//! Document module

mod cases {
	use intrors_util::util;
	use intrors_practice::sequenceops;

	#[test]
	fn test_find_index() {
		let funcs: Vec<fn(i32, &[i32]) -> Option<usize>> = vec![
			sequenceops::find_index_lp::<i32>];
		let (arr, arr_rev) = ([2, 1, 0, 4, 3], [3, 4, 0, 1, 2]);
		let (vec, vec_rev) = (vec![2, 1, 0, 4, 3], vec![3, 4, 0, 1, 2]);
		
		for f in funcs.iter() {
			assert!(Some(4) == f(3, &arr) && Some(4) == f(3, &vec));
			assert!(Some(0) == f(3, &arr_rev) && Some(0) == f(3, &vec_rev));
		}
	}

	#[test]
	fn test_reverse() {
		let funcs_mut_imm: Vec<(fn(&mut [i32]) -> (), fn(&[i32]) -> Vec<i32>)> = 
			vec![(sequenceops::reverse_lp, sequenceops::reverse_i)];
		let (arr, arr_rev) = ([2, 1, 0, 4, 3], [3, 4, 0, 1, 2]);
		let (vec, vec_rev) = (vec![2, 1, 0, 4, 3], vec![3, 4, 0, 1, 2]);
		//let mut tmp = arr.clone(); 
		let mut tmp = sequenceops::copy_of_lp(&arr).to_owned();
		
		for &(fn_mut, fn_imm) in funcs_mut_imm.iter() {
			fn_mut(&mut tmp);
			assert!(arr_rev.to_vec().eq(&tmp), "{:?} != {:?}", arr_rev, tmp);
			
			tmp = sequenceops::copy_of_lp(&vec).to_owned();
			//tmp = vec.clone(); 
			fn_mut(&mut tmp);
			assert!(vec_rev.eq(&tmp), "{:?} != {:?}", vec_rev, tmp);
			
			assert!(arr_rev.to_vec().eq(&fn_imm(&arr)), "{:?} != {:?}", 
				arr_rev, tmp);
			assert!(vec_rev.eq(&fn_imm(&vec)), "{:?} != {:?}", vec_rev, tmp);
		}
	}
}
