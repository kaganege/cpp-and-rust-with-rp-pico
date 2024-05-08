::custom_print::define_macros!(#[macro_export] { print, println }, concat, |value: &str| {
  let str = ::core::format_args!("{value}\0").as_str().unwrap();
  let cstr = ::core::ffi::CStr::from_bytes_with_nul(str.as_bytes()).unwrap();
  unsafe { crate::printf(cstr.as_ptr() as _) };
});

::custom_print::define_macros!(#[macro_export] { eprint, eprintln, dbg }, concat, |value: &str| {
  let str = ::core::format_args!("error: {value}\0").as_str().unwrap();
  let cstr = ::core::ffi::CStr::from_bytes_with_nul(str.as_bytes()).unwrap();
  unsafe { crate::printf(cstr.as_ptr() as _) };
});
