use crate::amiga::cia::*;
use crate::amiga::custom::*;

/// Wait for raster line
#[inline(always)]
pub fn wait_line(line: u32) {
    let custom = Custom::instance();
    while (custom.vposr() & 0x1ff00) != ((line << 8) & 0x1ff00) {}
}

/// Wait for end of frame
#[inline(always)]
pub fn wait_eof() {
    wait_line(303);
}

/// Is right mouse button pressed?
#[inline(always)]
pub fn right_mouse_button() -> bool {
    Custom::instance().potinp() & (1 << 10) == 0
}

/// Is left mouse button pressed?
#[inline(always)]
pub fn left_mouse_button() -> bool {
    CIA::a().pra() & CiaAPortABit::GamePort0.flag() == 0
}

/// Wait until blitter is not busy
#[inline(always)]
pub fn wait_blit() {
    Custom::instance().dmaconr(); // A1000 compat
    while Custom::instance().dmaconr() & DmaBit::Blitter.flag() != 0 {}
}
