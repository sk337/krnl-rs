#![no_std]

// use core::arch::asm;
mod io;
mod vga_buffer;

use core::arch::asm;
use core::panic::PanicInfo;
use vga_buffer::{Buffer, Color, ColorCode, Status, Writer};

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut writer = Writer {
        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.clear_screen();
    writer.print_status(Status::ERROR, "Kernel panic!");
    writer.println("----PANIC----");
    writer.write_string("Error: ");
    writer.write_string(_info.message().as_str().unwrap_or_else(|| "no message"));
    writer.new_line();
    if _info.location().is_some() {
        writer.write_string("Location: ");
        writer.new_line();
        writer.write_string("  ");
        writer.write_string(_info.location().unwrap().file());
        writer.write_string("@");
        writer.write_number(_info.location().unwrap().line() as u64);
        writer.write_string(":");
        writer.write_number(_info.location().unwrap().column() as u64);
        writer.new_line();
    };

    loop {
        unsafe {
            asm!("hlt");
        }
    }
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
    writer.print_status(Status::INFO, "Did you know hummus is yummy?");
    writer.print_status(Status::ERROR, "I'm out of hummus :(");

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
