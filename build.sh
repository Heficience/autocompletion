# build the autocompletion files


mkdir -p build
rm -f build/linux/*
rm -f build/windows/*
mkdir -p build/linux
mkdir -p build/windows

cargo build --release --bin autocompletion --target-dir build --target x86_64-unknown-linux-gnu
cp autocompletion/build/release/autocompletion build/linux/
# for windows
cargo build --release --bin autocompletion --target-dir build --target x86_64-pc-windows-gnu
cp autocompletion/build/release/autocompletion.exe build/windows/
