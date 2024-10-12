.POSIX:
.PHONY: clean run run-iso all

ASFLAGS = -f elf32 -g
CXX := $(shell if command -v clang++ > /dev/null; then echo clang++; else echo g++; fi)
MKISO := $(shell if command -v grub2-mkrescue > /dev/null; then echo grub2-mkrescue; else echo grub-mkrescue; fi)
QEMU = qemu-system-i386
# QEMU = qemu-system-x86_64
QEMU_FLAGS = -m 256M
LINKER_ARCH = elf_i386
# LINKER_ARCH = elf_x86-64
TARGET = i686-unknown-linux-gnu
# TARGET = x86_64-unknown-none
ENTRY = bootloader.asm
LINKER_SCRIPT = linker.ld
ISO_DATA_DIR = iso/boot
ISO_DIR = iso
OBJ = bootloader.o main.o
LD_FLAGS = -m $(LINKER_ARCH) -nostdlib -T $(LINKER_SCRIPT)

all: main.iso

%.o: asm/%.asm
	nasm $(ASFLAGS) $< -o $@

main.o: $(wildcard src/*.rs) .cargo/config.toml Cargo.toml
	cargo build --target $(TARGET)
	cp target/$(TARGET)/debug/libkrnl_rs.a main.o

main.elf: $(OBJ)
	ld $(LD_FLAGS) -o $@ $(OBJ)

main.iso: main.elf
	mkdir -p $(ISO_DATA_DIR)
	cp $< $(ISO_DATA_DIR)
	$(MKISO) -o $@ $(ISO_DIR)

clean:
	rm -f *.o *.elf *.d *.iso

run: main.elf
	$(QEMU) $(QEMU_FLAGS) -kernel $<

run-iso: main.iso
	$(QEMU) $(QEMU_FLAGS) -cdrom $<
