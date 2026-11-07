OUTPUT_ARCH( "z80" )

MEMORY
{
  ram : ORIGIN = 0x100, LENGTH = 40K
}

ENTRY(_start)

PHDRS
{
  text PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}

/*
We are now going to organize the memory based on which
section it is in. In assembly, we can change the section
with the ".section" directive. However, in C++ and Rust,
CPU instructions go into text, global constants go into
rodata, global initialized variables go into data, and
global uninitialized variables go into bss.
*/
SECTIONS
{
  /*
    The first part of our RAM layout will be the text section.
  Since our CPU instructions are here, and our memory starts at
  0x8000_0000, we need our entry point to line up here.
  */
  .text : {

    PROVIDE(_text_start = .);
    *(.text.init) *(.text .text.*)
    PROVIDE(_global_pointer = .);
    *(.rodata .rodata.*)
    *(.sdata .sdata.*) *(.data .data.*)
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_text_end = .);

  } >ram AT>ram :text

  PROVIDE(_memory_start = ORIGIN(ram));

  /*PROVIDE(_stack = _bss_end + 0x80000);*/
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));
}

