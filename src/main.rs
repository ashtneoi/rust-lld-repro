#![no_std]
#![feature(global_asm)]
#![feature(start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop { }
}

#[no_mangle]
#[start]
fn _start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

global_asm!(
    r#"
        .section foo
        bar:
            j bar
    "#;
);
