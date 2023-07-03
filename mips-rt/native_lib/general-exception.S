/*
 * General exception wrapper
 * Saves all registers on the stack for easy error reporting
 *
 * signature of the general exception handler:
 *    pub extern "C" fn _general_exception_handler(cp0_cause: u32, cp0_status: u32);
 *
 */

#include <regdef.h>
#include <cp0defs.h>

.extern _general_exception_handler
.section .text.general_exception, "ax"
.set noreorder
.set noat
.set nomips16
.globl _general_exception_context
.ent _general_exception_context

_general_exception_context:

  # Save off the non-callee saved registers that may get mucked with
   addiu sp, sp, -88
   sw $1,  4(sp)     // 1
   sw v0,  8(sp)     // 2
   sw v1, 12(sp)     // 3
   sw a0, 16(sp)     // 4
   sw a1, 20(sp)     // 5
   sw a2, 24(sp)     // 6
   sw a3, 28(sp)     // 7
   sw t0, 32(sp)     // 8
   sw t1, 36(sp)     // 9
   sw t2, 40(sp)     // 10
   sw t3, 44(sp)     // 11
   sw t4, 48(sp)     // 12
   sw t5, 52(sp)     // 13
   sw t6, 56(sp)     // 14
   sw t7, 60(sp)     // 15
                     // 16-23 s0-s7, callee saved
   sw t8, 64(sp)     // 24
   sw t9, 68(sp)     // 25
                     // 26-27 kernel use
                     // 28 Global pointer (GP)
                     // 29 Stack pointer (SP)
                     // 30 Frame Pointer (FP) or s8 (callee saved)
   sw ra, 72(sp)     // 31
   mflo t0
   sw t0, 76(sp)
   mfhi t0
   sw t0, 80(sp)

#if defined(__PIC__)
   lw      t9,%call16(_general_exception_handler)(gp)
   nop
   # Pass Cause and Status to the handler function
   mfc0 a0, _CP0_CAUSE
   mfc0 a1, _CP0_STATUS
   jalr    t9
   nop
#else
   la      k0,_general_exception_handler
   nop
   
   # Pass Cause and Status to the handler function
   mfc0 a0, _CP0_CAUSE
   mfc0 a1, _CP0_STATUS
   jalr    k0
   nop
#endif

   lw t0, 80(sp)
   mthi t0
   lw t0, 76(sp)
   mtlo t0

   lw $1,  4(sp)
   lw v0,  8(sp)
   lw v1, 12(sp)
   lw a0, 16(sp)
   lw a1, 20(sp)
   lw a2, 24(sp)
   lw a3, 28(sp)
   lw t0, 32(sp)
   lw t1, 36(sp)
   lw t2, 40(sp)
   lw t3, 44(sp)
   lw t4, 48(sp)
   lw t5, 52(sp)
   lw t6, 56(sp)
   lw t7, 60(sp)
   lw t8, 64(sp)
   lw t9, 68(sp)
   lw ra, 72(sp)
   addiu sp, sp, 88

   ehb
   eret

  .end _general_exception_context

.weak _general_exception_handler
.ent _general_exception_handler
_general_exception_handler:
        jr      ra
        nop
.end _general_exception_handler
