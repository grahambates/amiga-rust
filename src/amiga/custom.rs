use core::ptr::{read_volatile, write_volatile};

macro_rules! define_write_fn {
    ($name:ident, $ty:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $name(&mut self, value: $ty) {
            unsafe { write_volatile(&mut self.$name, value); }
        }
    };
}
macro_rules! define_read_fn {
    ($name:ident, $ty:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $name(&self) -> $ty {
            unsafe { read_volatile(&self.$name) }
        }
    };
}

impl Custom {
    /// Get instance of struct at fixed address 0xdff000
    pub fn instance() -> &'static mut Self {
        unsafe { &mut *(0xdff000 as *mut Self) }
    }

    // Getters / setters
    define_read_fn!(bltddat, u16, "Blitter dest. early read (dummy address)");
    define_read_fn!(dmaconr, u16, "Dma control (and blitter status) read");
    define_read_fn!(vposr, u32, "Read vertical most sig. bits (and frame flop)");
    define_read_fn!(dskdatr, u16, "Disk data early read (dummy address)");
    define_read_fn!(joy0dat, u16, "Joystick-mouse 0 data (vert, horiz)");
    define_read_fn!(joy1dat, u16, "Joystick-mouse 1 data (vert, horiz)");
    define_read_fn!(clxdat, u16, "Collision data reg. (read and clear)");
    define_read_fn!(adkconr, u16, "Audio,disk control register read");
    define_read_fn!(pot0dat, u16, "Pot counter data left pair (vert, horiz)");
    define_read_fn!(pot1dat, u16, "Pot counter data right pair (vert, horiz)");
    define_read_fn!(potinp, u16, "Pot pin data read");
    define_read_fn!(serdatr, u16, "Serial port data and status read");
    define_read_fn!(dskbytr, u16, "Disk data byte and status read");
    define_read_fn!(intenar, u16, "Interrupt enable bits read");
    define_read_fn!(intreqr, u16, "Interrupt request bits read");
    define_write_fn!(dskpt, u32, "Disk pointer");
    define_write_fn!(dsklen, u16, "Disk length");
    define_write_fn!(dskdat, u16, "Disk DMA data write");
    define_write_fn!(refptr, u16, "Refresh pointer");
    define_write_fn!(vposw, u16, "Write vert most sig. bits (and frame flop)");
    define_write_fn!(vhposw, u16, "Write vert and horiz pos of beam");
    define_write_fn!(copcon, u16, "Coprocessor control");
    define_write_fn!(serdat, u16, "Serial port data and stop bits write");
    define_write_fn!(serper, u16, "Serial port period and control");
    define_write_fn!(potgo, u16, "Pot count start,pot pin drive enable data");
    define_write_fn!(joytest, u16, "Write to all 4 joystick-mouse counters at once");
    define_write_fn!(strequ, u16, "Strobe for horiz sync with VB and EQU");
    define_write_fn!(strvbl, u16, "Strobe for horiz sync with VB (vert blank)");
    define_write_fn!(strhor, u16, "Strobe for horiz sync");
    define_write_fn!(strlong, u16, "Strobe for identification of long horiz line");
    define_write_fn!(bltcon0, u16, "Blitter control register 0");
    define_write_fn!(bltcon1, u16, "Blitter control register 1");
    define_write_fn!(bltafwm, u16, "Blitter first word mask for source A");
    define_write_fn!(bltalwm, u16, "Blitter last word mask for source A");
    define_write_fn!(bltcpt, u32, "Blitter pointer to source C");
    define_write_fn!(bltbpt, u32, "Blitter pointer to source B");
    define_write_fn!(bltapt, u32, "Blitter pointer to source A");
    define_write_fn!(bltdpt, u32, "Blitter pointer to dest D");
    define_write_fn!(bltsize, u16, "Blitter start and size (win/width,height)");
    define_write_fn!(bltcon0l, u8, "control 0, lower 8 bits (minterms)");
    define_write_fn!(bltsizv, u16, "V size (for 15 bit vertical size)");
    define_write_fn!(bltsizh, u16, "H size and start (for 11 bit H size)");
    define_write_fn!(bltcmod, u16, "Blitter modulo for source C");
    define_write_fn!(bltbmod, u16, "Blitter modulo for source B");
    define_write_fn!(bltamod, u16, "Blitter modulo for source A");
    define_write_fn!(bltdmod, u16, "Blitter modulo for dest D");
    define_write_fn!(bltcdat, u16, "Blitter source C data register");
    define_write_fn!(bltbdat, u16, "Blitter source B data register");
    define_write_fn!(bltadat, u16, "Blitter source A data register");
    define_read_fn!(deniseid, u16, "revision level for Denise/Lisa (video out chip)");
    define_write_fn!(dsksync, u16, "Disk sync pattern reg for disk read");
    define_write_fn!(cop1lc, u32, "Coprocessor 1st location");
    define_write_fn!(cop2lc, u32, "Coprocessor 2nd location");
    define_write_fn!(copjmp1, u16, "Coprocessor restart at 1st location");
    define_write_fn!(copjmp2, u16, "Coprocessor restart at 2nd location");
    define_write_fn!(copins, u16, "Coprocessor inst fetch identify");
    define_write_fn!(diwstrt, u16, "Display window start (upper left vert,horiz pos)");
    define_write_fn!(diwstop, u16, "Display window stop (lower right vert,horiz pos)");
    define_write_fn!(ddfstrt, u16, "Display bit plane data fetch start,horiz pos");
    define_write_fn!(ddfstop, u16, "Display bit plane data fetch stop,horiz pos");
    define_write_fn!(dmacon, u16, "DMA control write (clear or set)");
    define_write_fn!(clxcon, u16, "Collision control");
    define_write_fn!(intena, u16, "Interrupt enable bits (clear or set bits)");
    define_write_fn!(intreq, u16, "Interrupt request bits (clear or set bits)");
    define_write_fn!(adkcon, u16, "Audio,disk,UART control");
    define_write_fn!(bplcon0, u16, "Bitplane control (miscellaneous control bits)");
    define_write_fn!(bplcon1, u16, "Bitplane control (scroll value)");
    define_write_fn!(bplcon2, u16, "Bitplane control (video priority control)");
    define_write_fn!(bplcon3, u16, "Bitplane control (enhanced features)");
    define_write_fn!(bpl1mod, u16, "Bitplane modulo (odd planes)");
    define_write_fn!(bpl2mod, u16, "Bitplane modulo (even planes)");
    define_write_fn!(bplcon4, u16, "(bitplane and sprite-masks)");
    define_write_fn!(clxcon2, u16, "control");
    define_write_fn!(htotal, u16, "number count, horiz line (VARBEAMEN=1)");
    define_write_fn!(hsstop, u16, "line position for HSYNC stop");
    define_write_fn!(hbstrt, u16, "line position for HBLANK start");
    define_write_fn!(hbstop, u16, "line position for HBLANK stop");
    define_write_fn!(vtotal, u16, "numbered vertical line (VARBEAMEN=1)");
    define_write_fn!(vsstop, u16, "line position for VSYNC stop");
    define_write_fn!(vbstrt, u16, "line for VBLANK start");
    define_write_fn!(vbstop, u16, "line for VBLANK stop");
    define_write_fn!(sprhstrt, u16, "sprite vertical start");
    define_write_fn!(sprhstop, u16, "sprite vertical stop");
    define_write_fn!(bplhstrt, u16, "bit plane vertical start");
    define_write_fn!(bplhstop, u16, "bit plane vertical stop");
    define_write_fn!(hhposw, u16, "mode hires H beam counter write");
    define_read_fn!(hhposr, u16, "mode hires H beam counter read");
    define_write_fn!(beamcon0, u16, "Beam counter control register (SHRES,UHRES,PAL)");
    define_write_fn!(hsstrt, u16, "sync start (VARHSY)");
    define_write_fn!(vsstrt, u16, "sync start (VARVSY)");
    define_write_fn!(hcenter, u16, "position for Vsync on interlace");
    define_write_fn!(diwhigh, u16, "window - upper bits for start/stop");
    define_write_fn!(fmode, u16, "register");
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
    /// Audio channels
    pub aud: [AudChannel; 4],
    /// Bitplane pointers
    pub bplpt: [u32; 8],
    bplcon0: u16,
    bplcon1: u16,
    bplcon2: u16,
    bplcon3: u16,
    bpl1mod: u16,
    bpl2mod: u16,
    bplcon4: u16,
    clxcon2: u16,
    /// Bitplane data
    pub bpldat: [u16; 8],
    /// Sprite pointers
    pub sprpt: [u32; 8],
    /// Sprite data
    pub spr: [SpriteDef; 8],
    /// Color table
    pub color: [u16; 32],
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
    /// Location
    pub ptr: u32,
    /// Length
    pub len: u16,
    /// Period
    pub per: u16,
    /// Volume
    pub vol: u16,
    /// Data
    pub dat: u16,
    pad: [u16; 2],
}

#[repr(C)]
pub struct SpriteDef {
    /// Vert,horiz start pos data
    pub pos: u16,
    /// Position and control data
    pub ctl: u16,
    /// Image data register A
    pub dataa: u16,
    /// Image data register B
    pub datab: u16,
}

/// Interrupt control bits
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

/// DMA control bits
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
        0x03FF
    }
}

// Custom offsets
#[repr(u16)]
pub enum CustomOffset {
    /// Blitter dest. early read (dummy address)
    BLTDDAT = 0x000,
    /// Dma control (and blitter status) read
    Dmaconr = 0x002,
    /// Read vertical most sig. bits (and frame flop)
    Vposr = 0x004 ,
    /// Read vert and horiz position of beam
    Vhposr = 0x006 ,
    /// Disk data early read (dummy address)
    Dskdatr = 0x008,
    /// Joystick-mouse 0 data (vert, horiz)
    Joy0dat = 0x00a,
    /// Joystick-mouse 1 data (vert, horiz)
    Joy1dat = 0x00c,
    /// Collision data reg. (read and clear)
    Clxdat = 0x00e,
    /// Audio,disk control register read
    Adkconr = 0x010,
    /// Pot counter data left pair (vert, horiz)
    Pot0dat = 0x012,
    /// Pot counter data right pair (vert, horiz)
    Pot1dat = 0x014,
    /// Pot pin data read
    Potinp = 0x016,
    /// Serial port data and status read
    Serdatr = 0x018,
    /// Disk data byte and status read
    Dskbytr = 0x01a,
    /// Interrupt enable bits read
    Intenar = 0x01c,
    /// Interrupt request bits read
    Intreqr = 0x01e,
    /// Disk pointer (high 5 bits, was 3 bits)
    Dskpth = 0x020,
    /// Disk pointer (low 15 bits)
    Dskptl = 0x022,
    /// Disk length
    Dsklen = 0x024,
    /// Disk DMA data write
    Dskdat = 0x026,
    /// Refresh pointer
    Refptr = 0x028,
    /// Write vert most sig. bits (and frame flop)
    Vposw = 0x02a,
    /// Write vert and horiz pos of beam
    Vhposw = 0x02c,
    /// Coprocessor control
    Copcon = 0x02e,
    /// Serial port data and stop bits write
    Serdat = 0x030,
    /// Serial port period and control
    Serper = 0x032,
    /// Pot count start,pot pin drive enable data
    Potgo = 0x034,
    /// Write to all 4 joystick-mouse counters at once
    Joytest = 0x036,
    /// Strobe for horiz sync with VB and EQU
    Strequ = 0x038,
    /// Strobe for horiz sync with VB (vert blank)
    Strvbl = 0x03a,
    /// Strobe for horiz sync
    Strhor = 0x03c,
    /// Strobe for identification of long horiz line
    Strlong = 0x03e,
    /// Blitter control register 0
    Bltcon0 = 0x040,
    /// Blitter control register 1
    Bltcon1 = 0x042,
    /// Blitter first word mask for source A
    Bltafwm = 0x044,
    /// Blitter last word mask for source A
    Bltalwm = 0x046,
    /// Blitter pointer to source C (high 5 bits, was 3 bits)
    Bltcpth = 0x048,
    /// Blitter pointer to source C (low 15 bits)
    Bltcptl = 0x04a,
    /// Blitter pointer to source B (high 5 bits, was 3 bits)
    Bltbpth = 0x04c,
    /// Blitter pointer to source B (low 15 bits)
    Bltbptl = 0x04e,
    /// Blitter pointer to source A (high 5 bits, was 3 bits)
    Bltapth = 0x050,
    /// Blitter pointer to source A (low 15 bits)
    Bltaptl = 0x052,
    /// Blitter pointer to dest D (high 5 bits, was 3 bits)
    Bltdpth = 0x054,
    /// Blitter pointer to dest D (low 15 bits)
    Bltdptl = 0x056,
    /// Blitter start and size (win/width,height)
    Bltsize = 0x058,
    /// control 0, lower 8 bits (minterms)
    Bltcon0l = 0x05a,
    /// V size (for 15 bit vertical size)
    Bltsizv = 0x05c,
    /// H size and start (for 11 bit H size)
    Bltsizh = 0x05e,
    /// Blitter modulo for source C
    Bltcmod = 0x060,
    /// Blitter modulo for source B
    Bltbmod = 0x062,
    /// Blitter modulo for source A
    Bltamod = 0x064,
    /// Blitter modulo for dest D
    Bltdmod = 0x066,
    /// Blitter source C data register
    Bltcdat = 0x070,
    /// Blitter source B data register
    Bltbdat = 0x072,
    /// Blitter source A data register
    Bltadat = 0x074,
    /// . logic UHRES sprite pointer and data identifier
    Sprhdat = 0x078,
    /// . logic UHRES bit plane identifier
    Bplhdat = 0x07a,
    /// revision level for Denise/Lisa (video out chip)
    Deniseid = 0x07c,
    /// Disk sync pattern reg for disk read
    Dsksync = 0x07e,
    /// Coprocessor 1st location (high 5 bits,was 3 bits)
    Cop1lch = 0x080,
    /// Coprocessor 1st location (low 15 bits)
    Cop1lcl = 0x082,
    /// Coprocessor 2nd location(high 5 bits,was 3 bits)
    Cop2lch = 0x084,
    /// Coprocessor 2nd location (low 15 bits)
    Cop2lcl = 0x086,
    /// Coprocessor restart at 1st location
    Copjmp1 = 0x088,
    /// Coprocessor restart at 2nd location
    Copjmp2 = 0x08a,
    /// Coprocessor inst fetch identify
    Copins = 0x08c,
    /// Display window start (upper left vert,horiz pos)
    Diwstrt = 0x08e,
    /// Display window stop (lower right vert,horiz pos)
    Diwstop = 0x090,
    /// Display bit plane data fetch start,horiz pos
    Ddfstrt = 0x092,
    /// Display bit plane data fetch stop,horiz pos
    Ddfstop = 0x094,
    /// DMA control write (clear or set)
    Dmacon = 0x096,
    /// Collision control
    Clxcon = 0x098,
    /// Interrupt enable bits (clear or set bits)
    Intena = 0x09a,
    /// Interrupt request bits (clear or set bits)
    Intreq = 0x09c,
    /// Audio,disk,UART control
    Adkcon = 0x09e,
    /// Audio channel 0 location (high 5 bits was 3 bits)
    Aud0lch = 0x0a0,
    /// Audio channel 0 location (low 15 bits)
    Aud0lcl = 0x0a2,
    /// Audio channel 0 length
    Aud0len = 0x0a4,
    /// Audio channel 0 period
    Aud0per = 0x0a6,
    /// Audio channel 0 volume
    Aud0vol = 0x0a8,
    /// Audio channel 0 data
    Aud0dat = 0x0aa,
    /// Audio channel 1 location (high 5 bits was 3 bits)
    Aud1lch = 0x0b0,
    /// Audio channel 1 location (low 15 bits)
    Aud1lcl = 0x0b2,
    /// Audio channel 1 length
    Aud1len = 0x0b4,
    /// Audio channel 1 period
    Aud1per = 0x0b6,
    /// Audio channel 1 volume
    Aud1vol = 0x0b8,
    /// Audio channel 1 data
    Aud1dat = 0x0ba,
    /// Audio channel 2 location (high 5 bits was 3 bits)
    Aud2lch = 0x0c0,
    /// Audio channel 2 location (low 15 bits)
    Aud2lcl = 0x0c2,
    /// Audio channel 2 length
    Aud2len = 0x0c4,
    /// Audio channel 2 period
    Aud2per = 0x0c6,
    /// Audio channel 2 volume
    Aud2vol = 0x0c8,
    /// Audio channel 2 data
    Aud2dat = 0x0ca,
    /// Audio channel 3 location (high 5 bits was 3 bits)
    Aud3lch = 0x0d0,
    /// Audio channel 3 location (low 15 bits)
    Aud3lcl = 0x0d2,
    /// Audio channel 3 length
    Aud3len = 0x0d4,
    /// Audio channel 3 period
    Aud3per = 0x0d6,
    /// Audio channel 3 volume
    Aud3vol = 0x0d8,
    /// Audio channel 3 data
    Aud3dat = 0x0da,
    /// Bitplane pointer 1 (high 5 bits was 3 bits)
    Bpl1pth = 0x0e0,
    /// Bitplane pointer 1 (low 15 bits)
    Bpl1ptl = 0x0e2,
    /// Bitplane pointer 2 (high 5 bits was 3 bits)
    Bpl2pth = 0x0e4,
    /// Bitplane pointer 2 (low 15 bits)
    Bpl2ptl = 0x0e6,
    /// Bitplane pointer 3 (high 5 bits was 3 bits)
    Bpl3pth = 0x0e8,
    /// Bitplane pointer 3 (low 15 bits)
    Bpl3ptl = 0x0ea,
    /// Bitplane pointer 4 (high 5 bits was 3 bits)
    Bpl4pth = 0x0ec,
    /// Bitplane pointer 4 (low 15 bits)
    Bpl4ptl = 0x0ee,
    /// Bitplane pointer 5 (high 5 bits was 3 bits)
    Bpl5pth = 0x0f0,
    /// Bitplane pointer 5 (low 15 bits)
    Bpl5ptl = 0x0f2,
    /// Bitplane pointer 6 (high 5 bits was 3 bits)
    Bpl6pth = 0x0f4,
    /// Bitplane pointer 6 (low 15 bits)
    Bpl6ptl = 0x0f6,
    /// 7 (high 5 bits was 3 bits)
    Bpl7pth = 0x0f8,
    /// 7 (low 15 bits)
    Bpl7ptl = 0x0fa,
    /// 8 (high 5 bits was 3 bits)
    Bpl8pth = 0x0fc,
    /// 8 (low 15 bits)
    Bpl8ptl = 0x0fe,
    /// Bitplane control (miscellaneous control bits)
    Bplcon0 = 0x100,
    /// Bitplane control (scroll value)
    Bplcon1 = 0x102,
    /// Bitplane control (video priority control)
    Bplcon2 = 0x104,
    /// Bitplane control (enhanced features)
    Bplcon3 = 0x106,
    /// Bitplane modulo (odd planes)
    Bpl1mod = 0x108,
    /// Bitplane modulo (even planes)
    Bpl2mod = 0x10a,
    /// (bitplane and sprite-masks)
    Bplcon4 = 0x10c,
    /// control
    Clxcon2 = 0x10e,
    /// Bitplane 1 data (parallel to serial convert)
    Bpl1dat = 0x110,
    /// Bitplane 2 data (parallel to serial convert)
    Bpl2dat = 0x112,
    /// Bitplane 3 data (parallel to serial convert)
    Bpl3dat = 0x114,
    /// Bitplane 4 data (parallel to serial convert)
    Bpl4dat = 0x116,
    /// Bitplane 5 data (parallel to serial convert)
    Bpl5dat = 0x118,
    /// Bitplane 6 data (parallel to serial convert)
    Bpl6dat = 0x11a,
    /// data (parallel to serial convert)
    Bpl7dat = 0x11c,
    /// data (parallel to serial convert)
    Bpl8dat = 0x11e,
    /// Sprite 0 pointer (high 5 bits was 3 bits)
    Spr0pth = 0x120,
    /// Sprite 0 pointer (low 15 bits)
    Spr0ptl = 0x122,
    /// Sprite 1 pointer (high 5 bits was 3 bits)
    Spr1pth = 0x124,
    /// Sprite 1 pointer (low 15 bits)
    Spr1ptl = 0x126,
    /// Sprite 2 pointer (high 5 bits was 3 bits)
    Spr2pth = 0x128,
    /// Sprite 2 pointer (low 15 bits)
    Spr2ptl = 0x12a,
    /// Sprite 3 pointer (high 5 bits was 3 bits)
    Spr3pth = 0x12c,
    /// Sprite 3 pointer (low 15 bits)
    Spr3ptl = 0x12e,
    /// Sprite 4 pointer (high 5 bits was 3 bits)
    Spr4pth = 0x130,
    /// Sprite 4 pointer (low 15 bits)
    Spr4ptl = 0x132,
    /// Sprite 5 pointer (high 5 bits was 3 bits)
    Spr5pth = 0x134,
    /// Sprite 5 pointer (low 15 bits)
    Spr5ptl = 0x136,
    /// Sprite 6 pointer (high 5 bits was 3 bits)
    Spr6pth = 0x138,
    /// Sprite 6 pointer (low 15 bits)
    Spr6ptl = 0x13a,
    /// Sprite 7 pointer (high 5 bits was 3 bits)
    Spr7pth = 0x13c,
    /// Sprite 7 pointer (low 15 bits)
    Spr7ptl = 0x13e,
    /// Sprite 0 vert,horiz start pos data
    Spr0pos = 0x140,
    /// Sprite 0 position and control data
    Spr0ctl = 0x142,
    /// Sprite 0 image data register A
    Spr0data = 0x144,
    /// Sprite 0 image data register B
    Spr0datb = 0x146,
    /// Sprite 1 vert,horiz start pos data
    Spr1pos = 0x148,
    /// Sprite 1 position and control data
    Spr1ctl = 0x14a,
    /// Sprite 1 image data register A
    Spr1data = 0x14c,
    /// Sprite 1 image data register B
    Spr1datb = 0x14e,
    /// Sprite 2 vert,horiz start pos data
    Spr2pos = 0x150,
    /// Sprite 2 position and control data
    Spr2ctl = 0x152,
    /// Sprite 2 image data register A
    Spr2data = 0x154,
    /// Sprite 2 image data register B
    Spr2datb = 0x156,
    /// Sprite 3 vert,horiz start pos data
    Spr3pos = 0x158,
    /// Sprite 3 position and control data
    Spr3ctl = 0x15a,
    /// Sprite 3 image data register A
    Spr3data = 0x15c,
    /// Sprite 3 image data register B
    Spr3datb = 0x15e,
    /// Sprite 4 vert,horiz start pos data
    Spr4pos = 0x160,
    /// Sprite 4 position and control data
    Spr4ctl = 0x162,
    /// Sprite 4 image data register A
    Spr4data = 0x164,
    /// Sprite 4 image data register B
    Spr4datb = 0x166,
    /// Sprite 5 vert,horiz start pos data
    Spr5pos = 0x168,
    /// Sprite 5 position and control data
    Spr5ctl = 0x16a,
    /// Sprite 5 image data register A
    Spr5data = 0x16c,
    /// Sprite 5 image data register B
    Spr5datb = 0x16e,
    /// Sprite 6 vert,horiz start pos data
    Spr6pos = 0x170,
    /// Sprite 6 position and control data
    Spr6ctl = 0x172,
    /// Sprite 6 image data register A
    Spr6data = 0x174,
    /// Sprite 6 image data register B
    Spr6datb = 0x176,
    /// Sprite 7 vert,horiz start pos data
    Spr7pos = 0x178,
    /// Sprite 7 position and control data
    Spr7ctl = 0x17a,
    /// Sprite 7 image data register A
    Spr7data = 0x17c,
    /// Sprite 7 image data register B
    Spr7datb = 0x17e,
    /// Color table 0
    Color00 = 0x180,
    /// Color table 1
    Color01 = 0x182,
    /// Color table 2
    Color02 = 0x184,
    /// Color table 3
    Color03 = 0x186,
    /// Color table 4
    Color04 = 0x188,
    /// Color table 5
    Color05 = 0x18a,
    /// Color table 6
    Color06 = 0x18c,
    /// Color table 7
    Color07 = 0x18e,
    /// Color table 8
    Color08 = 0x190,
    /// Color table 9
    Color09 = 0x192,
    /// Color table 10
    Color10 = 0x194,
    /// Color table 11
    Color11 = 0x196,
    /// Color table 12
    Color12 = 0x198,
    /// Color table 13
    Color13 = 0x19a,
    /// Color table 14
    Color14 = 0x19c,
    /// Color table 15
    Color15 = 0x19e,
    /// Color table 16
    Color16 = 0x1a0,
    /// Color table 17
    Color17 = 0x1a2,
    /// Color table 18
    Color18 = 0x1a4,
    /// Color table 19
    Color19 = 0x1a6,
    /// Color table 20
    Color20 = 0x1a8,
    /// Color table 21
    Color21 = 0x1aa,
    /// Color table 22
    Color22 = 0x1ac,
    /// Color table 23
    Color23 = 0x1ae,
    /// Color table 24
    Color24 = 0x1b0,
    /// Color table 25
    Color25 = 0x1b2,
    /// Color table 26
    Color26 = 0x1b4,
    /// Color table 27
    Color27 = 0x1b6,
    /// Color table 28
    Color28 = 0x1b8,
    /// Color table 29
    Color29 = 0x1ba,
    /// Color table 30
    Color30 = 0x1bc,
    /// Color table 31
    Color31 = 0x1be,
    /// number count, horiz line (VARBEAMEN=1)
    Htotal = 0x1c0,
    /// line position for HSYNC stop
    Hsstop = 0x1c2,
    /// line position for HBLANK start
    Hbstrt = 0x1c4,
    /// line position for HBLANK stop
    Hbstop = 0x1c6,
    /// numbered vertical line (VARBEAMEN=1)
    Vtotal = 0x1c8,
    /// line position for VSYNC stop
    Vsstop = 0x1ca,
    /// line for VBLANK start
    Vbstrt = 0x1cc,
    /// line for VBLANK stop
    Vbstop = 0x1ce,
    /// sprite vertical start
    Sprhstrt = 0x1d0,
    /// sprite vertical stop
    Sprhstop = 0x1d2,
    /// bit plane vertical start
    Bplhstrt = 0x1d4,
    /// bit plane vertical stop
    Bplhstop = 0x1d6,
    /// mode hires H beam counter write
    Hhposw = 0x1d8,
    /// mode hires H beam counter read
    Hhposr = 0x1da,
    /// Beam counter control register (SHRES,UHRES,PAL)
    Beamcon0 = 0x1dc,
    /// sync start (VARHSY)
    Hsstrt = 0x1de,
    /// sync start (VARVSY)
    Vsstrt = 0x1e0,
    /// position for Vsync on interlace
    Hcenter = 0x1e2,
    /// window - upper bits for start/stop
    Diwhigh = 0x1e4,
    /// bit plane modulo
    Bplhmod = 0x1e6,
    /// sprite pointer (high 5 bits)
    Sprhpth = 0x1e8,
    /// sprite pointer (low 15 bits)
    Sprhptl = 0x1ea,
    /// (UHRES) bitplane pointer (hi 5 bits)
    Bplhpth = 0x1ec,
    /// (UHRES) bitplane pointer (lo 15 bits)
    Bplhptl = 0x1ee,
    /// register
    Fmode = 0x1fc,
}

impl CustomOffset {
    pub const fn as_u16(self) -> u16 {
        self as u16
    }
}
