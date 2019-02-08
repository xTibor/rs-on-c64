#![feature(lang_items, core_intrinsics)]
#![feature(start)]
#![no_std]
use core::intrinsics;

extern crate libc;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        let border_color = 0xD020 as *mut u8;
        let background_color = 0xD021 as *mut u8;

        let screen_ptr = 0x0400 as *mut u8;
        let color_ptr = 0xD800 as *mut u8;

        *border_color = 0x0A;
        *background_color = 0x00;

        core::intrinsics::write_bytes(screen_ptr, 0x20, 1000);

        let text = b"hello from rust";
        let mut n = 0;

        loop {
            let position = n * 40 + n;

            core::intrinsics::copy(text.as_ptr(), screen_ptr.offset(position), text.len());
            core::intrinsics::write_bytes(color_ptr.offset(position), *border_color, text.len());

            n = (n + 1) % 25;

            let i = 0;
            while i < 63 {
                *border_color += 1;
                i += 1;
            }
        }
    }
    0
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}
