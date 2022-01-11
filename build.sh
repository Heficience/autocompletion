# build the autocompletion files

mkdir -p build

cargo build --release --bin autocompletion --all-targets --target-dir build
