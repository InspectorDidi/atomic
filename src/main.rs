#![feature(const_fn)]
#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;

#[macro_use]
mod vga_buffer;

use core::fmt::Write;

#[no_mangle]
pub extern fn _start() -> ! {
    println!("Hello, {}!", "world");

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! {
    loop {}
}
