# Makefile for tiny-os

# Assembler
AS = nasm
ASFLAGS = -f bin

# Output files
BOOT_BIN = boot.bin
OS_IMAGE = tiny-os.img

# Default target
all: $(OS_IMAGE)

# Build boot sector
$(BOOT_BIN): boot.asm
	$(AS) $(ASFLAGS) $< -o $@

# Create OS image (currently just the boot sector)
$(OS_IMAGE): $(BOOT_BIN)
	cp $(BOOT_BIN) $(OS_IMAGE)

# Run in QEMU emulator
run: $(OS_IMAGE)
	qemu-system-x86_64 -drive format=raw,file=$(OS_IMAGE)

# Clean build artifacts
clean:
	rm -f $(BOOT_BIN) $(OS_IMAGE)

.PHONY: all run clean
