#![allow(non_camel_case_types, unused)]

pub use alloc::ffi::*;
pub use core::ffi::*;

pub type CResult<T = ()> = Result<T, c_int>;

pub trait ToResult {
  type Type;

  fn to_result(&self) -> CResult<Self::Type>;
}

impl ToResult for c_int {
  type Type = ();

  fn to_result(&self) -> CResult<Self::Type> {
    if self.is_positive() {
      Err(*self)
    } else {
      Ok(())
    }
  }
}
