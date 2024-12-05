use core::arch::*;

pub fn old_open_library(name: *const u8) -> u32 {
    let ptr: u32;
    unsafe {
        asm!(
            "move.l 4, %a6", // exec base
            ".short 0x4eae", // jsr {offset}(a6)
            ".short -408",
            in("a1") name,
            out("d0") ptr,
            options(nostack),
        )
    }
    return ptr;
}
