ENTRY(_start);

SECTIONS {
    . = 0x100000; /* Kernel entry point at 1MB */

    .text : {
        *(.multiboot)
        *(.text.*)
        *(.text)
    }

    .rodata : {
        *(.rodata.*)
        *(.rodata)
    }

    .data : {
        *(.data.*)
    }

    .bss : {
        *(.bss.*)
    }
}
