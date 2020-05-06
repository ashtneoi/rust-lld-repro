#![no_std]
#![feature(lang_items)]
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
        foo_dummy(bar_dummy(STUFF[argc as usize])) as isize
    }
}

#[lang = "eh_personality"]
fn personality() { }

extern "C" {
    fn foo_dummy(sth: u8) -> u8;
    fn bar_dummy(sth: u8) -> u8;
}

global_asm!(
    r#"
        .section foo, "ax"
        foo_dummy:
            ret

        .section bar, "ax"
        bar_dummy:
            ret
    "#;
);
