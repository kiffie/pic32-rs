/*
 * wrapper for interrupt handlers that saves registers on the stack
 *
 * address of interrupt handler to be called must be in register k0
 *
 */

#include <regdef.h>
#include <cp0defs.h>

.section .text.isr_context, "ax"
.set noreorder
.set noat
.set nomips16
.weak _isr_context
.ent _isr_context

_isr_context:

        addiu sp, sp, -96
        mfc0    k1,_CP0_EPC
        sw      k1,84(sp)
        sw      t0,32(sp)

        mfc0    k1,_CP0_CAUSE           # set IRQ priority (IPL) to RIPL
        mfc0    t0,_CP0_STATUS
        sw      t0,88(sp)
        srl     k1,k1,10                # extract RIPL from cause register
        ins     t0,k1,10,6              # insert it into IPL field
        ins     t0,zero,1,4             # clear EXL, ERL, UM
#if 0
        ins     t0,zero,29,1            # disable COP1 (i.e, FPU)
#endif
        mtc0    t0,_CP0_STATUS          # IRQs are now enabled again

        sw      $1,4(sp)
        sw      v0,8(sp)
        sw      v1,12(sp)
        sw      a0,16(sp)
        sw      a1,20(sp)
        sw      a2,24(sp)
        sw      a3,28(sp)
        sw      t1,36(sp)
        sw      t2,40(sp)
        sw      t3,44(sp)
        sw      t4,48(sp)
        sw      t5,52(sp)
        sw      t6,56(sp)
        sw      t7,60(sp)
        sw      t8,64(sp)
        sw      t9,68(sp)
        sw      ra,72(sp)
        mflo    t0
        sw      t0,76(sp)
        mfhi    t0
        sw      t0,80(sp)

        jalr    k0
        nop

        lw t0,  80(sp)
        mthi    t0
        lw      t0,76(sp)
        mtlo    t0
        lw      at_reg,4(sp)
        lw      v0,8(sp)
        lw      v1,12(sp)
        lw      a0,16(sp)
        lw      a1,20(sp)
        lw      a2,24(sp)
        lw      a3,28(sp)
        lw      t0,32(sp)
        lw      t1,36(sp)
        lw      t2,40(sp)
        lw      t3,44(sp)
        lw      t4,48(sp)
        lw      t5,52(sp)
        lw      t6,56(sp)
        lw      t7,60(sp)
        lw      t8,64(sp)
        lw      t9,68(sp)
        lw      ra,72(sp)

        di
        ehb
        lw      k1,84(sp)
        mtc0    k1,_CP0_EPC
        lw      k1,88(sp)
        mtc0    k1,_CP0_STATUS
        addiu   sp,sp,96
        eret
.end _isr_context
