//! Document module

pub fn swap_items<T: Clone>(idx1: usize, idx2: usize, arr: &mut[T]) {
	//arr.swap(idx1, idx2);
	let swap = arr[idx1].clone(); arr[idx1] = arr[idx2].clone(); arr[idx2] = swap;
}

pub fn copy_of_lp<T: Clone>(arr: &[T]) -> Vec<T> {
	//arr.clone().to_vec() //arr.iter().map(move |el| el.clone()).collect()
	let mut new_vec = Vec::<T>::new();
	
	for el in arr.iter().cloned() {
		new_vec.push(el);
	}
	new_vec
}

pub fn find_index_lp<T: Eq>(data: T, arr: &[T]) -> Option<usize> {
	//match arr.iter().position(|x| data.eq(x))
	for i in 0..arr.len() {
		if data.eq(&arr[i]) { return Some(i as usize); }
	}
	None
}

pub fn reverse_lp<T: Clone>(mut arr: &mut [T]) {
	//arr.reverse();
	info!(target: "prac", ":reverse_lp()");
	
	for i in 0..(arr.len() / 2) {
		swap_items(i, arr.len() - 1 - i, &mut arr);
	}
}

pub fn reverse_i<T: Clone>(arr: &[T]) -> Vec<T> {
	fn iter<T: Clone>(rst: &[T], acc: &mut Vec<T>) -> Vec<T> {
		match rst.is_empty() {
			true => acc.clone(),
			_ => { acc.push(rst.last().unwrap().clone()) ; 
				iter(&rst[0..(rst.len() - 1)], acc) }
		}
	}
	iter(&arr, &mut Vec::<T>::new())
}

