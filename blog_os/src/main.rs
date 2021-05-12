// We simply start by adding a no_std to current module.
//
// This gives following errors -
// 1. println! not found (understandable)
// 2. #[panic_handler] function required but not found.
// 3. Language item required but not found `eh_personality`.
#![no_std]
//
//
//
// This is required to add lang_item `eh_personality`
#![feature(lang_items)]
//
//
//
// No main, we define our 'own' entrypoint
#![no_main]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
#[no_mangle]
fn rust_eh_personality() {}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
