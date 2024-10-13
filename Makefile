.POSIX:
.PHONY: clean run run-iso all

# Config Variables

ARCH = x64
# ARCH = x86
# TODO: Add support for ARM
# MODE = release
MODE = debug


ifeq ($(ARCH), x64)
    ASM_ARCH = elf64
    RUST_TARGET = x86_64-unknown-none
    LINKER_ARCH = elf_x86_64
    QEMU = qemu-system-x86_64
    BOOTLOADER = bootloader64.o
    EFI_FORMAT = x86_64-efi
    EFI_BOOT = BOOTX64.EFI
else
    ASM_ARCH = elf32
    RUST_TARGET = i686-unknown-linux-gnu
    LINKER_ARCH = elf_i386
    QEMU = qemu-system-i386
    BOOTLOADER = bootloader.o
    EFI_FORMAT = i386-efi
    EFI_BOOT = BOOTIA32.EFI
endif


ifeq ($(MODE), release)
    MODE_ALIAS = release
    MODE_FLAG = --release
else
    MODE_ALIAS = debug
    MODE_FLAG =
endif


ASFLAGS = -f $(ASM_ARCH) -g
CXX := $(shell if command -v clang++ > /dev/null; then echo clang++; else echo g++; fi)
MKISO := $(shell if command -v grub2-mkrescue > /dev/null; then echo grub2-mkrescue; else echo grub-mkrescue; fi)
MKEFI := $(shell if command -v grub2-mkstandalone > /dev/null; then echo grub2-mkstandalone; else echo grub-mkstandalone; fi)

QEMU_FLAGS = -m 256M

LINKER_SCRIPT = linker.ld
ISO_DATA_DIR = iso/boot
ISO_DIR = iso
OBJ = $(BOOTLOADER) main.o
LD_FLAGS = -m $(LINKER_ARCH) -nostdlib -T $(LINKER_SCRIPT)

all: main.iso create-live-iso

%.o: asm/%.asm
	nasm $(ASFLAGS) $< -o $@

main.o: $(wildcard src/*.rs) .cargo/config.toml Cargo.toml
	cargo +nightly build $(MODE_FLAG) --target $(RUST_TARGET)
	cp target/$(RUST_TARGET)/$(MODE_ALIAS)/libkrnl_rs.a main.o

main.elf: $(OBJ)
	ld $(LD_FLAGS) -o $@ $(OBJ)

$(EFI_BOOT):
	$(MKEFI) -O $(EFI_FORMAT) -o $(EFI_BOOT) "boot/grub/grub.cfg=grub.cfg"

main.iso: main.elf $(EFI_BOOT)
	mkdir -p $(ISO_DATA_DIR)
	cp $< $(ISO_DATA_DIR)
	rm -rf $(ISO_DIR)/EFI
	mkdir -p $(ISO_DIR)/EFI/BOOT
	cp ./$(EFI_BOOT) $(ISO_DIR)/EFI/BOOT/$(EFI_BOOT)
	$(MKISO) -o $@ $(ISO_DIR)

clean:
	rm -f *.o *.elf *.d *.iso

run: main.elf
	$(QEMU) $(QEMU_FLAGS) -kernel $<

run-iso: main.iso
	$(QEMU) $(QEMU_FLAGS) -cdrom $<

create-live-iso: main.iso
	mv main.iso hummus_$(ARCH)_$(MODE_ALIAS).iso
