#!/bin/bash

echo "Installing dependencies..."
sudo apt install libx11-dev libxdo-dev -y

# build the autocompletion files
mkdir -p build
cargo build --release --target-dir build --target x86_64-unknown-linux-gnu
echo "windows build : BETA!! NOT TESTED"
cargo build --release --target-dir build --target x86_64-pc-windows-gnu

