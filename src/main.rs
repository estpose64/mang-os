// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // don't link the Rust standard library

use core::panic::PanicInfo;

mod vga_buffer;

// don't mangle the name of this function
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, World!");
    println!("Fuck Microsoft!");
    println!("Long Live Linux!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
