extern crate pkg_config;

fn main() {
	/*let prefix = (&env::var("PREFIX").unwrap_or_else(|_| {
		String::from("/usr/local")})).to_owned();
	let libdir = match pkg_config::Config::get_variable("intro_c-practice", "libdir") {
		Ok(s) => s,
		Err(_) => String::from(prefix + "/lib") //String::new()
	};
	//println!("cargo:rustc-flags=-L {} -l {}", libdir, "intro_c-practice");
	println!("cargo:rustc-link-search={}\ncargo:rustc-link-lib={}",
		libdir, "intro_c-practice");*/

	pkg_config::Config::new().probe("intro_c-practice").unwrap();
}
