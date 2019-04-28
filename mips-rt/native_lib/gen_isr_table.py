#!/usr/bin/python3
#
# generate ISR table
#

vect_fmt = '''\
# vector {0}
.weak _vector_{0}_fn, _vector_{0}_j
_vector_{0}_fn = _default_isr_fn
_vector_{0}_context  = _isr_context
.section .vector_{0}, "ax"
.ent _vector_{0}
_vector_{0}:
        la  k0,_vector_{0}_fn
        j   _vector_{0}_context
        nop
.end _vector_{0}
'''


for i in range(0, 64):
    print(vect_fmt.format(i))
