CC=aarch64-linux-gnu-gcc
CFLAGS=-ansi -pedantic -Wall -Wextra -fPIC
LD=aarch64-linux-gnu-ld
LDFLAGS=-T linker.ld --gc-sections


ark: ark.release
run: run.release

ark-dbg: ark.debug
run-dbg: run.debug

ark.%: boot.o libark.%
	$(LD) $(LDFLAGS) -o $@ $< target/aarch64-unknown-linux-gnu/$*/libark.a

boot.o: src/boot.S
	$(CC) $(CFLAGS) -o $@ -c $^

# Bogus .FORCE prequisite ensures cargo is invoked on every build.
.FORCE:
libark.release: .FORCE
	cargo build --release

libark.debug: .FORCE
	cargo build

.PHONY: run
run.%: ark.%
	qemu-system-aarch64 -M versatilepb -cpu cortex-a57 \
		-m 256M \
		-serial stdio \
		-kernel $<

.PHONY: clean
clean:
	cargo clean
	rm -f boot.o ark.release ark.debug
