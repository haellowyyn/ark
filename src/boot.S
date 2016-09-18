.global _start
_start:
	bl init_stack
	bl enable_fpu

	bl rust_main


init_stack:
	/* sp alignment: last 4 bits must always be 0b0000 [armv8-arm D1.8.2] */
	ldr x0, =0x500
	mov sp, x0
	ret


enable_fpu:
	mrs x0, CPACR_EL1
	orr x0, x0, #(0x3 << 20)
	msr CPACR_EL1, x0
	isb
	ret
