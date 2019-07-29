/*
 * Exception table for PIC32
 */

#include <regdef.h>
#include <cp0defs.h>
.set noreorder
.set nomips16

.section .text.default_isr, "ax"
.ent _default_isr_fn
_default_isr_fn:
        jr      ra
        nop
.end _default_isr_fn

# General Exception
.section .gen_handler, "ax"
.ent _gen_exception
.global _gen_exception
_gen_exception:
        la      k0,_general_exception_context
        jr      k0
        nop
.end _gen_exception

# vector 0
.weak _vector_0_fn, _vector_0_context
_vector_0_fn = _default_isr_fn
_vector_0_context  = _isr_context
.section .vector_0, "ax"
.ent _vector_0
_vector_0:
        la  k0,_vector_0_fn
        j   _vector_0_context
        nop
.end _vector_0

# vector 1
.weak _vector_1_fn, _vector_1_context
_vector_1_fn = _default_isr_fn
_vector_1_context  = _isr_context
.section .vector_1, "ax"
.ent _vector_1
_vector_1:
        la  k0,_vector_1_fn
        j   _vector_1_context
        nop
.end _vector_1

# vector 2
.weak _vector_2_fn, _vector_2_context
_vector_2_fn = _default_isr_fn
_vector_2_context  = _isr_context
.section .vector_2, "ax"
.ent _vector_2
_vector_2:
        la  k0,_vector_2_fn
        j   _vector_2_context
        nop
.end _vector_2

# vector 3
.weak _vector_3_fn, _vector_3_context
_vector_3_fn = _default_isr_fn
_vector_3_context  = _isr_context
.section .vector_3, "ax"
.ent _vector_3
_vector_3:
        la  k0,_vector_3_fn
        j   _vector_3_context
        nop
.end _vector_3

# vector 4
.weak _vector_4_fn, _vector_4_context
_vector_4_fn = _default_isr_fn
_vector_4_context  = _isr_context
.section .vector_4, "ax"
.ent _vector_4
_vector_4:
        la  k0,_vector_4_fn
        j   _vector_4_context
        nop
.end _vector_4

# vector 5
.weak _vector_5_fn, _vector_5_context
_vector_5_fn = _default_isr_fn
_vector_5_context  = _isr_context
.section .vector_5, "ax"
.ent _vector_5
_vector_5:
        la  k0,_vector_5_fn
        j   _vector_5_context
        nop
.end _vector_5

# vector 6
.weak _vector_6_fn, _vector_6_context
_vector_6_fn = _default_isr_fn
_vector_6_context  = _isr_context
.section .vector_6, "ax"
.ent _vector_6
_vector_6:
        la  k0,_vector_6_fn
        j   _vector_6_context
        nop
.end _vector_6

# vector 7
.weak _vector_7_fn, _vector_7_context
_vector_7_fn = _default_isr_fn
_vector_7_context  = _isr_context
.section .vector_7, "ax"
.ent _vector_7
_vector_7:
        la  k0,_vector_7_fn
        j   _vector_7_context
        nop
.end _vector_7

# vector 8
.weak _vector_8_fn, _vector_8_context
_vector_8_fn = _default_isr_fn
_vector_8_context  = _isr_context
.section .vector_8, "ax"
.ent _vector_8
_vector_8:
        la  k0,_vector_8_fn
        j   _vector_8_context
        nop
.end _vector_8

# vector 9
.weak _vector_9_fn, _vector_9_context
_vector_9_fn = _default_isr_fn
_vector_9_context  = _isr_context
.section .vector_9, "ax"
.ent _vector_9
_vector_9:
        la  k0,_vector_9_fn
        j   _vector_9_context
        nop
.end _vector_9

# vector 10
.weak _vector_10_fn, _vector_10_context
_vector_10_fn = _default_isr_fn
_vector_10_context  = _isr_context
.section .vector_10, "ax"
.ent _vector_10
_vector_10:
        la  k0,_vector_10_fn
        j   _vector_10_context
        nop
.end _vector_10

# vector 11
.weak _vector_11_fn, _vector_11_context
_vector_11_fn = _default_isr_fn
_vector_11_context  = _isr_context
.section .vector_11, "ax"
.ent _vector_11
_vector_11:
        la  k0,_vector_11_fn
        j   _vector_11_context
        nop
.end _vector_11

# vector 12
.weak _vector_12_fn, _vector_12_context
_vector_12_fn = _default_isr_fn
_vector_12_context  = _isr_context
.section .vector_12, "ax"
.ent _vector_12
_vector_12:
        la  k0,_vector_12_fn
        j   _vector_12_context
        nop
.end _vector_12

# vector 13
.weak _vector_13_fn, _vector_13_context
_vector_13_fn = _default_isr_fn
_vector_13_context  = _isr_context
.section .vector_13, "ax"
.ent _vector_13
_vector_13:
        la  k0,_vector_13_fn
        j   _vector_13_context
        nop
.end _vector_13

# vector 14
.weak _vector_14_fn, _vector_14_context
_vector_14_fn = _default_isr_fn
_vector_14_context  = _isr_context
.section .vector_14, "ax"
.ent _vector_14
_vector_14:
        la  k0,_vector_14_fn
        j   _vector_14_context
        nop
.end _vector_14

# vector 15
.weak _vector_15_fn, _vector_15_context
_vector_15_fn = _default_isr_fn
_vector_15_context  = _isr_context
.section .vector_15, "ax"
.ent _vector_15
_vector_15:
        la  k0,_vector_15_fn
        j   _vector_15_context
        nop
.end _vector_15

# vector 16
.weak _vector_16_fn, _vector_16_context
_vector_16_fn = _default_isr_fn
_vector_16_context  = _isr_context
.section .vector_16, "ax"
.ent _vector_16
_vector_16:
        la  k0,_vector_16_fn
        j   _vector_16_context
        nop
.end _vector_16

# vector 17
.weak _vector_17_fn, _vector_17_context
_vector_17_fn = _default_isr_fn
_vector_17_context  = _isr_context
.section .vector_17, "ax"
.ent _vector_17
_vector_17:
        la  k0,_vector_17_fn
        j   _vector_17_context
        nop
.end _vector_17

# vector 18
.weak _vector_18_fn, _vector_18_context
_vector_18_fn = _default_isr_fn
_vector_18_context  = _isr_context
.section .vector_18, "ax"
.ent _vector_18
_vector_18:
        la  k0,_vector_18_fn
        j   _vector_18_context
        nop
.end _vector_18

# vector 19
.weak _vector_19_fn, _vector_19_context
_vector_19_fn = _default_isr_fn
_vector_19_context  = _isr_context
.section .vector_19, "ax"
.ent _vector_19
_vector_19:
        la  k0,_vector_19_fn
        j   _vector_19_context
        nop
.end _vector_19

# vector 20
.weak _vector_20_fn, _vector_20_context
_vector_20_fn = _default_isr_fn
_vector_20_context  = _isr_context
.section .vector_20, "ax"
.ent _vector_20
_vector_20:
        la  k0,_vector_20_fn
        j   _vector_20_context
        nop
.end _vector_20

# vector 21
.weak _vector_21_fn, _vector_21_context
_vector_21_fn = _default_isr_fn
_vector_21_context  = _isr_context
.section .vector_21, "ax"
.ent _vector_21
_vector_21:
        la  k0,_vector_21_fn
        j   _vector_21_context
        nop
.end _vector_21

# vector 22
.weak _vector_22_fn, _vector_22_context
_vector_22_fn = _default_isr_fn
_vector_22_context  = _isr_context
.section .vector_22, "ax"
.ent _vector_22
_vector_22:
        la  k0,_vector_22_fn
        j   _vector_22_context
        nop
.end _vector_22

# vector 23
.weak _vector_23_fn, _vector_23_context
_vector_23_fn = _default_isr_fn
_vector_23_context  = _isr_context
.section .vector_23, "ax"
.ent _vector_23
_vector_23:
        la  k0,_vector_23_fn
        j   _vector_23_context
        nop
.end _vector_23

# vector 24
.weak _vector_24_fn, _vector_24_context
_vector_24_fn = _default_isr_fn
_vector_24_context  = _isr_context
.section .vector_24, "ax"
.ent _vector_24
_vector_24:
        la  k0,_vector_24_fn
        j   _vector_24_context
        nop
.end _vector_24

# vector 25
.weak _vector_25_fn, _vector_25_context
_vector_25_fn = _default_isr_fn
_vector_25_context  = _isr_context
.section .vector_25, "ax"
.ent _vector_25
_vector_25:
        la  k0,_vector_25_fn
        j   _vector_25_context
        nop
.end _vector_25

# vector 26
.weak _vector_26_fn, _vector_26_context
_vector_26_fn = _default_isr_fn
_vector_26_context  = _isr_context
.section .vector_26, "ax"
.ent _vector_26
_vector_26:
        la  k0,_vector_26_fn
        j   _vector_26_context
        nop
.end _vector_26

# vector 27
.weak _vector_27_fn, _vector_27_context
_vector_27_fn = _default_isr_fn
_vector_27_context  = _isr_context
.section .vector_27, "ax"
.ent _vector_27
_vector_27:
        la  k0,_vector_27_fn
        j   _vector_27_context
        nop
.end _vector_27

# vector 28
.weak _vector_28_fn, _vector_28_context
_vector_28_fn = _default_isr_fn
_vector_28_context  = _isr_context
.section .vector_28, "ax"
.ent _vector_28
_vector_28:
        la  k0,_vector_28_fn
        j   _vector_28_context
        nop
.end _vector_28

# vector 29
.weak _vector_29_fn, _vector_29_context
_vector_29_fn = _default_isr_fn
_vector_29_context  = _isr_context
.section .vector_29, "ax"
.ent _vector_29
_vector_29:
        la  k0,_vector_29_fn
        j   _vector_29_context
        nop
.end _vector_29

# vector 30
.weak _vector_30_fn, _vector_30_context
_vector_30_fn = _default_isr_fn
_vector_30_context  = _isr_context
.section .vector_30, "ax"
.ent _vector_30
_vector_30:
        la  k0,_vector_30_fn
        j   _vector_30_context
        nop
.end _vector_30

# vector 31
.weak _vector_31_fn, _vector_31_context
_vector_31_fn = _default_isr_fn
_vector_31_context  = _isr_context
.section .vector_31, "ax"
.ent _vector_31
_vector_31:
        la  k0,_vector_31_fn
        j   _vector_31_context
        nop
.end _vector_31

# vector 32
.weak _vector_32_fn, _vector_32_context
_vector_32_fn = _default_isr_fn
_vector_32_context  = _isr_context
.section .vector_32, "ax"
.ent _vector_32
_vector_32:
        la  k0,_vector_32_fn
        j   _vector_32_context
        nop
.end _vector_32

# vector 33
.weak _vector_33_fn, _vector_33_context
_vector_33_fn = _default_isr_fn
_vector_33_context  = _isr_context
.section .vector_33, "ax"
.ent _vector_33
_vector_33:
        la  k0,_vector_33_fn
        j   _vector_33_context
        nop
.end _vector_33

# vector 34
.weak _vector_34_fn, _vector_34_context
_vector_34_fn = _default_isr_fn
_vector_34_context  = _isr_context
.section .vector_34, "ax"
.ent _vector_34
_vector_34:
        la  k0,_vector_34_fn
        j   _vector_34_context
        nop
.end _vector_34

# vector 35
.weak _vector_35_fn, _vector_35_context
_vector_35_fn = _default_isr_fn
_vector_35_context  = _isr_context
.section .vector_35, "ax"
.ent _vector_35
_vector_35:
        la  k0,_vector_35_fn
        j   _vector_35_context
        nop
.end _vector_35

# vector 36
.weak _vector_36_fn, _vector_36_context
_vector_36_fn = _default_isr_fn
_vector_36_context  = _isr_context
.section .vector_36, "ax"
.ent _vector_36
_vector_36:
        la  k0,_vector_36_fn
        j   _vector_36_context
        nop
.end _vector_36

# vector 37
.weak _vector_37_fn, _vector_37_context
_vector_37_fn = _default_isr_fn
_vector_37_context  = _isr_context
.section .vector_37, "ax"
.ent _vector_37
_vector_37:
        la  k0,_vector_37_fn
        j   _vector_37_context
        nop
.end _vector_37

# vector 38
.weak _vector_38_fn, _vector_38_context
_vector_38_fn = _default_isr_fn
_vector_38_context  = _isr_context
.section .vector_38, "ax"
.ent _vector_38
_vector_38:
        la  k0,_vector_38_fn
        j   _vector_38_context
        nop
.end _vector_38

# vector 39
.weak _vector_39_fn, _vector_39_context
_vector_39_fn = _default_isr_fn
_vector_39_context  = _isr_context
.section .vector_39, "ax"
.ent _vector_39
_vector_39:
        la  k0,_vector_39_fn
        j   _vector_39_context
        nop
.end _vector_39

# vector 40
.weak _vector_40_fn, _vector_40_context
_vector_40_fn = _default_isr_fn
_vector_40_context  = _isr_context
.section .vector_40, "ax"
.ent _vector_40
_vector_40:
        la  k0,_vector_40_fn
        j   _vector_40_context
        nop
.end _vector_40

# vector 41
.weak _vector_41_fn, _vector_41_context
_vector_41_fn = _default_isr_fn
_vector_41_context  = _isr_context
.section .vector_41, "ax"
.ent _vector_41
_vector_41:
        la  k0,_vector_41_fn
        j   _vector_41_context
        nop
.end _vector_41

# vector 42
.weak _vector_42_fn, _vector_42_context
_vector_42_fn = _default_isr_fn
_vector_42_context  = _isr_context
.section .vector_42, "ax"
.ent _vector_42
_vector_42:
        la  k0,_vector_42_fn
        j   _vector_42_context
        nop
.end _vector_42

# vector 43
.weak _vector_43_fn, _vector_43_context
_vector_43_fn = _default_isr_fn
_vector_43_context  = _isr_context
.section .vector_43, "ax"
.ent _vector_43
_vector_43:
        la  k0,_vector_43_fn
        j   _vector_43_context
        nop
.end _vector_43

# vector 44
.weak _vector_44_fn, _vector_44_context
_vector_44_fn = _default_isr_fn
_vector_44_context  = _isr_context
.section .vector_44, "ax"
.ent _vector_44
_vector_44:
        la  k0,_vector_44_fn
        j   _vector_44_context
        nop
.end _vector_44

# vector 45
.weak _vector_45_fn, _vector_45_context
_vector_45_fn = _default_isr_fn
_vector_45_context  = _isr_context
.section .vector_45, "ax"
.ent _vector_45
_vector_45:
        la  k0,_vector_45_fn
        j   _vector_45_context
        nop
.end _vector_45

# vector 46
.weak _vector_46_fn, _vector_46_context
_vector_46_fn = _default_isr_fn
_vector_46_context  = _isr_context
.section .vector_46, "ax"
.ent _vector_46
_vector_46:
        la  k0,_vector_46_fn
        j   _vector_46_context
        nop
.end _vector_46

# vector 47
.weak _vector_47_fn, _vector_47_context
_vector_47_fn = _default_isr_fn
_vector_47_context  = _isr_context
.section .vector_47, "ax"
.ent _vector_47
_vector_47:
        la  k0,_vector_47_fn
        j   _vector_47_context
        nop
.end _vector_47

# vector 48
.weak _vector_48_fn, _vector_48_context
_vector_48_fn = _default_isr_fn
_vector_48_context  = _isr_context
.section .vector_48, "ax"
.ent _vector_48
_vector_48:
        la  k0,_vector_48_fn
        j   _vector_48_context
        nop
.end _vector_48

# vector 49
.weak _vector_49_fn, _vector_49_context
_vector_49_fn = _default_isr_fn
_vector_49_context  = _isr_context
.section .vector_49, "ax"
.ent _vector_49
_vector_49:
        la  k0,_vector_49_fn
        j   _vector_49_context
        nop
.end _vector_49

# vector 50
.weak _vector_50_fn, _vector_50_context
_vector_50_fn = _default_isr_fn
_vector_50_context  = _isr_context
.section .vector_50, "ax"
.ent _vector_50
_vector_50:
        la  k0,_vector_50_fn
        j   _vector_50_context
        nop
.end _vector_50

# vector 51
.weak _vector_51_fn, _vector_51_context
_vector_51_fn = _default_isr_fn
_vector_51_context  = _isr_context
.section .vector_51, "ax"
.ent _vector_51
_vector_51:
        la  k0,_vector_51_fn
        j   _vector_51_context
        nop
.end _vector_51

# vector 52
.weak _vector_52_fn, _vector_52_context
_vector_52_fn = _default_isr_fn
_vector_52_context  = _isr_context
.section .vector_52, "ax"
.ent _vector_52
_vector_52:
        la  k0,_vector_52_fn
        j   _vector_52_context
        nop
.end _vector_52

# vector 53
.weak _vector_53_fn, _vector_53_context
_vector_53_fn = _default_isr_fn
_vector_53_context  = _isr_context
.section .vector_53, "ax"
.ent _vector_53
_vector_53:
        la  k0,_vector_53_fn
        j   _vector_53_context
        nop
.end _vector_53

# vector 54
.weak _vector_54_fn, _vector_54_context
_vector_54_fn = _default_isr_fn
_vector_54_context  = _isr_context
.section .vector_54, "ax"
.ent _vector_54
_vector_54:
        la  k0,_vector_54_fn
        j   _vector_54_context
        nop
.end _vector_54

# vector 55
.weak _vector_55_fn, _vector_55_context
_vector_55_fn = _default_isr_fn
_vector_55_context  = _isr_context
.section .vector_55, "ax"
.ent _vector_55
_vector_55:
        la  k0,_vector_55_fn
        j   _vector_55_context
        nop
.end _vector_55

# vector 56
.weak _vector_56_fn, _vector_56_context
_vector_56_fn = _default_isr_fn
_vector_56_context  = _isr_context
.section .vector_56, "ax"
.ent _vector_56
_vector_56:
        la  k0,_vector_56_fn
        j   _vector_56_context
        nop
.end _vector_56

# vector 57
.weak _vector_57_fn, _vector_57_context
_vector_57_fn = _default_isr_fn
_vector_57_context  = _isr_context
.section .vector_57, "ax"
.ent _vector_57
_vector_57:
        la  k0,_vector_57_fn
        j   _vector_57_context
        nop
.end _vector_57

# vector 58
.weak _vector_58_fn, _vector_58_context
_vector_58_fn = _default_isr_fn
_vector_58_context  = _isr_context
.section .vector_58, "ax"
.ent _vector_58
_vector_58:
        la  k0,_vector_58_fn
        j   _vector_58_context
        nop
.end _vector_58

# vector 59
.weak _vector_59_fn, _vector_59_context
_vector_59_fn = _default_isr_fn
_vector_59_context  = _isr_context
.section .vector_59, "ax"
.ent _vector_59
_vector_59:
        la  k0,_vector_59_fn
        j   _vector_59_context
        nop
.end _vector_59

# vector 60
.weak _vector_60_fn, _vector_60_context
_vector_60_fn = _default_isr_fn
_vector_60_context  = _isr_context
.section .vector_60, "ax"
.ent _vector_60
_vector_60:
        la  k0,_vector_60_fn
        j   _vector_60_context
        nop
.end _vector_60

# vector 61
.weak _vector_61_fn, _vector_61_context
_vector_61_fn = _default_isr_fn
_vector_61_context  = _isr_context
.section .vector_61, "ax"
.ent _vector_61
_vector_61:
        la  k0,_vector_61_fn
        j   _vector_61_context
        nop
.end _vector_61

# vector 62
.weak _vector_62_fn, _vector_62_context
_vector_62_fn = _default_isr_fn
_vector_62_context  = _isr_context
.section .vector_62, "ax"
.ent _vector_62
_vector_62:
        la  k0,_vector_62_fn
        j   _vector_62_context
        nop
.end _vector_62

# vector 63
.weak _vector_63_fn, _vector_63_context
_vector_63_fn = _default_isr_fn
_vector_63_context  = _isr_context
.section .vector_63, "ax"
.ent _vector_63
_vector_63:
        la  k0,_vector_63_fn
        j   _vector_63_context
        nop
.end _vector_63
