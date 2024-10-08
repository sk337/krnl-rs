#!/bin/bash
sudo apt install qemu-system-x86_64 -y

rustup toolchain install nightly
rustup default nightly
rustup target add x86_64-unknown-none
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
