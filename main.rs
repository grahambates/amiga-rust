#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Custom regs:
const COLOR00: *mut u16 = 0xdff180 as *mut u16;
const DMACON: *mut u16 = 0xdff096 as *mut u16;
const INTENA: *mut u16 = 0xdff09a as *mut u16;
const COP1LC: *mut usize = 0xdff080 as *mut usize;

extern "C" fn start() -> ! {

    // Everything is unsafe, lol
    unsafe {
        // DMA and interrupts off
        *DMACON = 0x7fff;
        *INTENA = 0x7fff;

        // Set BG red
        *COLOR00 = 0xf00;

        // This wants to do a 32bit multiply operation and crashes ¯\_(ツ)_/¯
        // COP1LC.write_volatile(&COPPER as *const _ as usize);
        // // Enable copper DMA
        // *DMACON = 0x8280;

    }
    loop {}
}

// Define the entry point of the program
#[no_mangle]
pub extern "C" fn _start() -> ! {
    start();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link_section = ".MEMF_CHIP"] // Elf2hunk needs this suffix to map to correct memory type
static COPPER: [u16; 4] = [
    0x180, 0xff0, // Change bg col to yellow
    0xffff, 0xfffd, // end copperlist
];
