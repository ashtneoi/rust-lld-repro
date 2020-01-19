#![no_std]
#![feature(global_asm)]
#![feature(start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop { }
}

#[no_mangle]
fn abort() -> ! {
    loop { }
}

static STUFF: &[u8] = b"here's a string that we're going to read from";

#[no_mangle]
#[start]
fn _start(argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        bar(STUFF[argc as usize])
    }
}

extern "C" {
    fn bar(sth: u8) -> isize;
}

global_asm!(
    r#"
        .section foo, "ax"
        bar:
            ret
    "#;
);
