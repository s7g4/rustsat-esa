/* Linker script for the QEMU lm3s6965evb machine (Cortex-M3) */
MEMORY
{
  /* 256KB Flash */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  /* 64KB RAM */
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}
