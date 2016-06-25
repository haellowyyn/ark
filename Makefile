CC=aarch64-linux-gnu-gcc
CFLAGS=-ansi -pedantic -Wall -Wextra -fPIC
LD=aarch64-linux-gnu-ld
LDFLAGS=-N -Ttext=0x10000 --gc-sections

LOADER=boot.o
RUST_LIB=target/aarch64-unknown-linux-gnu/debug/libark.a

kernel: $(LOADER) $(RUST_LIB)
	$(LD) $(LDFLAGS) -o $@ $(LOADER) $(RUST_LIB)

$(LOADER): src/boot.S
	$(CC) $(CFLAGS) -o $@ -c $<

$(RUST_LIB):
	cargo build --target=aarch64-unknown-linux-gnu

clean:
	cargo clean
	rm -f $(LOADER) kernel

run: kernel
	qemu-system-aarch64 -M versatilepb -cpu cortex-a57 \
	 	-nographic \
		-kernel $<
