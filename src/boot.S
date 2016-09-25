.section .text

.global _entry
_entry:
	bl _init_stack
	bl _enable_fpu

	bl rust_main


_init_stack:
	/* sp alignment: last 4 bits must always be 0b0000 [armv8-arm D1.8.2] */
	ldr x0, =_stack_top
	mov sp, x0
	ret


_enable_fpu:
	mrs x0, CPACR_EL1
	orr x0, x0, #(0x3 << 20)
	msr CPACR_EL1, x0
	isb
	ret


.section .bss

/* kernel stack */
.global _stack_bottom
.global _stack_top

	.lcomm _guard_stack_bottom, 0x1000
	.lcomm _stack_bottom, 0x1000
	.lcomm _stack_top, 0
	.lcomm _guard_stack_top, 0x1000
