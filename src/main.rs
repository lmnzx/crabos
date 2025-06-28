#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello from rust!!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    //
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("hiii\n").unwrap();
    // write!(
    //     vga_buffer::WRITER.lock(),
    //     "{} / {} = {}\nThis is on the next line",
    //     3.0,
    //     2.0,
    //     3.0 / 2.0
    // )
    // .unwrap();

    println!("helllo {}", "world");

    panic!("ooh no");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
