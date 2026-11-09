
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
mod serial;


use core::panic::PanicInfo;

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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // create a new port at iobase 
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion...");
    assert_eq!(1, 0);
    serial_println!("[ok]");
}
