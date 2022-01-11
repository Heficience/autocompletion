# build the autocompletion files


mkdir -p build
cargo build --release --target-dir build --target x86_64-unknown-linux-gnu
cargo build --release --target-dir build --target x86_64-pc-windows-gnu

