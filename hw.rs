use core::arch::*;
use crate::custom::*;

pub struct SysState {
    dmacon: u16,
    intena: u16,
    intreq: u16,
    actiview: u32,
    cop1: u32,
    cop2: u32,
}

pub fn kill_system() -> SysState {
    let custom = Custom::instance();
    let mut actiview: u32;
    let mut cop1: u32;
    let mut cop2: u32;

    unsafe {
        asm!(
            "move.l 4, %a6", // exec base
            "move.l 156(%a6), %a6", // Graphics.library (ExecBase->IntVects[6].iv_Data)
            "move.l 34(%a6), {0}", // gb_ActiView
            "move.l 38(%a6), {1}", // gb_copinit
            "move.l 50(%a6), {2}", // gb_LOFlist
            out(reg) actiview,
            out(reg) cop1,
            out(reg) cop2,
            options(nostack),
        );
    }

    load_view(0);

    // Backup system state
    let state: SysState = SysState{
        dmacon : custom.dmaconr(),
        intena : custom.intenar(),
        intreq : custom.intreqr(),
        actiview,
        cop1,
        cop2,
    };

    wait_line(303);

    // DMA and interrupts off
    custom.dmacon(DmaBit::all_flags());
    custom.intena(InterruptBit::all_flags());

    return state;
}

pub fn restore_system(state: SysState) {
    wait_line(303);
    let custom = Custom::instance();
    // Disable all:
    custom.dmacon(DmaBit::all_flags());
    custom.intena(InterruptBit::all_flags());
    // Restore:
    custom.dmacon(DmaBit::SetClr.flag() | DmaBit::Master.flag() | state.dmacon);
    custom.intena(InterruptBit::SetClr.flag() | state.intena);
    custom.cop1lc(state.cop1);
    custom.cop2lc(state.cop2);
    // Load View
    load_view(state.actiview);

    // Return status - TODO: better way to do this?
    unsafe { asm!( "move.l $0, %d0", options(nostack)); }
}

#[inline(always)]
pub fn wait_line(line: u32) {
    let custom = Custom::instance();
    while (custom.vposr() & 0x1ff00) != ((line << 8) & 0x1ff00) {}
}

#[inline(always)]
pub fn right_mouse_button() -> bool {
    Custom::instance().potinp() & (1 << 10) == 0
}


fn load_view(view: u32) {
    unsafe {
        asm!(
            "move.l 4, %a6", // exec base
            "move.l 156(%a6), %a6", // Graphics.library (ExecBase->IntVects[6].iv_Data)
            ".short 0x4eae", // jsr {offset}(a6)
            ".short -222", // _LVOLoadView
            in("a1") view,
            options(nostack),
        );
    }
}

// Copper

pub struct CopInst {
    pub first: u16,
    pub second: u16,
}

impl CopInst {
    pub const fn mov(offset: CustomOffset, value: u16) -> Self {
        Self {
            first: offset.as_u16(),
            second: value,
        }
    }

    pub const fn raw(first: u16, second: u16) -> Self {
        Self {
            first,
            second,
        }
    }

    pub const fn end() -> Self {
        Self {
            first: 0xffff,
            second: 0xfffe,
        }
    }
}
