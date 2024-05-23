::custom_print::define_macros!(#[macro_export] { print, println }, concat, |value: &str| {
  unsafe { crate::printf(value.as_ptr()) };
});

::custom_print::define_macros!(#[macro_export] { eprint, eprintln, dbg }, concat, |value: &str| {
  unsafe {
    crate::printf("error: ".as_ptr());
    crate::printf(value.as_ptr());
  }
});
