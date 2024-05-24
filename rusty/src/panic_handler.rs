use crate::*;
use core::panic::PanicInfo;

const AIRCR_REGISTER: *mut u32 = (0xe0000000u32 + 0x0ED0C) as _;

unsafe fn hardware_reset() {
  *AIRCR_REGISTER = 0x5FA0004;
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
  if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
    eprintln!("panic occurred: {s:?}");
  } else {
    eprintln!("panic occurred");
  }
  println!("Rebooting...");
  unsafe { hardware_reset() };
  loop {}
}
