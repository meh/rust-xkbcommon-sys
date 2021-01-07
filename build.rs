use std::env;

extern crate pkg_config;
use pkg_config::{Config, Error};

fn is_static() -> bool {
	env::var("CARGO_FEATURE_STATIC").is_ok()
}

fn common() -> Result<(), Error> {
	if let Ok(path) = env::var("XKBCOMMON_LIB_DIR") {
		for lib in &["xkbcommon"] {
			println!("cargo:rustc-link-lib={}={}", if is_static() { "static" } else { "dylib" }, lib);
		}

		println!("cargo:rustc-link-search=native={}", path);
	}
	else {
		try!(Config::new().statik(is_static()).probe("xkbcommon"));
	}

	Ok(())
}

fn x11() -> Result<(), Error> {
	if env::var("CARGO_FEATURE_X11").is_ok() {
		if let Ok(path) = env::var("XKBCOMMON_LIB_DIR") {
			for lib in &["xkbcommon-x11"] {
				println!("cargo:rustc-link-lib={}={}", if is_static() { "static" } else { "dylib" }, lib);
			}

			println!("cargo:rustc-link-search=native={}", path);
		}
		else {
			try!(Config::new().statik(is_static()).probe("xkbcommon-x11"));
		}
	}

	Ok(())
}

fn main() {
	common().unwrap();
	x11().unwrap();
}
