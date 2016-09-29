//! Document module

mod cases {
	use std::collections::{HashMap, HashSet, BinaryHeap};
	use intrors_util::util;

	#[test]
	fn test_vectors() {
		let num_arr: &[i64] = &[16, 2, 77, 29];
		let nines: &[i64] = &[9, 9, 9, 9];
		let mut vec1: Vec<i64> = vec![];
		
		assert!(vec1.is_empty(), "isEmpty");
		vec1.extend(num_arr.iter());
		assert!(num_arr.len() == vec1.len(), "length");
		assert!(16 == vec1[0], "first");
		assert!(Some(1 as usize) == 
			vec1.iter().position(|e| &num_arr[1] == e), "indexOf");
		vec1.extend(nines.iter());
		assert!((num_arr.len() + nines.len()) == vec1.len(), "addAll");
		vec1.sort();
		assert!("[2, 9, 9, 9, 9, 16, 29, 77]" == 
			format!("{:?}", vec1), "sort");
	}
	
	#[test]
	fn test_maps() {
		let char_arr = ['a', 'e', 'k', 'p', 'u', 'k', 'a'];
		let mut map1: HashMap<String, char> = HashMap::new();
		
		assert!(map1.is_empty(), "isEmpty");
		for i in 0..char_arr.len() {
			let key1 = format!("ltr {}", i % 5);
			
			if !map1.contains_key(&key1) {
				map1.insert(key1, char_arr[i]);
			}
		}
		assert!(map1.contains_key("ltr 1"), "contains");
		assert!(Some(&'k') == map1.get("ltr 2"), "get");
		map1.remove("ltr 2");
		assert!(None == map1.get("ltr 2"), "remove");
		map1.insert("ltr 2".to_string(), 'Z');
		assert!(map1.contains_key("ltr 2"), "put");
		
		let mut res = map1.iter().collect::<Vec<(&String, &char)>>();
		res.sort(); //res.sort_by(|a, b| (a.0).cmp(b.0));
		assert!("[(\"ltr 0\", 'a'), (\"ltr 1\", 'e'), (\"ltr 2\", 'Z'), (\"ltr 3\", 'p'), (\"ltr 4\", 'u')]".to_string() == 
			format!("{:?}", res), "toString: {}", format!("{:?}", res));
	}
	
	#[test]
	fn test_sets() {
		let char_arr = ['a', 'e', 'k', 'p', 'u', 'k', 'a'];
		let mut set1: HashSet<char> = HashSet::new();
		let set2: HashSet<_> = ['q', 'p', 'z', 'u'].iter().cloned().collect();
		
		assert!(set1.is_empty(), "isEmpty");
		for item in char_arr.iter().cloned() {
			if !set1.contains(&item) {
				set1.insert(item);
			}
		}
		assert!(set1.contains(&'k'), "contains");
		
		let union: HashSet<_> = set1.union(&set2).collect();
		assert_eq!(union, ['a', 'e', 'k', 'p', 'q', 'u', 'z'].iter().collect(), "union");
		
		let intersect: HashSet<_> = set1.intersection(&set2).collect();
		assert_eq!(intersect, ['p', 'u'].iter().collect(), "intersection");
		
		let differ: HashSet<_> = set1.difference(&set2).collect();
		assert_eq!(differ, ['a', 'e', 'k'].iter().collect(), "differ");
		
		let xor: HashSet<_> = set1.symmetric_difference(&set2).collect();
		assert_eq!(xor, ['a', 'e', 'k', 'q', 'z'].iter().collect(), "xor");
		
		let mut res: Vec<&char> = set1.iter().collect();
		res.sort();
		assert!("['a', 'e', 'k', 'p', 'u']" == format!("{:?}", res), 
			"toString: {}", format!("{:?}", res));
	}

	#[test]
	fn test_heaps() {
		let num_arr: &[util::OrdF64] = &[util::OrdF64::new(25.7),
			util::OrdF64::new(0.1), util::OrdF64::new(78.5), 
			util::OrdF64::new(52.3)];
		let mut heap1: BinaryHeap<util::OrdF64> = BinaryHeap::new();
		
		assert!(heap1.is_empty(), "isEmpty");
		for item in num_arr.iter().cloned() {
			heap1.push(item);
		}
		assert!(num_arr.len() == heap1.len(), "length");
		assert_eq!(util::OrdF64::new(78.5), *heap1.peek().unwrap(), "peek");
		assert_eq!(util::OrdF64::new(78.5), heap1.pop().unwrap(), "pop");
		heap1.push(util::OrdF64::new(79.0));
		assert_eq!(util::OrdF64::new(79.0), *heap1.peek().unwrap(), "push");
		let heap2 = heap1.clone();
		let res = heap2.into_sorted_vec();
		assert!("[OrdF64(0.1), OrdF64(25.7), OrdF64(52.3), OrdF64(79.0)]" == 
			format!("{:?}", res), "toString: {}", format!("{:?}", res));
	}
}
