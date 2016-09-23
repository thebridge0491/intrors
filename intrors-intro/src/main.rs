//! Document crate

#[macro_use] 
extern crate log;
//extern crate env_logger;
extern crate log4rs;
extern crate argparse;
extern crate getopts;
extern crate regex;
extern crate rand;
extern crate chrono;
extern crate ini;
//extern crate serde_json;
//extern crate toml;
//extern crate yaml_rust;

extern crate intrors_util;
extern crate intrors_practice;
extern crate intrors_intro;

use std::fmt::{Write};
use std::process;
use std::default::{Default};
use std::env;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use argparse::{ArgumentParser, Store, StoreTrue};
use getopts::{Options};
use regex::{Regex};
use chrono::{Local};
use ini::{Ini};
//use serde_json::{Value};
//use yaml_rust::{YamlLoader};

use intrors_util::util;
use intrors_practice::{classic, sequenceops};
use intrors_intro::{person, intro};

#[derive(Debug)]
struct OptsRecord {
	name: String, num: u32, is_expt2: bool
}

//#[derive(Debug)] #[derive(Default)]
#[derive(Debug, Default)]
pub struct User {
   pub name: String, pub num: u32, pub timestamp: i64
}

const PI: f32 = 3.14;

type UcharT = char;

enum ConstItems { ZERO, NUMZ = 26 }

enum Uvar { Width(u32), Height(f32) }

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn run_intro(prog: String, rsrc_path: String, opts_rec: OptsRecord) {
    let time_in = Local::now();
    
    // basic datatypes
    let mut is_done: bool = false;
    let (mut num_i, mut arr_len) = (0, ConstItems::ZERO as usize);
    let mut time_diff: f32 = 0.0;
    let delay_secs: u64 = 2500;
    let mut ch: char = '\0';
    
    // pointers
    
    
    // strings & arrays
    let (mut greet_buf, mut date_buf) = ("".to_string(), "".to_string());
    let num_arr = [9, 0o11, 0x9, 0b1001];
    
    // composites
    let u_var1 = Uvar::Width(36);
    let mut pers = person::Person::new("I.M. Computer".to_string(), 32);
    
    let num = if 0 == opts_rec.num { (rand::random::<u32>() % 17) + 2 } else { opts_rec.num };
    
    for elem in num_arr.iter().cloned() {
    	num_i += elem;
    }
    arr_len = num_arr.len();
    assert!(arr_len * num_arr[0] == num_i, "arr_len * num_arr[0] != num_i");
    
    let mut user1 = User { name: opts_rec.name.clone(), //num: opts_rec.num, 
    	timestamp: time_in.timestamp(), ..Default::default() };
    
    ch = intro::delay_char(delay_secs);
    
    let re = Regex::new(r"^(?i)quit$").unwrap();
    let match1 = re.is_match(&opts_rec.name);
    
    println!("{} match: {} to {}", if match1 { "Good" } else { "Does not" },
		opts_rec.name, re.as_str());
    
    let pers1 = pers.clone();
    user1.num = num;
    
    date_buf = time_in.format("%c").to_string();//time_in.to_rfc3339()
    
    greet_buf = intro::greeting(&(rsrc_path + "/greet.txt"), &user1.name);
    println!("{}\n{}!\n", date_buf, greet_buf);
    
    let time_dur = (Local::now() - time_in).num_milliseconds() as f32 / 1000.0;
    println!("(program {}) Took {:.1} secs.", prog, time_dur);
    
    let mut arr_ints = vec![9, 9, 9, 9];
    arr_ints.extend([2, 1, 0, 4, 3].iter());
    
    if opts_rec.is_expt2 {
    	println!("expt({:.1}, {:.1}): {:.1}", 2.0, num.clone() as f64,
    		classic::expt_lp(2.0, num.clone() as f64));
    	
    	let res = format!("{:?}", arr_ints);
    	let mut arr = sequenceops::copy_of_lp(&arr_ints).to_owned();
    	sequenceops::reverse_lp(&mut arr);
    	println!("reverse({}): {:?}", res, arr);
    	arr.sort();
    	println!("{}.sort(): {:?}", res, arr);
    	arr_ints.sort_by(|a, b| b.cmp(a));
    	println!("{}.sort_by(|a, b| b.cmp(a)): {:?}", res, arr_ints);
    } else {
    	println!("fact({}): {}", num, classic::fact_lp(num as i64));
    	
    	let res = format!("{:?}", arr_ints);
    	println!("find_index_lp({}, {:?}): {:?}", 3, res, 
    		sequenceops::find_index_lp(3, &arr_ints));
    	arr_ints.push(50);
    	println!("{}.push({}): {:?}", res, 50, arr_ints);
    }
    //println!("{}", std::iter::repeat("-").take(40).collect::<String>());
    println!("{}", String::from_utf8(vec![b'-' ; 40]).unwrap());
    
    println!("{:?}", pers);
    pers.set_age(33);
    println!("{:?}", pers);
    println!("{}", String::from_utf8(vec![b'-' ; 40]).unwrap());
    
    println!("pers == pers1: {:?}", pers.eq(&pers1));
    println!("hash(&pers): {:?} ; hash(&pers1): {:?}", hash(&pers), hash(&pers1));
}

fn print_usage(prog: &str, opts: Options) {
	let brief = format!("Usage: {} [options]", prog);
	print!("{}", opts.usage(&brief));
}

fn parse_getopts(opts: &mut OptsRecord, args: &Vec<String>, prog: &str) {
    info!(":parse_getopts()");
    let mut options = Options::new();
    options.optopt("u", "user", "set user name", "USER");
    options.optopt("n", "num", "set number", "NUM");
    options.optflag("2", "", "expt2 vice fact");
    options.optflag("h", "help", "print this help menu");
    let matches = match options.parse(&args[1..]) {
    	Ok(m) => { m }
    	//Err(f) => { panic!(f.to_string()) }
    	Err(f) => { println!("{:?}", f); print_usage(&prog, options); 
    		process::exit(0) }
    };
    for it in ["h", "u", "n", "2"].iter() {
    	let el = it.to_string();
    	match el {
    		_ if el == "h" => if matches.opt_present("h") {
    			print_usage(&prog, options); process::exit(0); }, //return; },
    		_ if el == "u" => if matches.opt_present("u") {
    			opts.name = matches.opt_str("u").unwrap(); },
    		_ if el == "n" => if matches.opt_present("n") {
    			opts.num = matches.opt_str("n").unwrap().parse().unwrap(); },
    		_ if el == "2" => if matches.opt_present("2") {
    			opts.is_expt2 = matches.opt_present("2"); },
    		_ => (),
    	}
    }
}

fn parse_cmdopts(opts: &mut OptsRecord, prog: &str) {
	info!(":parse_cmdopts()");
	let mut ap = ArgumentParser::new();
	ap.set_description("Get cmdln args");
	ap.refer(&mut opts.name).add_option(&["-u", "--user"], Store, "user");
	ap.refer(&mut opts.num).add_option(&["-n", "--num"], Store, "num");
	ap.refer(&mut opts.is_expt2).add_option(&["-2"], StoreTrue, 
		"expt2 vice fact");
	
	match ap.parse_args() {
		Ok(()) => {}
		//Err(x) => { process::exit(x); }
		Err(_) => { let mut stderr = std::io::stderr();
			let _ = ap.print_help(&prog, &mut stderr);
			process::exit(0); }
	}
}

/// Entry point
fn main() {
	let rsrc_path = &env::var("RSRC_PATH").unwrap_or_else(|_| {
		String::from("resources")});
	
	//env::set_var("RUST_LOG", "main=debug,main::misc=info");
    //env_logger::init().unwrap();
    log4rs::init_file(rsrc_path.to_owned() + "/log4rs.yaml",
		Default::default()).unwrap();
    
    debug!(target: "prac", ":main() - entering");
    
    let args: Vec<String> = env::args().collect();
    let prog = args[0].clone();
    let mut opts = OptsRecord {name: "World".to_string(), num: 0, 
    	is_expt2: false};
    
    //parse_getopts(&mut opts, &args, &prog);
    parse_cmdopts(&mut opts, &prog);
    
    
    let ini_cfg = Ini::load_from_file(rsrc_path.to_owned() + "/prac.conf").unwrap();
    let ini_str = util::ini_to_str(&ini_cfg);
    
    /*let json_str = match util::read_file(&[rsrc_path, "/prac.json"].concat()) {
		Ok(s) => s,
		Err(_) => String::new()
	};
    let json_data : Value = serde_json::from_str(&json_str).unwrap();
    let json_root = json_data.as_object().unwrap();
    let json_user1 = json_root["user1"].as_object().unwrap();
    
    let toml_str = util::read_file(&[rsrc_path, "/prac.toml"].concat()
		).unwrap_or(String::new());
    let toml_data : Value = toml::from_str(&toml_str).unwrap();
    let toml_root = toml_data.as_object().unwrap();
    let toml_user1 = toml_root["user1"].as_object().unwrap();
    
    let yaml_str = util::read_file(&[rsrc_path, "/prac.yaml"].concat()
		).unwrap_or(String::new());
	let yaml_vec = &YamlLoader::load_from_str(&yaml_str).unwrap();
	let yaml_user1 = &yaml_vec[0]["user1"];*/
    
    let row_arr = [
		[ini_str.as_str(),
			ini_cfg.get_from(Some("default"), "domain").unwrap(),
			ini_cfg.get_from(Some("user1"), "name").unwrap()]/*,
		[json_str.as_str(),
			json_root["domain"].as_str().unwrap(),
			json_user1["name"].as_str().unwrap()],
		[toml_str.as_str(),
			toml_root["domain"].as_str().unwrap(),
			toml_user1["name"].as_str().unwrap()],
		[yaml_str.as_str(),
			yaml_vec[0]["domain"].as_str().unwrap(),
			yaml_user1["name"].as_str().unwrap()]*/
    ];
    
    for row in row_arr.iter().cloned() {
		println!("config: {}", row[0]);
		println!("domain: {}", row[1]);
		println!("user1Name: {}\n", row[2]);
	}
    
    run_intro(prog, rsrc_path.to_string(), opts);
    
    //info!(":main() {}:{}:{} - exiting", file!(), line!(), column!());
    info!(":main() - exiting");
}
