all:
	cargo xb
	cargo objcopy -- --strip-all -O binary target/riscv32imac-unknown-none-elf/debug/rust-sipeed-longan-nano rust-sipeed-longan-nano.bin

release:
	cargo xb --release
	cargo objcopy -- --strip-all -O binary target/riscv32imac-unknown-none-elf/release/rust-sipeed-longan-nano rust-sipeed-longan-nano.bin

