/*
 * Interrupts
 */

#include <regdef.h>
#include <cp0defs.h>

.section .text
.set noreorder
.set nomips16

# enable multi-vectored interrupts
#
# void mips_enable_mv_irq(void)
#
.global mips_enable_mv_irq
.ent mips_enable_mv_irq
mips_enable_mv_irq:
        mfc0    t0,_CP0_CAUSE           # set CAUSE IV bit
        li      t1,_CP0_CAUSE_IV_MASK
        or      t0,t0,t1
        mtc0    t0,_CP0_CAUSE
        jr      ra
        nop
.end mips_enable_mv_irq

# disable interrupts and return previous status
#
# unsigned mips_di(void)
#
.global mips_di
.ent mips_di
mips_di:
        di      v0
        ehb
        jr      ra
        nop
.end mips_di

# enable interrupts and return previous status
#
# unsigned mips_ei(void)
#
.global mips_ei
.ent mips_ei
mips_ei:
        jr      ra
        ei      v0
.end mips_ei

# restore interrupts based on saved status
#
# void mips_restore_irq(unsigned previous_status)
#
.global mips_restore_irq
.ent mips_restore_irq
mips_restore_irq:
        mfc0    t0,_CP0_STATUS          # the IE bit is a position 0
        ins     t0,a0,0,1
        mtc0    t0,_CP0_STATUS
        ehb
        jr      ra
        nop
.end mips_restore_irq

