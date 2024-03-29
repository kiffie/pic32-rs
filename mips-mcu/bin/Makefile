#
# Makefile for PIC32 native library for Rust runtime
#

# A 'normal' GCC toolchain can be used as well for processing the assembly files
# rather than the ChipKit or XC32 toolchain. When using a 'normal' GCC toolchain,
# the object file patching with bbe is not needed.
# The "Quoted section flags are deprecated" warning from pic32-gcc can be
# ignored.
CC=pic32-gcc

LIB = libmipsmcu

CFLAGS = -Wall -mips32r2 -msoft-float -Os -Iinclude

OBJECTS = mips_irq.o mips_core_timer.o

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
	$(AR) rcsD $@ $^

clean:
	rm -f $(OBJECTS) $(DEPS)

mrproper: clean
	rm -f $(LIB).a

-include $(DEPS)
