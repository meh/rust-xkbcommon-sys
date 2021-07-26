use std::{env, path::PathBuf};
use pkg_config::{Config, Error};

fn main() {
	common().unwrap();
	x11().unwrap();

	let mut builder = bindgen::Builder::default()
		.rustified_enum("*")
		.prepend_enum_name(false)
		.derive_eq(true)
		.size_t_is_usize(true);

	builder = builder
		.header("src/wrapper.h");

	if env::var("CARGO_FEATURE_X11").is_ok() {
		builder = builder.header("src/wrapper-x11.h");
	}

  // Finish the builder and generate the bindings.
  builder
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings")
		.write_to_file(output().join("bindings.rs"))
		.unwrap();
}

fn is_static() -> bool {
	env::var("CARGO_FEATURE_STATIC").is_ok()
}

fn output() -> PathBuf {
	PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn common() -> Result<(), Error> {
	if let Ok(path) = env::var("XKBCOMMON_LIB_DIR") {
		for lib in &["xkbcommon"] {
			println!("cargo:rustc-link-lib={}={}", if is_static() { "static" } else { "dylib" }, lib);
		}

		println!("cargo:rustc-link-search=native={}", path);
	}
	else {
		Config::new().statik(is_static()).probe("xkbcommon")?;
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
			Config::new().statik(is_static()).probe("xkbcommon-x11")?;
		}
	}

	Ok(())
}
