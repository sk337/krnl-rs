use core::arch::asm;

pub static PIC0_DATA: u16 = 0x20; // PIC0_DATA
pub static PIC0_CTRL: u16 = 0x21; // PIC0_CTRL

pub static PIC1_DATA: u16 = 0xa0; // PIC1_DATA
pub static PIC1_CTRL: u16 = 0xa1; // PIC1_CTRL

pub fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!(
            "inb dx, al",    // Direct register usage
            in("dx") port,
            out("al") value    // Output goes to `al`
        );
    }
    value
}

pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!(
            "outb al, dx",   // Direct register usage
            in("al") data,      // Input data from `al`
            in("dx") port
        );
    }
}

pub fn inw(port: u16) -> u16 {
    let value: u16;
    unsafe {
        asm!(
            "inw dx, ax",    // Direct register usage
            in("dx") port,
            out("ax") value    // Output goes to `ax`
        );
    }
    value
}

pub fn outw(port: u16, data: u16) {
    unsafe {
        asm!(
            "outw ax, dx",   // Direct register usage
            in("dx") port,
            in("ax") data      // Input data from `ax`
        );
    }
}

pub fn inl(port: u16) -> u32 {
    let value: u32;
    unsafe {
        asm!(
            "inl dx, eax",   // Direct register usage
            in("dx") port,
            out("eax") value   // Output goes to `eax`
        );
    }
    value
}

pub fn outl(port: u16, data: u32) {
    unsafe {
        asm!(
            "outl eax, dx",  // Direct register usage
            in("dx") port,
            in("eax") data     // Input data from `eax`
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

// Delay function to simulate I/O timing (to be implemented as needed)
