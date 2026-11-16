# rust_os

A small x86_64 operating system kernel written in Rust. It is a learning project to understand what happens below the standard library: no operating system, no C runtime, no `std`.

The project follows Philipp Oppermann's blog series [Writing an OS in Rust](https://os.phil-opp.com/).

## Features

- **Freestanding kernel:** `#![no_std]` and `#![no_main]` with a custom entry point `_start` (C calling convention) and a custom panic handler
- **VGA text mode driver:** a `Writer` for the 80x25 text buffer with colors and newline handling, using volatile writes so the compiler cannot optimize them away
- **`print!` and `println!` macros:** backed by a global `WRITER` (`lazy_static` + spinlock), so panic messages are printed to the screen
- **Serial output:** `serial_print!` and `serial_println!` over the UART at port `0x3F8`, used to report test results to the host terminal
- **Custom test framework:** `#![test_runner]` instead of the standard test harness, which needs `std`. QEMU is closed after the run through the `isa-debug-exit` device, and the exit code reports success or failure
- **Bootable image:** built with the [`bootloader`](https://crates.io/crates/bootloader) crate and the `bootimage` tool, then started in QEMU

## Custom target

`x86_64-rust_os.json` describes the bare-metal machine the kernel is compiled for:

| Setting | Value | Why |
| --- | --- | --- |
| `llvm-target` / `os` | `x86_64-unknown-none` / `none` | no operating system |
| `panic-strategy` | `abort` | no stack unwinding without an OS |
| `disable-redzone` | `true` | interrupts would corrupt the red zone on the stack |
| `features` | `-mmx,-sse,+soft-float` | SIMD registers are not saved on interrupts, so floating point is done in software |
| `linker` | `rust-lld` | the linker that ships with Rust |

## Requirements

- Rust **nightly** with the `rust-src` and `llvm-tools-preview` components (a custom target needs `-Zbuild-std` to compile `core`)
- [`bootimage`](https://crates.io/crates/bootimage)
- [QEMU](https://www.qemu.org/) (`qemu-system-x86_64`)

```sh
rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview --toolchain nightly
cargo install bootimage
```

## Build and run

`.cargo/config.toml` sets the target, `build-std` and `bootimage runner`, so the usual cargo commands work:

```sh
cargo +nightly build     # compile the kernel
cargo +nightly run       # build a bootable image and start it in QEMU
cargo +nightly test      # run the kernel tests in QEMU, results appear in the terminal
```

## Project structure

```
rust_os/
├── .cargo/config.toml    # target, build-std and the bootimage runner
├── Cargo.toml            # dependencies and bootimage test settings
├── x86_64-rust_os.json   # custom bare-metal target
└── src/
    ├── main.rs           # entry point, panic handlers, test runner, QEMU exit
    ├── vga_buffer.rs     # VGA text mode driver and print!/println!
    └── serial.rs         # serial port output for test results
```

## Roadmap

- [x] freestanding binary and minimal kernel
- [x] VGA text output
- [ ] unit tests (framework in place, tests are being written)
- [ ] CPU exceptions and interrupts
- [ ] memory management (paging)

## Credits

Based on the work of [Philipp Oppermann](https://os.phil-opp.com/).
