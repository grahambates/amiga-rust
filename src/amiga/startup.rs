use crate::amiga::custom::*;
use crate::amiga::gfx::*;
use crate::amiga::utils::wait_eof;
use crate::get_l6int;
use crate::set_l6int;

/// Backed up system state to restore on exit
pub struct SysState {
    dmacon: u16,
    intena: u16,
    intreq: u16,
    actiview: u32,
    cop1: u32,
    cop2: u32,
    l6int: u32,
}

/// Disable the OS and get state to restore later
pub fn kill_system() -> SysState {
    let custom = Custom::instance();

    let gfxbase = GraphicsLib::instance();
    let actiview = gfxbase.actiview;
    let cop1 = gfxbase.copinit;
    let cop2 = gfxbase.loflist;

    // Backup system state
    let state: SysState = SysState {
        dmacon: custom.dmaconr(),
        intena: custom.intenar(),
        intreq: custom.intreqr(),
        actiview,
        cop1,
        cop2,
        l6int: get_l6int(),
    };

    gfxbase.load_view(0);
    wait_eof();

    // DMA and interrupts off
    custom.dmacon(DmaBit::all_flags());
    custom.intena(InterruptBit::all_flags());

    return state;
}

/// Restore OS and restore previous state
pub fn restore_system(state: SysState) {
    wait_eof();
    let custom = Custom::instance();
    // Disable all:
    custom.dmacon(DmaBit::all_flags());
    custom.intena(InterruptBit::all_flags());
    set_l6int(state.l6int);
    // Restore:
    custom.dmacon(DmaBit::SetClr.flag() | DmaBit::Master.flag() | state.dmacon);
    custom.intena(InterruptBit::SetClr.flag() | state.intena);
    custom.cop1lc(state.cop1);
    custom.cop2lc(state.cop2);
    // Load View
    GraphicsLib::instance().load_view(state.actiview);
}
