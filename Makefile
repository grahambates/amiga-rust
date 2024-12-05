program = out/a
OUT = $(program)

PROFILE=release
# PROFILE=debug
PROG=m68k-rust
TARGETDIR=target/m68k-bare-metal/$(PROFILE)
ELFEXE=$(TARGETDIR)/$(PROG).elf

FS-UAE=/Applications/FS-UAE.app/Contents/MacOS/fs-uae
ELF2HUNK=elf2hunk
CC = m68k-amiga-elf-gcc
CCFLAGS = -g -MP -MMD -m68000 -Ofast -nostdlib -Wextra -Wno-unused-function -Wno-volatile-register-var -fomit-frame-pointer -fno-tree-loop-distribution -flto -fwhole-program -fno-exceptions
ASFLAGS = -Wa,-g,--register-prefix-optional

SOURCES = $(wildcard src/*.rs) $(wildcard src/amiga/*.rs)

$(OUT).exe: $(ELFEXE)
	cp $< $(OUT).elf
	$(ELF2HUNK) $(OUT).elf $@

$(ELFEXE): $(SOURCES) gcc8_a_support.o m68k-bare-metal.json
	# cargo +nightly build
	cargo +nightly build --release
	@m68k-amiga-elf-objdump --disassemble --no-show-raw-ins --visualize-jumps -S $@ >$(OUT).s

gcc8_a_support.o: support/gcc8_a_support.s
	$(CC) $(CCFLAGS) $(ASFLAGS) -c -o $@ $<

run: $(OUT).exe
	$(FS-UAE) --hard_drive_0=./out --console_debugger=1

clean:
	cargo clean
	$(RM) *.o $(OUT)/*.*
