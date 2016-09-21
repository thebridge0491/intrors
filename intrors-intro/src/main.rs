//! Document crate

#[macro_use] 
extern crate log;
//extern crate env_logger;
extern crate log4rs;
extern crate argparse;
extern crate getopts;
extern crate regex;
extern crate rand;
extern crate ini;
extern crate serde_json;
extern crate toml;
extern crate yaml_rust;

extern crate intrors_intro;

use std::fmt::{Write};
use std::process;
use std::default::{Default};
use std::env;
use argparse::{ArgumentParser, Store};
use getopts::{Options};
use regex::{Regex};
use ini::{Ini};
use serde_json::{Value};
use yaml_rust::{YamlLoader};

use intrors_intro::intro;

#[derive(Debug)]
struct OptsRecord {
	name: String
}

fn run_intro(prog: String, name: String) {
    let re = Regex::new(r"^(?i)quit$").unwrap();
    let match1 = re.is_match(&name);
    
    println!("{} match: {} to {}", match1, name, re.as_str());
}

fn print_usage(prog: &str, opts: Options) {
	let brief = format!("Usage: {} [options]", prog);
	print!("{}", opts.usage(&brief));
}

fn parse_getopts(opts: &mut OptsRecord, args: &Vec<String>, prog: &str) {
    info!(":parse_getopts()");
    let mut options = Options::new();
    options.optopt("u", "user", "set user name", "USER");
    options.optflag("h", "help", "print this help menu");
    let matches = match options.parse(&args[1..]) {
    	Ok(m) => { m }
    	//Err(f) => { panic!(f.to_string()) }
    	Err(f) => { println!("{:?}", f); print_usage(&prog, options); 
    		process::exit(0) }
    };
    for it in ["h", "u"].iter() {
    	let el = it.to_string();
    	match el {
    		_ if el == "h" => if matches.opt_present("h") {
    			print_usage(&prog, options); process::exit(0); }, //return; },
    		_ if el == "u" => if matches.opt_present("u") {
    			opts.name = matches.opt_str("u").unwrap(); },
    		_ => (),
    	}
    }
}

fn parse_cmdopts(opts: &mut OptsRecord, prog: &str) {
	info!(":parse_cmdopts()");
	let mut ap = ArgumentParser::new();
	ap.set_description("Get cmdln args");
	ap.refer(&mut opts.name).add_option(&["-u", "--user"], Store, "user");
	
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
    log4rs::init_file(rsrc_path.to_owned() + "/log4rs.toml",
		Default::default()).unwrap();
    
    debug!(target: "prac", ":main() - entering");
    
    let args: Vec<String> = env::args().collect();
    let prog = args[0].clone();
    let mut opts = OptsRecord {name: "World".to_string()};
    
    //parse_getopts(&mut opts, &args, &prog);
    parse_cmdopts(&mut opts, &prog);
    
    
    let ini_cfg = Ini::load_from_file(rsrc_path.to_owned() + "/prac.conf").unwrap();
    let ini_str = intro::ini_to_str(&ini_cfg);
    
    let json_str = match intro::read_file(&[rsrc_path, "/prac.json"].concat()) {
		Ok(s) => s,
		Err(_) => String::new()
	};
    let json_data : Value = serde_json::from_str(&json_str).unwrap();
    let json_root = json_data.as_object().unwrap();
    let json_user1 = json_root["user1"].as_object().unwrap();
    
    let toml_str = intro::read_file(&[rsrc_path, "/prac.toml"].concat()
		).unwrap_or(String::new());
    let toml_data : Value = toml::from_str(&toml_str).unwrap();
    let toml_root = toml_data.as_object().unwrap();
    let toml_user1 = toml_root["user1"].as_object().unwrap();
    
    let yaml_str = intro::read_file(&[rsrc_path, "/prac.yaml"].concat()
		).unwrap_or(String::new());
	let yaml_vec = &YamlLoader::load_from_str(&yaml_str).unwrap();
	let yaml_user1 = &yaml_vec[0]["user1"];
    
    let row_arr = [
		[ini_str.as_str(),
			ini_cfg.get_from(Some("default"), "domain").unwrap(),
			ini_cfg.get_from(Some("user1"), "name").unwrap()],
		[json_str.as_str(),
			json_root["domain"].as_str().unwrap(),
			json_user1["name"].as_str().unwrap()],
		[toml_str.as_str(),
			toml_root["domain"].as_str().unwrap(),
			toml_user1["name"].as_str().unwrap()],
		[yaml_str.as_str(),
			yaml_vec[0]["domain"].as_str().unwrap(),
			yaml_user1["name"].as_str().unwrap()]
    ];
    
    for row in row_arr.iter().cloned() {
		println!("config: {}", row[0]);
		println!("domain: {}", row[1]);
		println!("user1Name: {}\n", row[2]);
	}
    
    run_intro(prog, opts.name);
    
    //info!(":main() {}:{}:{} - exiting", file!(), line!(), column!());
    info!(":main() - exiting");
}
