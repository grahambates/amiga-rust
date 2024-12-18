#![allow(static_mut_refs, dead_code)]
#![feature(asm_experimental_arch, naked_functions)]
#![no_std]
#![no_main]

use core::arch::*;
use core::panic::PanicInfo;
mod amiga;
use crate::amiga::copper::*;
use crate::amiga::custom::*;
use crate::amiga::p61::*;
use crate::amiga::startup::*;
use crate::amiga::utils::*;

// Minimal panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {} // Enter an infinite loop as a placeholder for abort
}

//-------------------------------------------------------------------------------

// Screen setup:

const DIW_W: u16 = 320;
const DIW_H: u16 = 256;
const DIW_BW: u16 = DIW_W / 8;
const BPLS: u16 = 5;
const IMAGE_SIZE: u16 = DIW_BW * DIW_H * BPLS;

// Entrypoint
#[no_mangle]
extern "C" fn _start() {
    let state = kill_system();

    // Set bitplane pointers in copper
    let mut image_addr = IMAGE.as_ptr() as u32;
    for i in 0..BPLS {
        unsafe {
            COPPER[(i * 2) as usize].second = (image_addr >> 16) as u16;
            COPPER[(i * 2 + 1) as usize].second = (image_addr & 0xFFFF) as u16;
        }
        image_addr += DIW_BW as u32;
    }

    p61_init(MOD.as_ptr());
    set_interrupt_l6(l6int as * const ());

    // Set copper pointer
    unsafe {
        Custom::instance()
            .cop1lc(COPPER.as_ptr() as u32)
            .intena(InterruptBit::SetClr.flag() | InterruptBit::Vertb.flag())
            .dmacon(
                DmaBit::SetClr.flag()
                    | DmaBit::Master.flag()
                    | DmaBit::Copper.flag()
                    | DmaBit::Raster.flag(),
            );
    }

    while !left_mouse_button() {
        // Do effect
        wait_blit();
        wait_eof();
    }

    p61_end();
    restore_system(state);
    unsafe { asm!("move.l $0, %d0", options(nostack)) }
}

fn l6int() {
    unsafe {
        asm!(
            ".short 0x48e7, 0xfffe", // "movem.l %d0-%a6, -(%sp)",
            "jsr handle_interrupt",
            ".short 0x4cdf, 0x7fff", // "movem.l (%sp)+, %d0-%a6",
            "rte",
            options(nostack),
        )
    }
}

#[no_mangle]
fn handle_interrupt() {
    Custom::instance().intreq(InterruptBit::Vertb.flag());
    unsafe { FRAME += 1 }
    p61_music();
}

static mut FRAME: u32 = 0;

//-------------------------------------------------------------------------------

// Copper list

#[link_section = ".MEMF_CHIP"]
static mut COPPER: [CopInst; 53] = [
    // bitplane pointers
    CopInst::mov(CustomOffset::Bpl1pth, 0),
    CopInst::mov(CustomOffset::Bpl1ptl, 0),
    CopInst::mov(CustomOffset::Bpl2pth, 0),
    CopInst::mov(CustomOffset::Bpl2ptl, 0),
    CopInst::mov(CustomOffset::Bpl3pth, 0),
    CopInst::mov(CustomOffset::Bpl3ptl, 0),
    CopInst::mov(CustomOffset::Bpl4pth, 0),
    CopInst::mov(CustomOffset::Bpl4ptl, 0),
    CopInst::mov(CustomOffset::Bpl5pth, 0),
    CopInst::mov(CustomOffset::Bpl5ptl, 0),
    // Screen
    CopInst::mov(CustomOffset::Bplcon0, (BPLS << 12) | (1 << 9)),
    CopInst::mov(CustomOffset::Bplcon1, 0),
    CopInst::mov(CustomOffset::Bpl1mod, DIW_BW * (BPLS - 1)),
    CopInst::mov(CustomOffset::Bpl2mod, DIW_BW * (BPLS - 1)),
    CopInst::mov(CustomOffset::Diwstrt, 0x2c81),
    CopInst::mov(CustomOffset::Diwstop, 0x2cc1),
    CopInst::mov(CustomOffset::Ddfstrt, 0x38),
    CopInst::mov(CustomOffset::Ddfstop, 0xd0),
    // Palette
    CopInst::mov(CustomOffset::Color00, 0x0210),
    CopInst::mov(CustomOffset::Color01, 0x0d75),
    CopInst::mov(CustomOffset::Color02, 0x0e96),
    CopInst::mov(CustomOffset::Color03, 0x0b76),
    CopInst::mov(CustomOffset::Color04, 0x0655),
    CopInst::mov(CustomOffset::Color05, 0x0632),
    CopInst::mov(CustomOffset::Color06, 0x0854),
    CopInst::mov(CustomOffset::Color07, 0x0b64),
    CopInst::mov(CustomOffset::Color08, 0x0966),
    CopInst::mov(CustomOffset::Color09, 0x0422),
    CopInst::mov(CustomOffset::Color10, 0x0743),
    CopInst::mov(CustomOffset::Color11, 0x0644),
    CopInst::mov(CustomOffset::Color12, 0x0955),
    CopInst::mov(CustomOffset::Color13, 0x0978),
    CopInst::mov(CustomOffset::Color14, 0x0a89),
    CopInst::mov(CustomOffset::Color15, 0x0ea9),
    CopInst::mov(CustomOffset::Color16, 0x0942),
    CopInst::mov(CustomOffset::Color17, 0x0534),
    CopInst::mov(CustomOffset::Color18, 0x0831),
    CopInst::mov(CustomOffset::Color19, 0x0b52),
    CopInst::mov(CustomOffset::Color20, 0x0621),
    CopInst::mov(CustomOffset::Color21, 0x0d62),
    CopInst::mov(CustomOffset::Color22, 0x0a51),
    CopInst::mov(CustomOffset::Color23, 0x0543),
    CopInst::mov(CustomOffset::Color24, 0x0421),
    CopInst::mov(CustomOffset::Color25, 0x0fdb),
    CopInst::mov(CustomOffset::Color26, 0x0d94),
    CopInst::mov(CustomOffset::Color27, 0x0410),
    CopInst::mov(CustomOffset::Color28, 0x0778),
    CopInst::mov(CustomOffset::Color29, 0x0321),
    CopInst::mov(CustomOffset::Color30, 0x0c78),
    CopInst::mov(CustomOffset::Color31, 0x0834),
    // Test wait
    CopInst::wait_v(100),
    CopInst::mov(CustomOffset::Color00, 0x0111),
    // end copperlist
    CopInst::end(),
];

// Image data
#[link_section = ".MEMF_CHIP"]
static IMAGE: [u8; IMAGE_SIZE as usize] = *include_bytes!("../data/image.BPL");

// Module file
#[link_section = ".MEMF_CHIP"]
static MOD: [u8; 5240] = *include_bytes!("../data/testmod.p61");
