#![no_std]
extern crate alloc;
mod allocer;
mod io;
mod prelude;
mod traits;
mod vga_buffer;

use allocer::BumpAllocator; // Import the bump allocator
use core::arch::asm;
use core::panic::PanicInfo;
use prelude::*; // Import the prelude
use vga_buffer::{Buffer, Color, ColorCode, Status, Writer};

#[global_allocator]
static GLOBAL_ALLOCATOR: BumpAllocator = BumpAllocator::new(0x_4444_0000, 1024 * 1024 * 16); // Example heap location and size

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

    _info.location().map(|loc| {
        writer.write_string("Location: ");
        writer.write_string(loc.file());
        writer.write_string(":");
        writer.write_number(loc.line().to_string());
        writer.write_string(":");
        writer.write_number(loc.column().to_string());
        writer.new_line();
    });

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
    writer.println("HUMMUS OS :3");
    writer.set_color(Color::Black, Color::DarkGray);
    writer.println("Harder to install than Arch Linux! ;)");

    writer.print_status(Status::OK, "Kernel loaded successfully!");
    writer.print_status(Status::INFO, "Did you know hummus is yummy?");
    writer.print_status(Status::ERROR, "I'm out of hummus :(");
    writer.print_status(Status::DEBUG, "I foundz it! :D");

    let str = String::from("Hello, world!");

    writer.println(&str.as_str());

    // panic!("Test Panic");

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
