# tiny-os
My own tiny os.

A minimal x86 operating system that boots and displays a greeting message.

## Features
- Bootloader written in x86 assembly
- Displays "Hello from tiny-os!" on boot
- 512-byte boot sector

## Requirements
- NASM assembler
- QEMU (optional, for testing)

## Building

To build the OS image:
```bash
make
```

This will create `tiny-os.img`, a bootable disk image.

## Running

### With QEMU
```bash
make run
```

### With other emulators
You can use the `tiny-os.img` file with any x86 emulator or virtual machine that supports booting from a raw disk image.

## Cleaning

To remove build artifacts:
```bash
make clean
```

## How it works

The OS consists of a single boot sector (512 bytes) that:
1. Gets loaded by BIOS at address 0x7c00
2. Uses BIOS interrupt 0x10 (teletype output) to print characters
3. Enters an infinite loop after printing the message
4. Contains the boot signature (0xaa55) at bytes 510-511
