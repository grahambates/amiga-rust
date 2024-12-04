use core::ptr::{read_volatile, write_volatile};

impl Custom {
    pub fn instance() -> &'static mut Self {
        unsafe { &mut *(0xdff000 as *mut Self) }
    }

    // Getters
    pub fn dmacon(&mut self, value: u16) {
        unsafe { write_volatile(&mut self.dmacon, value); }
    }
    pub fn intena(&mut self, value: u16) {
        unsafe { write_volatile(&mut self.intena, value); }
    }
    pub fn cop1lc(&mut self, value: u32) {
        unsafe { write_volatile(&mut self.cop1lc, value); }
    }
    pub fn cop2lc(&mut self, value: u32) {
        unsafe { write_volatile(&mut self.cop2lc, value); }
    }

    // Setters
    pub fn vposr(&mut self) -> u32 {
        unsafe { read_volatile(&mut self.vposr) }   
    }
    pub fn dmaconr(&mut self) -> u16 {
        unsafe { read_volatile(&mut self.dmaconr) }   
    }
    pub fn intenar(&mut self) -> u16 {
        unsafe { read_volatile(&mut self.intenar) }   
    }
    pub fn intreqr(&mut self) -> u16 {
        unsafe { read_volatile(&mut self.intreqr) }   
    }
    pub fn potinp(&mut self) -> u16 {
        unsafe { read_volatile(&mut self.potinp) }   
    }

    // TODO: add the rest...
}

/// Custom struct
#[repr(C)]
pub struct Custom {
    bltddat: u16,
    dmaconr: u16,
    vposr: u32,
    dskdatr: u16,
    joy0dat: u16,
    joy1dat: u16,
    clxdat: u16,
    adkconr: u16,
    pot0dat: u16,
    pot1dat: u16,
    potinp: u16,
    serdatr: u16,
    dskbytr: u16,
    intenar: u16,
    intreqr: u16,
    dskpt: u32,
    dsklen: u16,
    dskdat: u16,
    refptr: u16,
    vposw: u16,
    vhposw: u16,
    copcon: u16,
    serdat: u16,
    serper: u16,
    potgo: u16,
    joytest: u16,
    strequ: u16,
    strvbl: u16,
    strhor: u16,
    strlong: u16,
    bltcon0: u16,
    bltcon1: u16,
    bltafwm: u16,
    bltalwm: u16,
    bltcpt: u32,
    bltbpt: u32,
    bltapt: u32,
    bltdpt: u32,
    bltsize: u16,
    pad2d: u8,
    bltcon0l: u8,
    bltsizv: u16,
    bltsizh: u16,
    bltcmod: u16,
    bltbmod: u16,
    bltamod: u16,
    bltdmod: u16,
    pad34: [u16; 4],
    bltcdat: u16,
    bltbdat: u16,
    bltadat: u16,
    pad3b: [u16; 3],
    deniseid: u16,
    dsksync: u16,
    cop1lc: u32,
    cop2lc: u32,
    copjmp1: u16,
    copjmp2: u16,
    copins: u16,
    diwstrt: u16,
    diwstop: u16,
    ddfstrt: u16,
    ddfstop: u16,
    dmacon: u16,
    clxcon: u16,
    intena: u16,
    intreq: u16,
    adkcon: u16,
    aud: [AudChannel; 4],
    bplpt: [u32; 8],
    bplcon0: u16,
    bplcon1: u16,
    bplcon2: u16,
    bplcon3: u16,
    bpl1mod: u16,
    bpl2mod: u16,
    bplcon4: u16,
    clxcon2: u16,
    bpldat: [u16; 8],
    sprpt: [u32; 8],
    spr: [SpriteDef; 8],
    color: [u16; 32],
    htotal: u16,
    hsstop: u16,
    hbstrt: u16,
    hbstop: u16,
    vtotal: u16,
    vsstop: u16,
    vbstrt: u16,
    vbstop: u16,
    sprhstrt: u16,
    sprhstop: u16,
    bplhstrt: u16,
    bplhstop: u16,
    hhposw: u16,
    hhposr: u16,
    beamcon0: u16,
    hsstrt: u16,
    vsstrt: u16,
    hcenter: u16,
    diwhigh: u16,
    padf3: [u16; 11],
    fmode: u16,
}

#[repr(C)]
pub struct AudChannel {
    pub ac_ptr: u32,
    pub ac_len: u16,
    pub ac_per: u16,
    pub ac_vol: u16,
    pub ac_dat: u16,
    pub ac_pad: [u16; 2],
}

#[repr(C)]
pub struct SpriteDef {
    pub pos: u16,
    pub ctl: u16,
    pub dataa: u16,
    pub datab: u16,
}

#[repr(u16)]
pub enum InterruptBit {
    SetClr = 15,
    IntEn = 14,
    Exter = 13,
    DiskSync = 12,
    Rbf = 11,
    Aud3 = 10,
    Aud2 = 9,
    Aud1 = 8,
    Aud0 = 7,
    Blit = 6,
    Vertb = 5,
    Coper = 4,
    Ports = 3,
    SoftInt = 2,
    DiskBlk = 1,
    Tbe = 0,
}

impl InterruptBit {
    /// Compute the interrupt flag for the bit
    pub const fn flag(self) -> u16 {
        1 << (self as u16)
    }

    /// All flags combined
    pub const fn all_flags() -> u16 {
        0x3FFF
    }
}

#[repr(u16)]
pub enum DmaBit {
    SetClr = 15,
    Aud0 = 0,
    Aud1 = 1,
    Aud2 = 2,
    Aud3 = 3,
    Disk = 4,
    Sprite = 5,
    Blitter = 6,
    Copper = 7,
    Raster = 8,
    Master = 9,
    BlitHog = 10,
}

impl DmaBit {
    /// Compute the DMA flag for the bit
    pub const fn flag(self) -> u16 {
        1 << (self as u16)
    }

    /// Audio-related flags (combines Aud0, Aud1, Aud2, Aud3)
    pub const fn audio_flags() -> u16 {
        DmaBit::Aud0.flag()
            | DmaBit::Aud1.flag()
            | DmaBit::Aud2.flag()
            | DmaBit::Aud3.flag()
    }

    /// All flags combined
    pub const fn all_flags() -> u16 {
        // DMAF_ALL covers all relevant bits, up to `Master`
        0x01FF
    }
}

// Custom offsets
#[repr(u16)]
pub enum CustomOffset {
    Bltddat = 0x000,
    Dmaconr = 0x002,
    Vposr = 0x004,
    Vhposr = 0x006,
    Dskdatr = 0x008,
    Joy0dat = 0x00a,
    Joy1dat = 0x00c,
    Clxdat = 0x00e,
    Adkconr = 0x010,
    Pot0dat = 0x012,
    Pot1dat = 0x014,
    Potinp = 0x016,
    Serdatr = 0x018,
    Dskbytr = 0x01a,
    Intenar = 0x01c,
    Intreqr = 0x01e,
    Dskpt = 0x020,
    Dsklen = 0x024,
    Dskdat = 0x026,
    Refptr = 0x028,
    Vposw = 0x02a,
    Vhposw = 0x02c,
    Copcon = 0x02e,
    Serdat = 0x030,
    Serper = 0x032,
    Potgo = 0x034,
    Joytest = 0x036,
    Str = 0x038,
    Strvbl = 0x03a,
    Strhor = 0x03c,
    Strlong = 0x03e,
    Bltcon0 = 0x040,
    Bltcon1 = 0x042,
    Bltafwm = 0x044,
    Bltalwm = 0x046,
    Bltcpt = 0x048,
    Bltbpt = 0x04c,
    Bltapt = 0x050,
    Bltdpt = 0x054,
    Bltsize = 0x058,
    Bltcon0l = 0x05b,
    Bltsizv = 0x05c,
    Bltsizh = 0x05e,
    Bltcmod = 0x060,
    Bltbmod = 0x062,
    Bltamod = 0x064,
    Bltdmod = 0x066,
    Bltcdat = 0x070,
    Bltbdat = 0x072,
    Bltadat = 0x074,
    Deniseid = 0x07c,
    Dsksync = 0x07e,
    Cop1lc = 0x080,
    Cop2lc = 0x084,
    Copjmp1 = 0x088,
    Copjmp2 = 0x08a,
    Copins = 0x08c,
    Diwstrt = 0x08e,
    Diwstop = 0x090,
    Ddfstrt = 0x092,
    Ddfstop = 0x094,
    Dmacon = 0x096,
    Clxcon = 0x098,
    Intena = 0x09a,
    Intreq = 0x09c,
    Adkcon = 0x09e,
    Aud0 = 0x0a0,
    Aud1 = 0x0b0,
    Aud2 = 0x0c0,
    Aud3 = 0x0d0,
    Bplpt1h = 0x0e0,
    Bplpt1l = 0x0e2,
    Bplpt2h = 0x0e4,
    Bplpt2l = 0x0e6,
    Bplpt3h = 0x0e8,
    Bplpt3l = 0x0ea,
    Bplpt4h = 0x0ec,
    Bplpt4l = 0x0ee,
    Bplpt5h = 0x0f0,
    Bplpt5l = 0x0f2,
    Bplcon0 = 0x100,
    Bplcon1 = 0x102,
    Bplcon2 = 0x104,
    Bplcon3 = 0x106,
    Bpl1mod = 0x108,
    Bpl2mod = 0x10a,
    Bplcon4 = 0x10c,
    Clxcon2 = 0x10e,
    Bpldat = 0x110,
    Sprpt = 0x120,
    Spr = 0x140,
    Color00 = 0x180,
    Color01 = 0x182,
    Color02 = 0x184,
    Color03 = 0x186,
    Color04 = 0x188,
    Color05 = 0x18A,
    Color06 = 0x18C,
    Color07 = 0x18E,
    Color08 = 0x190,
    Color09 = 0x192,
    Color10 = 0x194,
    Color11 = 0x196,
    Color12 = 0x198,
    Color13 = 0x19A,
    Color14 = 0x19C,
    Color15 = 0x19E,
    Color16 = 0x1A0,
    Color17 = 0x1A2,
    Color18 = 0x1A4,
    Color19 = 0x1A6,
    Color20 = 0x1A8,
    Color21 = 0x1AA,
    Color22 = 0x1AC,
    Color23 = 0x1AE,
    Color24 = 0x1B0,
    Color25 = 0x1B2,
    Color26 = 0x1B4,
    Color27 = 0x1B6,
    Color28 = 0x1B8,
    Color29 = 0x1BA,
    Color30 = 0x1BC,
    Color31 = 0x1BE,
    Htotal = 0x1c0,
    Hsstop = 0x1c2,
    Hbstrt = 0x1c4,
    Hbstop = 0x1c6,
    Vtotal = 0x1c8,
    Vsstop = 0x1ca,
    Vbstrt = 0x1cc,
    Vbstop = 0x1ce,
    Sprhstrt = 0x1d0,
    Sprhstop = 0x1d2,
    Bplhstrt = 0x1d4,
    Bplhstop = 0x1d6,
    Hhposw = 0x1d8,
    Hhposr = 0x1da,
    Beamcon0 = 0x1dc,
    Hsstrt = 0x1de,
    Vsstrt = 0x1e0,
    Hcenter = 0x1e2,
    Diwhigh = 0x1e4,
    Fmode = 0x1fc,
}

impl CustomOffset {
    pub const fn as_u16(self) -> u16 {
        self as u16
    }
}
