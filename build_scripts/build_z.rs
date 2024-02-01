use std::env;
use std::path::PathBuf;
use std::process::Command;
use crate::utils::copy_by_condition;

pub fn build_z() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let cur_dir = env::current_dir().unwrap();
  let z_dir = cur_dir.join("zlib");

  if !Command::new("./configure")
    .current_dir(&z_dir)
    .status()
    .expect("Failed to run configure for zlib")
    .success()
  {
    panic!("Can't build libz!");
  }
  if !Command::new("make")
    .current_dir(&z_dir)
    .status()
    .expect("Failed to run make for zlib")
    .success()
  {
    panic!("Can't build libz!");
  }

  copy_by_condition(z_dir, &out_dir, |filename| filename.ends_with("libz.a"));
  println!("cargo:rustc-link-lib=z");
}
