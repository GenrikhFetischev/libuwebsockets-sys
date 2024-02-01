use std::env;
use std::path::PathBuf;
#[cfg(feature = "uws_vendored")]
use std::process::Command;

use crate::utils::{copy_by_condition, read_dir_by_condition};

pub fn build_uwebsockets() {
  let host = env::var("HOST").unwrap();
  let target = env::var("TARGET").unwrap();
  let is_windows = host.contains("windows") && target.contains("windows");
  if is_windows {
    panic!("Windows is not currently supported");
  }

  let is_apple = host.contains("apple") && target.contains("apple");
  let is_linux = host.contains("linux") && target.contains("linux");


  let cur_dir = env::current_dir().unwrap();

  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let uws_dir = cur_dir.join("uWebSockets");
  let us_dir = uws_dir.join("uSockets");
  let capi_dir = uws_dir.join("capi");

  // Remove libuwebsockets.a built by make because to my understanding it has packed in a wrong way

  if !Command::new("make")
    .current_dir(&capi_dir)
    .arg("capi")
    .status()
    .expect("Failed to run make")
    .success()
  {
    panic!("Can't build UWS CAPI!");
  }

  let _ = Command::new("rm").current_dir(&capi_dir).arg("-f").arg("libuwebsockets.a").status().expect("failed to delete lib").success();

  let mut o_files = read_dir_by_condition(&us_dir, |file_name| file_name.ends_with(".o"));
  let success = if is_apple {
    let mut args = vec![
      "-static".to_string(),
      "-o".to_string(),
      capi_dir.join("libuwebsockets.a").to_str().unwrap().to_string(),
      capi_dir.join("libuwebsockets.o").to_str().unwrap().to_string(),
    ];
    args.append(&mut o_files);

    Command::new("libtool")
      .current_dir(&capi_dir)
      .args(args)
      .status()
      .expect("Failed to run libtool")
      .success()
  } else if is_linux {
    let mut args = vec![
      "rcs".to_string(),
      "-o".to_string(),
      capi_dir.join("libuwebsockets.a").to_str().unwrap().to_string(),
      capi_dir.join("libuwebsockets.o").to_str().unwrap().to_string(),
    ];
    args.append(&mut o_files);

    Command::new("ar")
      .current_dir(&out_dir)
      .args(args)
      .status()
      .expect("Failed to pack libuwebsockets")
      .success()
  } else { false };

  if !success {
    panic!("Failed to pack libuwebsockets.a");
  }

  copy_by_condition(capi_dir, &out_dir, |filename| filename.ends_with(".a"));


  println!("cargo:rustc-link-lib=uwebsockets");
}

