use std::env;
use std::path::PathBuf;

#[cfg(feature = "openssl_vendored")]
use crate::build_openssl::build_openssl;
#[cfg(feature = "uv_vendored")]
use crate::build_uv::build_uv;
#[cfg(feature = "uws_vendored")]
use crate::build_uwebsockets::build_uwebsockets;
#[cfg(feature = "z_vendored")]
use crate::build_z::build_z;

#[cfg(feature = "uv_vendored")]
mod build_uv;
#[cfg(feature = "uws_vendored")]
mod build_uwebsockets;
#[cfg(feature = "z_vendored")]
mod build_z;

#[cfg(feature = "openssl_vendored")]
mod build_openssl;

mod utils;

fn main() {
  let host = env::var("HOST").unwrap();
  let target = env::var("TARGET").unwrap();
  let is_windows = host.contains("windows") && target.contains("windows");
  if is_windows {
    panic!("Windows is not currently supported");
  }

  let cur_dir = env::current_dir().unwrap();

  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let uws_dir = cur_dir.join("uWebSockets");
  let us_dir = uws_dir.join("uSockets");
  let capi_dir = uws_dir.join("capi");

  bindgen::Builder::default()
    .clang_arg(format!("-I{}", us_dir.join("src").display()))
    .header(capi_dir.join("libuwebsockets.h").display().to_string())
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Unable to generate bindings")
    .write_to_file(out_dir.join("bindings.rs"))
    .expect("Couldn't write bindings!");

  println!("cargo:rustc-link-search=native={}", out_dir.display());

  #[cfg(feature = "uws_vendored")]
  build_uwebsockets();

  #[cfg(feature = "z_vendored")]
  build_z();

  #[cfg(feature = "uv_vendored")]
  build_uv();

  #[cfg(feature = "openssl_vendored")]
  build_openssl();
}

