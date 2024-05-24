::custom_print::define_macros!(#[macro_export] { print, println }, concat, |value: &str| {
  unsafe { crate::printf(format!("{value}\0").as_ptr()) };
});

::custom_print::define_macros!(#[macro_export] { eprint, eprintln, dbg }, concat, |value: &str| {
  unsafe {
    crate::printf(format!("error: {value}\0").as_ptr());
  }
});
