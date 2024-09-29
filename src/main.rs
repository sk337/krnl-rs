#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
use std::x86_64::asm;
// mod vga_buffer;

use core::panic::PanicInfo;
// use std::backtrace

// #[link_section = ".multiboot"]
// #[no_mangle]
// pub static MULTIBOOT_HEADER: [u32; 4] = [
//     0x1BADB002,                    // Magic number
//     0x0,                           // Flags
//     -(0x1BADB002i32 + 0x0) as u32, // Checksum
//     0x0,                           // Reserved
// ];

asm!(
    "section .multiboot_header
header_start:
    dd 0xe85250d6                ; magic number (multiboot 2)
    dd 0                         ; architecture 0 (protected mode i386)
    dd header_end - header_start ; header length
    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; insert optional multiboot tags here

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:"
);

static HELLO: &[u8] = b"Hello World!";

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
