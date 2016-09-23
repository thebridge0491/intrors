//! Document module

use libc::{c_int, c_long, c_float};

//#[link(name = "intro_c-practice")]
extern {
	pub fn fact_i(n: c_int) -> c_long;
	pub fn fact_lp(n: c_int) -> c_long;
	
	pub fn expt_i(b: c_float, n: c_float) -> c_float;
	pub fn expt_lp(b: c_float, n: c_float) -> c_float;
}
