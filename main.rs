#![allow(internal_features)]
#![feature(rustc_attrs, decl_macro, asm_experimental_arch)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};
use crate::custom::*;
mod custom;

// Copy/paste some built-in macros from std

#[rustc_builtin_macro]
pub macro asm("assembly template", $(operands,)* $(options($(option),*))?) {
    /* compiler built-in */
}

#[rustc_builtin_macro]
macro_rules! include_bytes {
    ($file:expr $(,)?) => {{ /* compiler built-in */ }};
}

// Minimal panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Define the entry point of the program
#[no_mangle]
pub extern "C" fn _start() {
    start();
}

//-------------------------------------------------------------------------------

// Screen setup:

const DIW_W: u16 = 320;
const DIW_H: u16 = 256;
const DIW_BW: u16 = DIW_W/8;
const BPLS: u16 = 5;
const IMAGE_SIZE: u16 = DIW_BW*DIW_H*BPLS;

// Entrypoint
extern "C" fn start() {
    kill_system();

    // Set bitplane pointers in copper
    let mut image_addr = IMAGE.as_ptr() as u32;
    for i in 0..BPLS {
        unsafe {
            COPPER[(1+i*4) as usize] = (image_addr >> 16) as u16; 
            COPPER[(3+i*4) as usize] = (image_addr & 0xFFFF) as u16;
        }
        image_addr += DIW_BW as u32;
    }

    #[allow(static_mut_refs)]
    unsafe {
        // Set copper pointer
        write_volatile(&mut (*CUSTOM).cop1lc, COPPER.as_ptr() as u32);
        // Enable copper and blitplane DMA
        write_volatile(&mut (*CUSTOM).dmacon, DMAF_SETCLR|DMAF_MASTER|DMAF_COPPER|DMAF_RASTER);
    }

    while !right_mouse_button() {
        wait_line(303);
    }

    restore_system();
}

fn kill_system() {
    // TODO:
    // Need to back up and restore
    wait_line(303);
    unsafe {
        // DMA and interrupts off
        write_volatile(&mut (*CUSTOM).dmacon, DMAF_ALL);
        write_volatile(&mut (*CUSTOM).intena, INTF_ALL);
    }
}

fn restore_system() {
    // TODO:
    unsafe {
        write_volatile(&mut (*CUSTOM).dmacon, DMAF_ALL);
    }
}

#[inline(always)]
fn wait_line(line: u32) {
    unsafe {
        while (read_volatile(&(*CUSTOM).vposr) & 0x1ff00) != ((line << 8) & 0x1ff00) {}
    }
}

#[inline(always)]
pub fn right_mouse_button() -> bool {
    unsafe {
        !(read_volatile(&(*CUSTOM).potinp) & (1<<10) != 0)
    }
}

//-------------------------------------------------------------------------------

// Copper list

#[link_section = ".MEMF_CHIP"]
static mut COPPER: [u16;102] = [
    // bitplane pointers
    BPLPT+0,0, // bpl1h
    BPLPT+2,0, // bpl1l
    BPLPT+4,0, // bpl2h
    BPLPT+6,0, // bpl2l
    BPLPT+8,0, // bpl3h
    BPLPT+10,0, // bpl3l
    BPLPT+12,0, // bpl4h
    BPLPT+14,0, // bpl4l
    BPLPT+16,0, // bpl5h
    BPLPT+18,0, // bpl5l
    // Screen
    BPLCON0,(BPLS<<12)|(1<<9),
    BPLCON1,0,
    BPL1MOD,DIW_BW*(BPLS-1),
    BPL2MOD,DIW_BW*(BPLS-1),
    DIWSTRT,0x2c81,
    DIWSTOP,0x2cc1,
    DDFSTRT,0x38,
    DDFSTOP,0xd0,
    // Palette
    0x0180,0x0210,0x0182,0x0d75,0x0184,0x0e96,0x0186,0x0b76,
    0x0188,0x0655,0x018a,0x0632,0x018c,0x0854,0x018e,0x0b64,
    0x0190,0x0966,0x0192,0x0422,0x0194,0x0743,0x0196,0x0644,
    0x0198,0x0955,0x019a,0x0978,0x019c,0x0a89,0x019e,0x0ea9,
    0x01a0,0x0942,0x01a2,0x0534,0x01a4,0x0831,0x01a6,0x0b52,
    0x01a8,0x0621,0x01aa,0x0d62,0x01ac,0x0a51,0x01ae,0x0543,
    0x01b0,0x0421,0x01b2,0x0fdb,0x01b4,0x0d94,0x01b6,0x0410,
    0x01b8,0x0778,0x01ba,0x0321,0x01bc,0x0c78,0x01be,0x0834,
    // end copperlist
    0xffff, 0xfffd, 
];

// Image data

#[link_section = ".MEMF_CHIP"]
static IMAGE: [u8; IMAGE_SIZE as usize] = *include_bytes!("image.BPL");
