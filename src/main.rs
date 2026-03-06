// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // don't link the Rust standard library

use core::panic::PanicInfo;

mod vga_buffer;

// don't mangle the name of this function
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    vga_buffer::print_stuff();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
