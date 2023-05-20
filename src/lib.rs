#[allow(warnings)]
#[allow(clippy::all)]
pub mod bindings {
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
