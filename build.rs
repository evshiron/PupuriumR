
use std::env;

pub fn main()
{
	
	//let out_dir = env::var_os("OUT_DIR").unwrap();

	println!("cargo:rustc-link-search=./libs");

}
