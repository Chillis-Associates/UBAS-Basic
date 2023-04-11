#![no_std]  // don't link the rust standard library
#![no_main] // disable all rust-level entry points

use core::panic::PanicInfo;

#[no_mangle]  // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named '_start' by deafult
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}