#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = vga_buffer::WRITER.lock();
    writeln!(writer, "Hello from rust_tiny_os!").unwrap();
    writeln!(writer, "Press Ctrl+C to quit QEMU (or close the window)").unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = vga_buffer::WRITER.lock();
    writeln!(writer, "PANIC: {}", info).ok();
    loop {}
}
