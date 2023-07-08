/*
 * Memory region of PIC32MX devices
 *
 * LENGTH values need to be adapted to specific device variant.
 * REGION_ALIAS functions are used to put the startup code either into the boot
 * flash memory or into the program flash memory keeping the boot flash free for
 * a boot loader.
 */

/* Symbols used for interrupt-vector table generation */
PROVIDE(_vector_spacing = 0x0001);
PROVIDE(_ebase_address = 0x9D000000);  /* first 4 KiB of program flash */

MEMORY
{
    boot_flash          (rx)    : ORIGIN = 0xBFC00000, LENGTH = 3k - 0x10
    program_flash       (rx)    : ORIGIN = 0x9D000000, LENGTH = 128k
    sram                (w!x)   : ORIGIN = 0x80000000, LENGTH = 32k
    configsfrs                  : ORIGIN = ORIGIN(boot_flash) + LENGTH(boot_flash), LENGTH = 0x10
}

REGION_ALIAS("exception_mem", program_flash)
REGION_ALIAS("program_mem", program_flash)
REGION_ALIAS("data_mem", sram)

/* aliases for direct start without bootloader
 * put the reset handler into the boot flash.
 */
REGION_ALIAS(reset_mem, boot_flash)

/* aliases for bootloader support
 * put the bootloader into the boot flash memory and the reset handler at the
 * beginning of the normal program flash memory.
 */
/* REGION_ALIAS(reset_mem, program_flash)
REGION_ALIAS(bootloader_mem, boot_flash) */
