/*
 * Wrapper for exception handlers that saves registers on the stack
 *
 * Address of exception handler to be called must be in register k0
 * This wrapper is not for interrupt exceptions.
 *
 * signature of the exception handler called by this wrapper:
 *    pub extern "C" fn _general_exception_handler(cp0_cause: u32, cp0_status: u32);
 *
 */

#include <regdef.h>
#include <cp0defs.h>

.section .text.exception_context, "ax"
.set noreorder
.set noat
.set nomips16
.globl _exception_context
.ent _exception_context

_exception_context:

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

   # Pass Cause and Status to the handler function
   mfc0 a0, _CP0_CAUSE
   mfc0 a1, _CP0_STATUS
   jalr    k0
   nop

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

  .end _exception_context
