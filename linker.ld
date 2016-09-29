ENTRY(_entry)

SECTIONS
{
  . = 0xffffff8000000000;

  _krnl_start = .;

  /* executable code, RX */
  .text : AT(ADDR(.text) - 0xffffff8000000000) {
    _text_start = .;
    *(.boot)
    *(.text .text.*)
    _text_end = .;
  }

  /* read-only data, R */
  .rodata ALIGN(4K) : AT(ADDR(.rodata) - 0xffffff8000000000) {
    _rodata_start = .;
    *(.rodata .rodata.*)
    *(.gcc_except_table)
    _rodata_end = .;
  }

  /* read-write data, RW */
  .data ALIGN(4K) : AT(ADDR(.data) - 0xffffff8000000000) {
    _data_start = .;
    *(.data .data.*)
    *(.got) *(.got.plt)
    _data_end = .;
  }

  /* uninitialized data, RW */
  .bss ALIGN(4K) : AT(ADDR(.bss) - 0xffffff8000000000) {
    _bss_start = .;
    *(.bss .bss.*)
    _bss_end = .;
  }

  _krnl_end = .;
}
