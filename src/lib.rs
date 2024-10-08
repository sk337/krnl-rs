#![no_std]

// use core::arch::asm;
mod io;
mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{Buffer, Color, ColorCode, Status, Writer};

static HELLO: &str = "Hello World!";

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // let message = "Kernel panic!".;
    // message.concat
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
        row_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.clear_screen();

    writer.set_color(Color::Black, Color::Pink);
    writer.println("HUMMUS :3");

    writer.print_status(Status::OK, "Kernel loaded successfully!");
    writer.println(HELLO);

    loop {}
}
