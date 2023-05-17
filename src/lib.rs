#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(warnings)]
pub mod bindings {
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
