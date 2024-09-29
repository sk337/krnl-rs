./build.sh
qemu-system-x86_64 -cdrom krnl-rs.iso
# qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-krnl-rs.bin

# qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/krnl-rs
