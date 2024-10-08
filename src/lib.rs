#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// use core::arch::asm;
mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{Buffer, Color, ColorCode, Writer};

static HELLO: &[u8] = b"Hello World!";

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // let message = "Kernel panic!".;
    // message.co
    // let mut writer = Writer {
    //     column_position: 0,
    //     color_code: ColorCode::new(Color::White, Color::Black),
    //     buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    // };

    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.clear_screen();

    loop {}
}
