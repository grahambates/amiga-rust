use core::arch::asm;

static PLAYER: [u8; 6502] = *include_bytes!("player610.6.no_cia.bin");

pub fn p61_init(module: *const u8) {
    unsafe {
        asm!(
		".short 0x48e7, 0xfffe", // "movem.l %d0-%a6, -(%sp)",
		"jsr (%a3)",
		".short 0x4cdf, 0x7fff", // "movem.l (%sp)+, %d0-%a6",
		in("a0") module,
		in("a1") 0,
		in("a2") 0,
		in("a3") PLAYER.as_ptr(),
		options(nostack, preserves_flags)
        )
    }
}

pub fn p61_music() {
    unsafe {
        asm!(
            ".short 0x48e7, 0xfffe", // "movem.l %d0-%a6, -(%sp)",
            "lea 0xdff000, %a6",
            ".short 0x4eab, 4", // "jsr 4(%a3)",
            ".short 0x4cdf, 0x7fff", // "movem.l (%sp)+, %d0-%a6",
            in("a3") PLAYER.as_ptr(),
            options(nostack, preserves_flags)
        )
    }
}

pub fn p61_end() {
    unsafe {
        asm!(
            ".short 0x48e7, 0xfffe", // "movem.l %d0-%a6, -(%sp)",
            "lea 0xdff000, %a6",
            ".short 0x4eab, 8", // "jsr 8(%a3)",
            ".short 0x4cdf, 0x7fff", // "movem.l (%sp)+, %d0-%a6",
            in("a3") PLAYER.as_ptr(),
            options(nostack, preserves_flags)
        )
    }
}