TARGET=x86-bare-metal
cargo build --target=$TARGET.json -Z build-std=core --release
PREFIX=target/$TARGET/release
gobjcopy -S -O binary -j .text $PREFIX/rust-os $PREFIX/core
gobjdump -S $PREFIX/rust-os > $PREFIX/res.asm
dd if=/dev/zero of=$PREFIX/kernel.img count=10000 2> /dev/null
dd if=$PREFIX/core of=$PREFIX/kernel.img conv=notrunc 2> /dev/null
printf "%b" '\x55\xaa' | dd bs=1 seek=510 of=$PREFIX/kernel.img conv=notrunc 2> /dev/null