/*********************************************************************
 *
 *                  Runtime Startup (modified for Rust)
 *
 *********************************************************************
 * Filename:        crt0.S
 *
 * Processor:       PIC32
 *
 * Compiler:        chipKIT
 
 * Company:         Microchip Technology Inc.
 *
 * Software License Agreement
 *
 * Copyright (c) 2014, Microchip Technology Inc. and its subsidiaries ("Microchip")
 * All rights reserved.
 *
 * This software is developed by Microchip Technology Inc. and its
 * subsidiaries ("Microchip").
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1.      Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *
 * 2.      Redistributions in binary form must reproduce the above copyright
 * notice, this list of conditions and the following disclaimer in the
 * documentation and/or other materials provided with the distribution.
 *
 * 3.      Microchip's name may not be used to endorse or promote products
 * derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY MICROCHIP "AS IS" AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL
 * MICROCHIP BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING BUT NOT LIMITED TO
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA OR PROFITS;
 * OR BUSINESS INTERRUPTION) HOWSOEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 * OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
 * ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 ********************************************************************/

#include <regdef.h>
#include <cp0defs.h>


/* This file contains 32-bit assembly code */
       .set nomips16

        ##################################################################
        # Entry point of the entire application
        ##################################################################
        .section .reset, "ax"
        .align 2
        .set noreorder
        .ent _reset

############################
# Begin ISA switching code #
############################

#if defined (__mips_micromips)
        .set micromips
#endif

#if (defined(__PIC32_HAS_MICROMIPS)) && (defined(__PIC32_HAS_MIPS32R2))
_reset:
        .word 0x10000003     /* MIPS32:    branch forward 0x10 bytes from here  */
                             /* MicroMIPS: ADDI32 $0, $0, 0x0007 (nop)          */
                             /* DO NOT change the relative branch               */

        .word 0x00000000     /* NOP */
__reset_micromips_isa:
        .set    micromips
        jal     _startup
        nop

        .align 2
        /* Device not in proper ISA mode */
        .set nomicromips
__reset_switch_isa:
        jal _startup
        nop

#else

_reset:
        la      t0,_startup
        jalr    t0
        nop

#endif  /* __PIC32_HAS_MICROMIPS */

        .align 2
        .end _reset
        .globl _reset
        .size _reset, .-_reset

        .section .reset.startup, "ax"
        .align 2
        .set noreorder

#if defined (__mips_micromips)
        .set micromips
#else
        .set nomicromips
#endif

############################
# End ISA switching code   #
############################


        ##################################################################
        # Startup code
        ##################################################################
        .align 2
        .set noreorder
        .ent _startup
_startup:
        ##################################################################
        # If entered because of an NMI, jump to the NMI handler.
        ##################################################################
        mfc0    k0,_CP0_STATUS
        ext     k0,k0,19,1              # Extract NMI bit
        beqz    k0,_no_nmi
        nop
#        la      k0,_nmi_handler
#        jr      k0
        nop
_no_nmi:

        ##################################################################
        # Initialize Stack Pointer
        #   _stack is initialized by the linker script to point to the
        #    starting location of the stack in DRM
        ##################################################################
        la      sp,_stack

        ##################################################################
        # Initialize Global Pointer
        #   _gp is initialized by the linker script to point to "middle"
        #   of the small variables region
        ##################################################################
#        la      gp,_gp


        ##################################################################
        # Call the "on reset" procedure
        ##################################################################
#        la      t0,_on_reset
#        jalr    t0
        nop

#if defined(INIT_MMU_MZ_FIXED) || defined(__PIC32_HAS_MMU_MZ_FIXED)
        ##################################################################
        # Initialize TLB for fixed mapping to EBI and SQI
        ##################################################################
        .extern __pic32_tlb_init_ebi_sqi
        la      t0,__pic32_tlb_init_ebi_sqi
        jalr    t0
        nop
#endif

#if defined(INIT_L1_CACHE) || defined(__PIC32_HAS_L1CACHE)
        ##################################################################
        # Initialize L1 cache register
        ##################################################################
        .extern __pic32_init_cache
        la      t0,__pic32_init_cache
        jalr    t0
        nop
#endif


        ##################################################################
        # Initialize CP0 registers
        ##################################################################
        # Initialize Count register
        ##################################################################
        mtc0    zero,_CP0_COUNT

        ##################################################################
        # Initialize Compare register
        ##################################################################
        li      t2,-1
        mtc0    t2,_CP0_COMPARE

        ##################################################################
        # Ensure BEV set and Initialize EBase register
        ##################################################################
        li      t0, (1<<22)
        mfc0    t2,_CP0_STATUS
        or      t2,t0,t2               # Set BEV bit 22
        mtc0    t2,_CP0_STATUS

        la      t1,_ebase_address
        ehb
        mtc0    t1,_CP0_EBASE

        ##################################################################
        # Initialize IntCtl/INTCON.VS register with _vector_spacing
        ##################################################################
        la      t1,_vector_spacing
#if defined(INIT_INTCONVS) || defined(__PIC32_HAS_INTCONVS)
        la      t0, INTCON
        lw      t2, 0(t0)
        li      t2, 0
        ins     t2, t1, 16, 7
#if defined(__PIC32MM) && defined(_INTCON_MVEC_MASK)
        ori     t2, t2, _INTCON_MVEC_MASK
#endif
        sw      t2, 0(t0)
#endif
        li      t2,0                    # Clear t2 and
        ins     t2,t1,5,5               # shift value to VS field
        mtc0    t2,_CP0_INTCTL

        ##################################################################
        # Initialize CAUSE registers
        # - Enable counting of Count register <DC = 0>
        # - Use special exception vector <IV = 1>
        # - Clear pending software interrupts <IP1:IP0 = 0>
        ##################################################################
        li      t1,0x00800000
        mtc0    t1,_CP0_CAUSE

        ##################################################################
        # Initialize STATUS register
        # - Access to Coprocessor 0 not allowed in user mode <CU0 = 0>
        # - User mode uses configured endianness <RE = 0>
        # - Preserve Bootstrap Exception vectors <BEV>
        # - Preserve soft reset <SR> and non-maskable interrupt <NMI>
        # - CorExtend enabled based on whether CorExtend User Defined
        #   Instructions have been implemented <CEE = Config<UDI>>
        # - Disable any pending interrupts <IM7..IM2 = 0, IM1..IM0 = 0>
        # - Disable hardware interrupts <IPL7:IPL2 = 0>
        # - Base mode is Kernel mode <UM = 0>
        # - Error level is normal <ERL = 0>
        # - Exception level is normal <EXL = 0>
        # - Interrupts are disabled <IE = 0>
        # - DSPr2 ASE is enabled for devices that support it <MX = 1>
        # - FPU64 is enabled for devices that support it <CU1=1> & <FR=1>
        ##################################################################
        mfc0    t0,_CP0_CONFIG
        ext     t1,t0,22,1              # Extract UDI from Config register
        sll     t1,t1,17                # Move UDI to Status.CEE location
        mfc0    t0,_CP0_STATUS
        and     t0,t0,0x00580000        # Preserve SR, NMI, and BEV
#if defined(INIT_DSPR2) || defined(__PIC32_HAS_DSPR2)
        li      t2, 0x01000000          # Set the Status.MX bit to enable DSP
        or      t0,t2,t0
#endif
#if defined(INIT_FPU64) || defined(__PIC32_HAS_FPU64)
        li      t2, 0x24000000          # Set the Status.CU1 and Status.FR bits to
        or      t0,t2,t0                # enable the FPU in FR64 mode
#endif

        or      t0,t1,t0                # Include Status.CEE (from UDI)
        mtc0    t0,_CP0_STATUS

#if defined(INIT_FPU64) || defined(__PIC32_HAS_FPU64)
                                        # FPU Control and Status
        li      t2,0x1000000            # FCSR: RM=0, FS=1, FO=0, FN=0
                                        # Enables: 0b00000 E=1, V=0, Z=0, O=0, U=0, I=0
        ctc1    t2, $31                 # High perf on denormal operands & tiny results
#endif
        ehb

        ##################################################################
        # Call the "on bootstrap" procedure
        ##################################################################
#        la      t0,_on_bootstrap
#        jalr    t0
        nop

        ##################################################################
        # Initialize Status<BEV> for normal exception vectors
        ##################################################################
        mfc0    t0,_CP0_STATUS
        and     t0,t0,0xffbfffff        # Clear BEV
        mtc0    t0,_CP0_STATUS

        ##################################################################
        # Call main. We do this via a thunk in the text section so that
        # a normal jump and link can be used, enabling the startup code
        # to work properly whether main is written in MIPS16 or MIPS32
        # code. I.e., the linker will correctly adjust the JAL to JALX if
        # necessary
        ##################################################################
        and     a0,a0,0
        and     a1,a1,0
        la      t0,_main_entry
        jr      t0
        nop

        .end _startup

        ##################################################################
        # Boot Exception Vector Handler
        # Jumps to _bootstrap_exception_handler
        ##################################################################
        .section .bev_handler, "ax"
        .align 2
        .set noreorder
        .ent _bev_exception
_bev_exception:
        #la        k0,_bootstrap_exception_handler
        jr        k0
        nop

        .end _bev_exception

#if defined(INIT_MMU_MZ_FIXED) || defined(__PIC32_HAS_MMU_MZ_FIXED)
        ##################################################################
        # Simple TLB-Refill Exception Vector
        # Jumps to _simple_tlb_refill_exception_context
        ##################################################################
        .section .simple_tlb_refill_vector,code,keep
        .align 2
        .set noreorder
        .ent simple_tlb_refill_vector
simple_tlb_refill_vector:
        la      k0,_simple_tlb_refill_exception_context
        jr      k0
        nop

        .end simple_tlb_refill_vector
#endif

#if defined(INIT_L1_CACHE) || defined(__PIC32_HAS_L1CACHE)
        ##################################################################
        # Cache-Error Exception Vector Handler
        # Jumps to _cache_err_exception_context
        ##################################################################
        .section .cache_err_vector,code,keep
        .align 2
        .set noreorder
        .ent _cache_err_vector
_cache_err_vector:
        la      k0,_cache_err_exception_context
        jr      k0
        nop

        .end _cache_err_vector
#endif

        .section .text.main_entry, "ax"
        .align 2
        .ent _main_entry
_main_entry:

        and     a0,a0,0
        and     a1,a1,0

        ##################################################################

        # Call main
        ##################################################################
        la    	t0,rust_reset
        jalr 	t0
        nop

#if defined(CALL_EXIT)
        ##################################################################
        # Call exit()
        ##################################################################
        jal exit
        nop
#endif

        ##################################################################
        # Just in case, go into infinite loop
        # Call a software breakpoint only with -mdebugger compiler option
        ##################################################################
        .weak __exception_handler_break
__crt0_exit:
1:
        la      v0,__exception_handler_break
        beq     v0,0,0f
        nop
        jalr    v0
        nop

0:      b       1b
        nop

        .globl __crt0_exit
        .end _main_entry
