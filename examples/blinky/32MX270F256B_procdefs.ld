/*************************************************************************
 * Processor-specific object file for PIC32MX170 and PIC32MX270
 *************************************************************************/

/*************************************************************************
 * Symbols used for interrupt-vector table generation
 *************************************************************************/
PROVIDE(_vector_spacing = 0x0001);
PROVIDE(_ebase_address = 0x9D000000);  /* first 4 KiB of program flash */


MEMORY
{
    boot_flash          (rx)    : ORIGIN = 0xBFC00000, LENGTH = 0xc00
    program_flash       (rx)    : ORIGIN = 0x9D000000, LENGTH = 256k
    sram                (w!x)   : ORIGIN = 0x80000000, LENGTH = 64k
    configsfrs                  : ORIGIN = 0xBFC00BF0, LENGTH = 0x10
}

REGION_ALIAS("exception_mem", program_flash)
REGION_ALIAS("program_mem", program_flash)
REGION_ALIAS("data_mem", sram)

/* aliases for direct start without bootloader
 * put the reset handler into the boot flash.
 */
REGION_ALIAS(reset_mem, boot_flash)

/* aliases for bootloader support
 * put the bootloader into the boot flash section and the reset handler at the
 * beginning of the normal program flash memory.
 */
/* REGION_ALIAS(reset_mem, program_flash)
REGION_ALIAS(bootloader_mem, boot_flash) */

/*************************************************************************
 * common part for all PIC32 devices
 *************************************************************************/
INPUT("device.x") /* interrupt vector symbols from Peripheral Access Crate */
INPUT("pic32_common.ld")
