
// we cant use the std lib, which requires an OS to work
#![no_std]

// we cant use the main, which requires work do be done beforehand
// like the C-Runtime, which builds the rust runtime
#![no_main]

// our own test runner because
// rusts test framework requires the std lib
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;


use core::panic::PanicInfo;


// our own entry point, which uses the c calling convention
// because rusts is unstable
// no mangle ensures the name stays the same
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}", "!");

    #[cfg(test)]
    test_main();

    loop {};
}

// we dont use the std lib, so we have to define our own panic handler
// so the system knows what do to during a panic, eg.:
// when index is out of bounds, unwrap() on None or Err, Div by Zero etc
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
