CC=aarch64-linux-gnu-gcc
CFLAGS=-ansi -pedantic -Wall -Wextra -fPIC
LD=aarch64-linux-gnu-ld
LDFLAGS=-N -Ttext=0x10000 --gc-sections

LIBARK=target/aarch64-unknown-linux-gnu/debug/libark.a

ark: boot.o $(LIBARK)
	$(LD) $(LDFLAGS) -o $@ $^

boot.o: src/boot.S
	$(CC) $(CFLAGS) -o $@ -c $^

# Bogus .FORCE prequisite ensures cargo is invoked on every build.
.FORCE:
$(LIBARK): .FORCE
	cargo build

.PHONY: clean
clean:
	cargo clean
	rm -f boot.o ark

.PHONY: run
run: ark
	qemu-system-aarch64 -M versatilepb -cpu cortex-a57 \
		-serial stdio \
		-kernel $< \
