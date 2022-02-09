#!/usr/bin/env bash

sudo apt install -y libusb-1.0-0-dev libftdi1-dev libudev-dev libudev0-dev gdb-multiarch

cargo install flip-link --version 0.1.5
cargo install cargo-binutils --version 0.3.3
cargo install probe-run --version 0.3.1
cargo install defmt-print --version 0.3.0
rustup component add llvm-tools-preview rustfmt clippy

sudo cp 99-st-link-v2_1.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
