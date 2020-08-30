/*************************************************************************
 * Processor-specific object file.
 *************************************************************************/

/*************************************************************************
 * Symbols used for interrupt-vector table generation
 *************************************************************************/
PROVIDE(_vector_spacing = 0x0001);
PROVIDE(_ebase_address = 0x9D03F000);  /* last 4 KiB of program flash */

/*************************************************************************
 * Memory Address Equates
 * _RESET_ADDR                    -- Reset Vector or entry point
 * _BEV_EXCPT_ADDR                -- Boot exception Vector
 * _DBG_EXCPT_ADDR                -- In-circuit Debugging Exception Vector
 * _DBG_CODE_ADDR                 -- In-circuit Debug Executive address
 * _DBG_CODE_SIZE                 -- In-circuit Debug Executive size
 * _GEN_EXCPT_ADDR                -- General Exception Vector
 *************************************************************************/
_RESET_ADDR                    = 0xBFC00000;
_BEV_EXCPT_ADDR                = 0xBFC00380;
_DBG_EXCPT_ADDR                = 0xBFC00480;
_DBG_CODE_ADDR                 = 0x9FC00490;
_DBG_CODE_SIZE                 = 0x760;
_GEN_EXCPT_ADDR                = _ebase_address + 0x180;

/*************************************************************************
 * Memory Regions
 *
 * Memory regions without attributes cannot be used for orphaned sections.
 * Only sections specifically assigned to these regions can be allocated
 * into these regions.
 *
 * The Debug exception vector is located at 0x9FC00480.
 *
 * The config_<address> sections are used to locate the config words at
 * their absolute addresses.
 *************************************************************************/


MEMORY
{
  kseg0_program_mem     (rx)  : ORIGIN = 0x9D000000, LENGTH = 0x3F000
/*  kseg0_boot_mem              : ORIGIN = 0x9FC00490, LENGTH = 0x970 */
  exception_mem               : ORIGIN = 0x9D03F000, LENGTH = 0x1000
  kseg1_boot_mem              : ORIGIN = 0xBFC00000, LENGTH = 0x490
  debug_exec_mem              : ORIGIN = 0x0FC00490, LENGTH = 0x760
  kseg1_data_mem       (w!x)  : ORIGIN = 0xA0000000, LENGTH = 0x10000
  sfrs                        : ORIGIN = 0xBF800000, LENGTH = 0x100000
  configsfrs                  : ORIGIN = 0xBFC00BF0, LENGTH = 0x10
}

/*************************************************************************
 * common part for all PIC32 devices
 *************************************************************************/
INPUT("device.x") /* interrupt vector symbols from Peripheral Access Crate */
INPUT("pic32_common.ld")
