use core::arch::*;
// use crate::amiga::exec::*;

#[repr(C)]
pub struct GraphicsLib {
    _pad0: [u8; 34],
    pub actiview: u32,
    pub copinit: u32,
    _pad1: [u8; 50 - 38 - 4],
    pub loflist: u32,
}

impl GraphicsLib {
    pub fn instance() -> &'static Self {
        let gfxbase_ptr: *mut Self;
        unsafe {
            asm!(
                "move.l 4, %a6", // exec base
                "move.l 156(%a6), {0}", // Graphics.library (ExecBase->IntVects[6].iv_Data)
                out(reg) gfxbase_ptr,
                options(nostack),
            )
        }
        // let library_name = b"graphics.library\0".as_ptr();
        // let gfxbase_ptr = old_open_library(library_name) as *mut Self;
        unsafe { &*gfxbase_ptr }
    }

    pub fn wait_tof(&self) {
        unsafe {
            asm!(
                "move.l {0}, %a6",
                ".short 0x4eae", // jsr {offset}(a6)
                ".short -270", // _LVOWaitTOF
                in(reg) self,
                options(nostack),
            );
        }
    }

    pub fn load_view(&self, view: u32) {
        unsafe {
            asm!(
                "move.l {0}, %a6",
                ".short 0x4eae", // jsr {offset}(a6)
                ".short -222", // _LVOLoadView
                in(reg) self,
                in("a1") view,
                options(nostack),
            );
        }
    }
}
