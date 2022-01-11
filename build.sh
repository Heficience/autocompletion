# build the autocompletion files
rmdir build/
mkdir -p build/

cargo build --release --bin autocompletion --all-targets --target-dir build/
