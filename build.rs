use std::env;
#[cfg(feature = "uws_vendored")]
use std::path::{Path};
use std::path::{PathBuf};
use std::process::Command;

fn main() {
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

  bindgen::Builder::default()
    .clang_arg(format!("-I{}", us_dir.join("src").display()))
    .header(capi_dir.join("libuwebsockets.h").display().to_string())
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings")
    .write_to_file(out_dir.join("bindings.rs"))
    .expect("Couldn't write bindings!");


  println!("cargo:rustc-link-lib=z");
  println!("cargo:rustc-link-lib=uv");
  println!("cargo:rustc-link-lib=ssl");
  println!("cargo:rustc-link-lib=crypto");


  if is_apple {
    println!("cargo:rustc-link-lib=c++");
  } else if is_linux {
    println!("cargo:rustc-link-lib=stdc++");
  }


  // Remove libuwebsockets.a built by make because to my understanding it has packed in a wrong way
  #[cfg(feature = "uws_vendored")]
  {
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

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=uwebsockets");
  }
}

#[cfg(feature = "uws_vendored")]
fn copy_by_condition<T>(from: impl AsRef<Path>, to: impl AsRef<Path>, condition: T)
  where
    T: Fn(&str) -> bool + Sized + 'static,
{
  use std::fs;
  let entries = from.as_ref().read_dir().unwrap();
  for entry in entries {
    let entry = entry.unwrap();
    if entry.file_type().unwrap().is_dir() {
      continue;
    }

    let file_name = entry.file_name();

    if !condition(file_name.to_str().unwrap()) {
      continue;
    }
    let from = entry.path();
    let to = to.as_ref().join(file_name);
    println!("copy from {from:#?} to: {to:#?}");
    fs::copy(&from, &to).unwrap();
  }
}


#[cfg(feature = "uws_vendored")]
fn read_dir_by_condition<T>(dir: &Path, condition: T) -> Vec<String>
  where
    T: Fn(&str) -> bool + Sized + 'static,
{
  let mut res = Vec::new();
  let entries = dir.read_dir().unwrap();
  for entry in entries {
    let entry = entry.unwrap();
    if entry.file_type().unwrap().is_dir() {
      continue;
    }

    let file_name = entry.file_name();
    let file_name = file_name.to_str().unwrap();

    if !condition(file_name) {
      continue;
    }

    res.push(dir.join(file_name).to_str().unwrap().to_string());
  }

  res
}

