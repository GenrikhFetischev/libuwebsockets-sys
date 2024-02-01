use std::env;
use std::path::PathBuf;
use std::process::Command;

use crate::utils::{copy_by_condition, create_folder_if_not_exists};

pub fn build_uv() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let cur_dir = env::current_dir().unwrap();
  let uv_dir = cur_dir.join("libuv");
  let uv_build_dir = uv_dir.join("build");


  create_folder_if_not_exists(&uv_build_dir).expect("Can't create build folder for libuv!");

  if !Command::new("cmake")
    .current_dir(&uv_dir)
    .arg("--build")
    .arg("build")
    .status()
    .expect("Failed to execute the cmake build").success() {
    panic!("Can't build libuv!");
  }


  copy_by_condition(uv_build_dir, &out_dir, |filename| filename.ends_with("libuv.a"));
  println!("cargo:rustc-link-lib=uv");
}
