#![no_std]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_use]
extern crate alloc;

mod allocator;
mod ctypes;
mod panic_handler;
mod print;

#[no_mangle]
pub extern "C" fn hello_from_rust() {
  println!("Hello from Rust!");
}
