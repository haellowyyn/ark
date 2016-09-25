ENTRY(_entry)

SECTIONS
{
  /* guard page */
  . = 0x1000;

  _krnl_start = .;

  /* executable code, RX */
  .text : ALIGN(4K) {
    _text_start = .;
    *(.text .text.*)
    _text_end = .;
  }

  /* read-only data, R */
  .rodata : ALIGN(4K) {
    _rodata_start = .;
    *(.rodata .rodata.*)
    *(.gcc_except_table)
    _rodata_end = .;
  }

  /* read-write data, RW */
  .data : ALIGN(4K) {
    _data_start = .;
    *(.data .data.*)
    *(.got) *(.got.plt)
    _data_end = .;
  }

  /* uninitialized data, RW */
  .bss : ALIGN(4K) {
    _bss_start = .;
    *(.bss .bss.*)
    _bss_end = .;
  }

  _krnl_end = .;
}
