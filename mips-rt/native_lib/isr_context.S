/*
 * wrapper for interrupt handlers that saves registers on the stack
 *
 * address of interrupt handler to be called must be in register k0
 *
 * Stack frame layout (min. 104 bytes)
 *
 *   0..15   outgoing argument area for the called handler (o32: callee may
 *           overwrite it, so nothing of ours lives here)
 *  16       $at
 *  20       v0
 *  24       v1
 *  28..40   a0..a3
 *  44..48   t0, t1
 *  52..80   t2..t9
 *  84       ra
 *  88       lo
 *  92       hi
 *  96       EPC
 * 100       Status
 * 104       (not used)
 */

#include <regdef.h>
#include <cp0defs.h>

#define FRAME_SIZE      104

#define SAVE_AT         16
#define SAVE_V0         20
#define SAVE_V1         24
#define SAVE_A0         28
#define SAVE_A1         32
#define SAVE_A2         36
#define SAVE_A3         40
#define SAVE_T0         44
#define SAVE_T1         48
#define SAVE_T2         52
#define SAVE_T3         56
#define SAVE_T4         60
#define SAVE_T5         64
#define SAVE_T6         68
#define SAVE_T7         72
#define SAVE_T8         76
#define SAVE_T9         80
#define SAVE_RA         84
#define SAVE_LO         88
#define SAVE_HI         92
#define SAVE_EPC        96
#define SAVE_STATUS     100

.section .text.isr_context, "ax"
.set noreorder
.set noat
.set nomips16
.weak _isr_context
.ent _isr_context
.frame sp, FRAME_SIZE, ra

_isr_context:

        addiu   sp,sp,-FRAME_SIZE
        mfc0    k1,_CP0_EPC
        sw      k1,SAVE_EPC(sp)
        sw      t0,SAVE_T0(sp)
        sw      t1,SAVE_T1(sp)

        move    t1,k0
        mfc0    k1,_CP0_CAUSE           # set IRQ priority (IPL) to RIPL
        mfc0    t0,_CP0_STATUS
        sw      t0,SAVE_STATUS(sp)
        srl     k1,k1,10                # extract RIPL from cause register
        ins     t0,k1,10,6              # insert it into IPL field
        ins     t0,zero,1,4             # clear EXL, ERL, UM
#if 0
        ins     t0,zero,29,1            # disable COP1 (i.e, FPU)
#endif
        mtc0    t0,_CP0_STATUS          # IRQs are now enabled again

        sw      at_reg,SAVE_AT(sp)
        sw      v0,SAVE_V0(sp)
        sw      v1,SAVE_V1(sp)
        sw      a0,SAVE_A0(sp)
        sw      a1,SAVE_A1(sp)
        sw      a2,SAVE_A2(sp)
        sw      a3,SAVE_A3(sp)
        sw      t2,SAVE_T2(sp)
        sw      t3,SAVE_T3(sp)
        sw      t4,SAVE_T4(sp)
        sw      t5,SAVE_T5(sp)
        sw      t6,SAVE_T6(sp)
        sw      t7,SAVE_T7(sp)
        sw      t8,SAVE_T8(sp)
        sw      t9,SAVE_T9(sp)
        sw      ra,SAVE_RA(sp)
        mflo    t0
        sw      t0,SAVE_LO(sp)
        mfhi    t0
        sw      t0,SAVE_HI(sp)

        jalr    t1
        nop

        lw      t0,SAVE_HI(sp)
        mthi    t0
        lw      t0,SAVE_LO(sp)
        mtlo    t0
        lw      at_reg,SAVE_AT(sp)
        lw      v0,SAVE_V0(sp)
        lw      v1,SAVE_V1(sp)
        lw      a0,SAVE_A0(sp)
        lw      a1,SAVE_A1(sp)
        lw      a2,SAVE_A2(sp)
        lw      a3,SAVE_A3(sp)
        lw      t0,SAVE_T0(sp)
        lw      t1,SAVE_T1(sp)
        lw      t2,SAVE_T2(sp)
        lw      t3,SAVE_T3(sp)
        lw      t4,SAVE_T4(sp)
        lw      t5,SAVE_T5(sp)
        lw      t6,SAVE_T6(sp)
        lw      t7,SAVE_T7(sp)
        lw      t8,SAVE_T8(sp)
        lw      t9,SAVE_T9(sp)
        lw      ra,SAVE_RA(sp)

        di
        ehb
        lw      k1,SAVE_EPC(sp)
        mtc0    k1,_CP0_EPC
        lw      k1,SAVE_STATUS(sp)
        mtc0    k1,_CP0_STATUS
        addiu   sp,sp,FRAME_SIZE
        eret
.end _isr_context
