use crate::amiga::custom::*;

/// Copper instruction
pub struct CopInst {
    pub first: u16,
    pub second: u16,
}

impl CopInst {
    /// Move to custom register
    pub const fn mov(offset: CustomOffset, value: u16) -> Self {
        Self {
            first: offset.as_u16(),
            second: value,
        }
    }

    /// Wait raster position
    pub const fn wait(v: u16, h: u16) -> Self {
        Self {
            first: ((v & 0xff) << 8) + (h & 0xfe) + 1,
            second: 0xfffe,
        }
    }

    /// Wait vertical raster position
    pub const fn wait_v(v: u16) -> Self {
        Self::wait(v, 4)
    }

    /// Wait horizontal raster position
    /// Still needs first bit of vertical
    pub const fn wait_h(v: u16, h: u16) -> Self {
        Self {
            first: ((v & 0x80) << 8) + (h & 0xfe) + 1,
            second: 0x80fe,
        }
    }

    /// Skip next instruction after raster position
    pub const fn skip(v: u16, h: u16) -> Self {
        Self {
            first: ((v & 0xff) << 8) + (h & 0xfe) + 1,
            second: 0xffff,
        }
    }

    /// Skip next instruction after vertical raster position
    pub const fn skip_v(v: u16) -> Self {
        Self::skip(v, 4)
    }

    /// Skip next instruction after horizontal raster position
    /// Still needs first bit of vertical
    pub const fn skip_h(v: u16, h: u16) -> Self {
        Self {
            first: ((v & 0x80) << 8) + (h & 0xfe) + 1,
            second: 0x80ff,
        }
    }

    /// End copperlist
    pub const fn nop() -> Self {
        Self {
            first: 0x0,
            second: 0x1fe,
        }
    }

    /// Wait for blitter
    pub const fn wait_blit() -> Self {
        Self {
            first: 1,
            second: 0,
        }
    }

    /// End copperlist
    pub const fn end() -> Self {
        Self {
            first: 0xffff,
            second: 0xfffe,
        }
    }
}
