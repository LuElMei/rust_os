
// we cant use the std lib, which requires an OS to work
#![no_std]

// we cant use the main, which requires work do be done beforehand
// like the C-Runtime, which builds the rust runtime
#![no_main]

// our own test runner because
// rusts test framework requires the std lib
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;


use core::panic::PanicInfo;

// we dont use the std lib, so we have to define our own panic handler
// so the system knows what do to during a panic, eg.:
// when index is out of bounds, unwrap() on None or Err, Div by Zero etc
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

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


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
