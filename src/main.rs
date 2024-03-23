#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
pub mod interrupts;
pub mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    interrupts::init_idt();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); // new

    println!("It did not crash!");
    loop {}
}
