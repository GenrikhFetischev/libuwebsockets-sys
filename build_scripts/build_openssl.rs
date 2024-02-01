use std::env;
use std::path::PathBuf;
use std::process::Command;

use crate::utils::copy_by_condition;

pub fn build_openssl() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let cur_dir = env::current_dir().unwrap();
  let openssl_dir = cur_dir.join("openssl");

  if !Command::new("./config")
    .current_dir(&openssl_dir)
    .arg("no-shared")
    .arg("no-quic")
    .arg("no-docs")
    .arg("no-tests")
    .arg("no-async")
    .arg("no-dynamic-engine")
    .status()
    .expect("Failed to run configure for openssl")
    .success()
  {
    panic!("Can't configure openssl build!");
  }
  if !Command::new("make")
    .current_dir(&openssl_dir)
    .status()
    .expect("Failed to run make for openssl")
    .success()
  {
    panic!("Can't build openssl!");
  }
  copy_by_condition(openssl_dir, &out_dir, |filename| filename.ends_with(".a"));
  println!("cargo:rustc-link-lib=crypto");
  println!("cargo:rustc-link-lib=ssl");
}
