#![expect(dead_code)]
use core::arch::asm;

pub static PIC0_DATA: u16 = 0x20; // PIC0_DATA
pub static PIC0_CTRL: u16 = 0x21; // PIC0_CTRL

pub static PIC1_DATA: u16 = 0xa0; // PIC1_DATA
pub static PIC1_CTRL: u16 = 0xa1; // PIC1_CTRL

pub fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!(
            "inb {}, {}",
            out(reg_byte) value, // Output value will be stored in AL
            in(reg) port,   // DX will hold the port number
        );
    }
    value
}

pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!(
            "outb {}, {1:x}",
            in(reg_byte) data, // AL will be the data to output
            in(reg) port, // DX will hold the port number
        );
    }
}

pub fn inw(port: u16) -> u16 {
    let value: u16;
    unsafe {
        asm!(
            "inw {0:x}, {1:x}",
            out(reg) value, // Output value will be stored in AX
            in(reg) port,   // DX will hold the port number
        );
    }
    value
}

pub fn outw(port: u16, data: u16) {
    unsafe {
        asm!(
            "outw {0:x}, {1:x}",
            in(reg) data,  // AX will be the data to output
            in(reg) port,  // DX will hold the port number
        );
    }
}

pub fn inl(port: u16) -> u32 {
    let value: u32;
    unsafe {
        asm!(
            "inl {}, {1:x}",
            out(reg) value, // Output value will be stored in EAX
            in(reg) port,    // DX will hold the port number
        );
    }
    value
}

pub fn outl(port: u16, data: u32) {
    unsafe {
        asm!(
            "outl {}, {1:x}",
            in(reg) data,  // EAX will be the data to output
            in(reg) port,   // DX will hold the port number
        );
    }
}

fn io_delay() {
    unsafe {
        asm!("outb al, $0x80", in("al") 0u8); // This is commonly used to cause a delay
    }
}

// Read byte from I/O port with slight delay (p variant)
pub fn inb_p(port: u16) -> u8 {
    let value = inb(port);
    io_delay(); // implement this delay as needed
    value
}

// Read word with delay
pub fn inw_p(port: u16) -> u16 {
    let value = inw(port);
    io_delay();
    value
}

// Read long with delay
pub fn inl_p(port: u16) -> u32 {
    let value = inl(port);
    io_delay();
    value
}

// Write byte with delay
pub fn outb_p(port: u16, data: u8) {
    outb(port, data);
    io_delay();
}

// Write word with delay
pub fn outw_p(port: u16, data: u16) {
    outw(port, data);
    io_delay();
}

// Write long with delay
pub fn outl_p(port: u16, data: u32) {
    outl(port, data);
    io_delay();
}
