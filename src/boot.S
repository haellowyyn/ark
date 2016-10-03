/* This is the kernel loader. It sets up everything needed for the kernel
 * to start running:
 *     - It sets up rudimentary address translation and switches on the MMU.
 *     - It jumps to high virtual memory at 0xffffff8000000000, were the
 *       kernel is going to run.
 *     - It initializes the kernel stack.
 *     - It switches on the FPU.
 */

.section .text

.global _entry
_entry:
	/* MMU initialization:
	 *
	 * We set both TCR_EL1.T0SZ and TCR_EL1.T1SZ to 25. This reduces the
	 * available virtual address space (to 1024 GB) but also makes address
	 * translation start at level 1 instead of level 0 [armv8-arm 4.2.5].
	 * As level-1 tables can map 1-GB pages, this enables us to set up
	 * initial address translation using a single-page translation table.
	 *
	 * By pointing both TTBR0_EL1 and TTBR1_EL1 to this translation table,
	 * we map the first GB of physical memory into the virtual memory space
	 * at both 0x0000000000000000 and 0xffffff8000000000. This way, the
	 * instruction fetches directly after switching on the MMU don't fail
	 * and we can safely jump to the high virtual memory.
   *
	 * Note that the physical memory size of the virtualpb is only at most
	 * 256 MB. But mapping a whole GB should be fine as long as we don't
	 * try to access VAs that map to the higher PAs.
	 */

	/* Set TCR_EL1, so address translation starts at level 1. */
  ldr x0, =0x500190019
	msr TCR_EL1, x0

  /* Set required entries in the boot translation table:
   * - first entry identity maps the first 1 GB of physical memory
   * - last entry points to the beginning of the translation table, enabling
   *   recusive mapping [http://wiki.osdev.org/Page_Tables#Recursive_mapping]
   *
   * We must subtract 0xffffff8000000000 from the VA of the boot translation
	 * table to optain its PA.
   */
  ldr x1, =(_boot_translation_table - 0xffffff8000000000)
  /* First entry:
   * - last two bits = 01 -> block descriptor
   * - set access flag to prevent AF faults
   */
  ldr x0, =0x0000000000000401
  str x0, [x1, 0]
  /* Last entry:
   * - last two bits = 11 -> table descriptor, page descriptor at level 3
   * - set access flag to prevent AF faults at level 3
   */
  ldr x2, =0x403
  orr x0, x1, x2
  str x0, [x1, 511 * 8]

	/* Point TTBR0_EL1 and TTBR1_EL1 to the boot translation table. */
	msr TTBR0_EL1, x1
	msr TTBR1_EL1, x1

	/* Switch on the MMU. */
	mrs x0, SCTLR_EL1
	orr x0, x0, 0x1
	msr SCTLR_EL1, x0

	/* Jump to the high virtual memory. */
  ldr x0, =_high_mem
	br x0


_high_mem:
	/* Set up the kernel stack. */
	ldr x0, =_stack_top
	mov sp, x0

	/* Enable the FPU. */
	mrs x0, CPACR_EL1
	orr x0, x0, #(0x3 << 20)
	msr CPACR_EL1, x0
	isb

	/* Enter the kernel. */
	b kernel


.section .bss

/* This is the translation table used for initially loading the kernel
 * at VA 0xffffff8000000000. Entries are filled during MMU initialization. */
.balign 0x1000
  .lcomm _boot_translation_table, 0x1000

/* The kernel stack and its guard pages. */
.global _stack_top
.global _stack_bottom

.balign 0x1000
	.lcomm _guard_stack_bottom, 0x1000
	.lcomm _stack_bottom, 0x1000
	.lcomm _stack_top, 0
	.lcomm _guard_stack_top, 0x1000
