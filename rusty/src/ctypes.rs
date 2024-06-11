#![allow(non_camel_case_types, unused)]

pub use alloc::ffi::*;
pub use core::ffi::*;

pub type CResult = Result<(), c_int>;

pub trait ToResult {
  fn to_result(&self) -> CResult;
}

impl ToResult for c_int {
  fn to_result(&self) -> CResult {
    if self.is_positive() {
      Err(*self)
    } else {
      Ok(())
    }
  }
}
