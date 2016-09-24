CC=aarch64-linux-gnu-gcc
CFLAGS=-ansi -pedantic -Wall -Wextra -fPIC
LD=aarch64-linux-gnu-ld
LDFLAGS=-N -Ttext=0x10000 --gc-sections

LIBARK=target/aarch64-unknown-linux-gnu/release/libark.a
LIBARK_dbg=target/aarch64-unknown-linux-gnu/debug/libark.a

### release build

ark: boot.o $(LIBARK)
	$(LD) $(LDFLAGS) -o $@ $^

boot.o: src/boot.S
	$(CC) $(CFLAGS) -o $@ -c $^

# Bogus .FORCE prequisite ensures cargo is invoked on every build.
.FORCE:
$(LIBARK): .FORCE
	cargo build --release

.PHONY: run
run: ark
	qemu-system-aarch64 -M versatilepb -cpu cortex-a57 \
		-serial stdio \
		-kernel $< \

### debug build

ark.dbg: boot.o $(LIBARK_dbg)
	$(LD) $(LDFLAGS) -o $@ $^

$(LIBARK_dbg): .FORCE
	cargo build

.PHONY: run.dbg
run.dbg: ark.dbg
	qemu-system-aarch64 -M versatilepb -cpu cortex-a57 \
		-serial stdio \
		-kernel $< \


.PHONY: clean
clean:
	cargo clean
	rm -f boot.o ark ark.dbg
