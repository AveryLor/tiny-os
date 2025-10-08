# rust-tiny-os

Tiny educational OS written in Rust. Boots in QEMU and prints to VGA text buffer.

## Prerequisites

- Rust (stable)
- `cargo install bootimage` (see https://github.com/rust-osdev/bootimage)
- `rustup target add x86_64-unknown-none`
- QEMU (qemu-system-x86_64)

## Build & Run

```bash
cargo bootimage --target x86_64-blog_os.json
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os.json/debug/bootimage-rust_tiny_os.bin
```

// NOTES
// - This is a compact starter to play around with printing to the screen and handling panics.
// - It intentionally uses `no_std` and a simple VGA writer. You can expand it: add interrupts, IDT, GDT, keyboard, basic heap, or paging.
// - If you get linker/bootloader version errors, check the `bootimage` and `bootloader` crate versions and see Philipp Oppermann's "Writing an OS in Rust" guide for matching versions: https://os.phil-opp.com/

// End of project files
