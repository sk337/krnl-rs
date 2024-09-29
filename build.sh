#!/bin/bash

cargo build --target x86_64.json

mkdir -p iso/boot/grub

cp grub.cfg iso/boot/grub

cp target/x86_64/debug/bootimage-krnl-rs.bin iso/boot/kernel.bin

grub2-mkrescue -o krnl-rs.iso iso

rm -r iso
