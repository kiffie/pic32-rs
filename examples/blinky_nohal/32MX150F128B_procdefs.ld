/*************************************************************************
 * Processor-specific object file for PIC32MX150
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
    sram                (w!x)   : ORIGIN = 0x80000000, LENGTH = 32k
    configsfrs                  : ORIGIN = 0xBFC00BF0, LENGTH = 0x10
}

REGION_ALIAS("exception_mem", program_flash)
REGION_ALIAS("program_mem", program_flash)
REGION_ALIAS("data_mem", sram)

/* no bootloader */
REGION_ALIAS("reset_mem", boot_flash)

/* bootloader in boot flash memory */
/* REGION_ALIAS("reset_mem", program_flash)
REGION_ALIAS("bootloader_mem", boot_flash) */

/*************************************************************************
 * common part for all PIC32 devices
 *************************************************************************/
INPUT("device.x") /* interrupt vector symbols from Peripheral Access Crate */
INPUT("pic32_common.ld")
