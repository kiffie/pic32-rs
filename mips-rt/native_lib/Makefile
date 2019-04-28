#
# Makefile for PIC32 native library for Rust runtime
#

# A 'normal' GCC toolchain can be used as well for processing the assembly files
# rather than the ChipKit or XC32 toolchain. When using a 'normal' GCC toolchain,
# the object file patching with bbe is not needed.
# The "Quoted section flags are deprecated" warning from pic32-gcc can be
# ignored.
CC=pic32-gcc

LIB = libnative

CFLAGS = -Wall -g -mips32r2 -msoft-float -Os -Iinclude

OBJECTS = crt0.o isr_context.o exception_table.o mips_irq.o general-exception.o

DEPS    = $(patsubst %.o, %.d, $(notdir $(OBJECTS)))

.PHONY: all
all: $(LIB).a

# bbe is used to fix the ELF header (which is incorrect when using pic32-gcc)
%.o: %.S
	$(CC) -c $(CFLAGS) -MMD -o $@_ $<
	bbe -e 'r 38 \x00' $@_ > $@
	rm -f $@_

$(LIB).a: $(OBJECTS)
	rm -f $@
	$(AR) crs $@ $^

clean:
	rm -f $(OBJECTS) $(DEPS)

mrpropoer: clean
	rm -f $(LIB).a

-include $(DEPS)
