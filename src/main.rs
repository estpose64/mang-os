// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // don't link the Rust standard library

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello, World!";

// don't mangle the name of this function
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	// this function is the entry point, since the linker looks for a function
	// named `_start` by default
	let vga_buffer = 0xb8000 as *mut u8;

	for (i, &byte) in HELLO.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0x2;
		}
	}

	loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop{}
}
