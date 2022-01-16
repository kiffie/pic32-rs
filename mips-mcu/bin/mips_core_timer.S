/*
 * Core timer (CPO count and compare registers)
 */

#include <regdef.h>
#include <cp0defs.h>

.section .text
.set noreorder
.set nomips16

# Read count register
#
# uint32_t mips_read_cp0_count()
#
.global mips_read_cp0_count
.ent mips_read_cp0_count
mips_read_cp0_count:
        jr      ra
        mfc0    v0,_CP0_COUNT
.end mips_read_cp0_count

# Write count register
#
# void mips_write_cp0_count(uint32_t count)
#
.global mips_write_cp0_count
.ent mips_write_cp0_count
mips_write_cp0_count:
        jr      ra
        mtc0    a0,_CP0_COUNT
.end mips_write_cp0_count

# Read compare register
#
# uint32_t mips_read_cp0_compare(void)
#
.global mips_read_cp0_compare
.ent mips_read_cp0_compare
mips_read_cp0_compare:
        jr      ra
        mfc0    v0,_CP0_COMPARE
.end mips_read_cp0_compare

# Write compare register
#
# void mips_write_cp0_compare(uint32_t compare)
#
.global mips_write_cp0_compare
.ent mips_write_cp0_compare
mips_write_cp0_compare:
        jr      ra
        mtc0    a0,_CP0_COMPARE
.end mips_write_cp0_compare
