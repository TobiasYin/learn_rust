.globl _start
_start:
  .code16

  # clean registry
  xorw    %ax, %ax
  movw    %ax, %ds
  movw    %ax, %es
  movw    %ax, %ss

  movw     0xb800，%ax
  movw     %ax, %es
  movw     byte[es:0],0x68
  movw     byte[es:1],0x00