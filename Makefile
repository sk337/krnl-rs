.POSIX:
.PHONY: clean all

ASFLAGS = -f elf32 -g
CXX := $(shell if command -v clang++ > /dev/null; then echo clang++; else echo g++; fi)
MKISO := $(shell if command -v grub2-mkrescue > /dev/null; then echo grub2-mkrescue; else echo grub-mkrescue; fi)
QEMU = qemu-system-i386
ENTRY = bootloader.asm
LINKER_SCRIPT = linker.ld
ISO_DATA_DIR = iso/boot
ISO_DIR = iso
OBJ = bootloader.o main.o
LD_FLAGS = -m elf_i386 -nostdlib -T $(LINKER_SCRIPT)

all: main.elf

%.o: asm/%.asm
	nasm $(ASFLAGS) $< -o $@

main.o: $(wildcard src/*.rs) .cargo/config.toml Cargo.toml
	cargo build
	cp target/i686-unknown-linux-gnu/debug/libkrnl_rs.so main.o

main.elf: $(OBJ)
	ld $(LD_FLAGS) -o $@ $(OBJ)


clean:
	rm -f *.o *.elf *.d

run: main.elf
	$(QEMU) -kernel $<
