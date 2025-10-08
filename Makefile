.PHONY: all run build


all: build


build:
cargo bootimage --target x86_64-blog_os.json


run: build
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os.json/debug/bootimage-rust_tiny_os.bin