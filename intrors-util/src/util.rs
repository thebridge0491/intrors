//! Document module

use std::io; //::Error;
use std::io::prelude::*;
use std::{cmp, string};
use std::collections::hash_map::HashMap;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OrdF64(f64);

impl OrdF64 {
	pub fn new(val: f64) -> OrdF64 {
		if val.is_nan() { OrdF64(0.0) } else { OrdF64(val) }
	}
}
impl Eq for OrdF64 {}
impl Ord for OrdF64 {
	fn cmp(&self, other: &OrdF64) -> cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

/// Example function `it_works`
pub fn it_works() {
}

/// Example function `it_works_c`
#[no_mangle]
pub extern fn it_works_c() {
}

pub fn read_file(path: &str) -> Result<String, io::Error> {
    use std::fs;
    let mut txt = String::new();
    let mut f_in = try!(fs::File::open(path));
    /*match f_in.read_to_string(&mut txt) {
		Ok(_) => Ok(txt),
		Err(err) => Err(err),
	}*/
    try!(f_in.read_to_string(&mut txt));
    Ok(txt)
}

pub fn get_input() -> Result<String, io::Error> {
	let mut input = String::new();
	/*match io::stdin().read_line(&mut input) {
		Ok(_) => Ok(input),
		Err(err) => Err(err),
	}*/
	try!(io::stdin().read_line(&mut input));
	Ok(input)
}

pub fn mkstring<T: string::ToString>(beg: &str, sep: &str, stop: &str,
		arr: &[T]) -> String {
	/*let mut res = String::new();
	for e in arr.iter() {
		res = format!("{}{}{}", res, if res.eq("") { "" } else { sep },
			e.to_string());
	}
	format!("{}{}{}", beg, res, stop)*/
	format!("{}{}{}", beg, arr.iter().fold(String::new(), |acc, e|
		format!("{}{}{}", acc, if acc.eq("") { "" } else { sep },
		e.to_string())), stop)
}

pub fn hmap_to_vec<K: string::ToString, V: string::ToString>(
		hmap1: &HashMap<K, V>) -> Vec<String> {
	/*let mut arr = Vec::<String>::new();
	for (k, v) in hmap1.iter() {
		arr.push(format!("{}: {}", k.to_string(), v.to_string()));
	}
	arr*/
	hmap1.iter().map(move |(k, v)| format!("{}: {}", k.to_string(),
		v.to_string())).collect()
}

pub fn ini_to_str(cfg: &ini::Ini) -> String {
	/*let mut arr: Vec<String> = Vec::<String>::new();
	for (sect, keys_vals) in cfg.iter() {
		for (k, v) in keys_vals.iter() {
			arr.push(format!("{}:{} => {}", sect.to_owned().unwrap(), k, v));
		}
	}*/
	
	let mut map0: HashMap<String, String> = HashMap::new();
	for (sect, keys_vals) in cfg.iter() {
		for (k, v) in keys_vals.iter() {
			map0.insert(format!("{}:{}", sect.to_owned().unwrap(), k),
				format!("{}", v));
		}
	}
	let mut map1: HashMap<&str, &str> = HashMap::new();
	map0.iter().for_each(|(k, v)| { map1.insert(&**k, &**v); });
	let arr: Vec<String> = hmap_to_vec(&map1);
	
	mkstring("\n{", ", ", "}\n", 
		//&arr.iter().map(|s| s.as_ref()).collect() as &Vec<&str>
		//&arr.iter().map(|s| s as &str).collect() as &Vec<&str>
		&arr.iter().map(|s| &**s).collect() as &Vec<&str>
		)
}

pub fn in_epsilon(tolerance: f64, flt1: f64, flt2: f64) -> bool {
	let delta = tolerance.abs();
	//(flt1 - delta) <= flt2 && (flt1 + delta) >= flt2
	!((flt1 + delta) < flt2) && !((flt2 + delta) < flt1)
}

pub fn cartesian_prod<T: Copy, S: Copy>(arr1: &[T], arr2: &[S]) -> Vec<(T, S)> {
	/*let mut arr_prod = Vec::<(T, S)>::new();
	for e in arr1.iter().cloned() {
		for f in arr2.iter().cloned() {
			arr_prod.push((e, f));
		}
	}
	arr_prod*/
	//iproduct!(arr1.iter().cloned(), arr2.iter().cloned()).collect()
	arr1.iter().flat_map(|x| arr2.iter().map(move |y| 
		(*x, *y))).filter(|e| true).collect()
}
