.global _start
_start:
	bl init_stack
	bl enable_fpu

	bl rust_main

init_stack:
	ldr x0, =0x500
	mov sp, x0
	ret


enable_fpu:
	mrs x0, cpacr_el1
	orr x0, x0, #(0x3 << 20)
	msr cpacr_el1, x0
	isb
	ret
