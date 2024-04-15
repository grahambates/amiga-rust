PROG=m68k-rust
TARGETDIR=target/m68k-bare-metal/debug
ELFEXE=$(TARGETDIR)/$(PROG)
HUNKEXE=$(TARGETDIR)/$(PROG).exe

FS-UAE=/Applications/FS-UAE-3.app/Contents/MacOS/fs-uae
ELF2HUNK=elf2hunk
CC = m68k-amiga-elf-gcc
CCFLAGS = -g -MP -MMD -m68000 -Ofast -nostdlib -Wextra -Wno-unused-function -Wno-volatile-register-var -fomit-frame-pointer -fno-tree-loop-distribution -flto -fwhole-program -fno-exceptions
ASFLAGS = -Wa,-g,--register-prefix-optional

$(HUNKEXE): $(ELFEXE)
	$(ELF2HUNK) $< $@

$(ELFEXE): main.rs gcc8_a_support.o
	cargo xbuild --target ./m68k-bare-metal.json

gcc8_a_support.o: gcc8_a_support.s
	$(CC) $(CCFLAGS) $(ASFLAGS) -c -o $@ $<

run: $(HUNKEXE)
	$(FS-UAE) --hard_drive_0=.

clean:
	cargo clean
	$(RM) *.o
