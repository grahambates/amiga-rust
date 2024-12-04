#![allow(static_mut_refs, dead_code)]
#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

use crate::custom::*;
use crate::hw::*;
use core::panic::PanicInfo;
mod custom;
mod hw;

// Minimal panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
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

    let custom = Custom::instance();

    // Set copper pointer
    unsafe {
        custom.cop1lc(COPPER.as_ptr() as u32);
    }
    // Enable copper and blitplane DMA
    custom.dmacon(
        DmaBit::SetClr.flag()
            | DmaBit::Master.flag()
            | DmaBit::Copper.flag()
            | DmaBit::Raster.flag(),
    );

    while !right_mouse_button() {
        wait_eof();
    }

    restore_system(state);
}

//-------------------------------------------------------------------------------

// Copper list

#[link_section = ".MEMF_CHIP"]
static mut COPPER: [CopInst; 51] = [
    // bitplane pointers
    CopInst::mov(CustomOffset::Bplpt1h, 0),
    CopInst::mov(CustomOffset::Bplpt1l, 0),
    CopInst::mov(CustomOffset::Bplpt2h, 0),
    CopInst::mov(CustomOffset::Bplpt2l, 0),
    CopInst::mov(CustomOffset::Bplpt3h, 0),
    CopInst::mov(CustomOffset::Bplpt3l, 0),
    CopInst::mov(CustomOffset::Bplpt4h, 0),
    CopInst::mov(CustomOffset::Bplpt4l, 0),
    CopInst::mov(CustomOffset::Bplpt5h, 0),
    CopInst::mov(CustomOffset::Bplpt5l, 0),
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
    // end copperlist
    CopInst::end(),
];

// Image data

#[link_section = ".MEMF_CHIP"]
static IMAGE: [u8; IMAGE_SIZE as usize] = *include_bytes!("image.BPL");
