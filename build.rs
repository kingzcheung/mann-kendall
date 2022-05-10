extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let file_name = "cbindgen.toml";
    let config = cbindgen::Config::from_file(file_name).unwrap();

    cbindgen::Builder::new().with_config(config)
      .with_crate(crate_dir)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("examples/cpp/mann_kendall.h");
}