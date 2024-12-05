use core::ptr::read_volatile;

/// CIAA struct
#[repr(C)]
pub struct CIA {
    /// Port A
    pub pra: u8, //0x000
    _pad0: [u8; 0x100-1],
    /// Port B
    pub prb: u8, //0x100
    _pad1: [u8; 0x100-1],
    /// Direction for port A
    pub ddra: u8, //0200
    _pad2: [u8; 0x100-1],
    /// Direction for port B
    pub ddrb: u8, //0300
    _pad3: [u8; 0x100-1],
    /// timer A low byte
    pub talo: u8, //0400
    _pad4: [u8; 0x100-1],
    /// timer A high byte
    pub tahi: u8, //0500
    _pad5: [u8; 0x100-1],
    /// timer B low byte
    pub tblo: u8, //0600
    _pad6: [u8; 0x100-1],
    /// timer B high byte
    pub tbhi: u8, //0700
    _pad7: [u8; 0x100-1],
    /// event counter bits 7-0
    pub todlow: u8, //0800
    _pad8: [u8; 0x100-1],
    /// event counter bits 15-8
    pub todmid: u8, //0900
    _pad9: [u8; 0x100-1],
    /// event counter bits 23-16
    pub todhi: u8, //0a00
    _pad10: [u8; 0x200-1],
    /// serial data
    pub sdr: u8, //0c00
    _pad11: [u8; 0x100-1],
    /// interrupt control register
    pub icr: u8, //0d00
    _pad12: [u8; 0x100-1],
    /// Control register A
    pub cra: u8, //0e00
    _pad13: [u8; 0x100-1],
    /// Control register B
    pub crb: u8, //0f00
}

impl CIA {
    /// ciaa instance ($bfe001)
    pub fn a() -> &'static mut Self {
        unsafe { &mut *(0xbfe001 as *mut Self) }
    }
    /// ciab instance ($bfd000)
    pub fn b() -> &'static mut Self {
        unsafe { &mut *(0xbfd000 as *mut Self) }
    }

    // Getters:
    /// Port A
    pub fn pra(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.pra) }
    }
    /// Port B
    pub fn prb(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.prb) }
    }
    /// Direction for port A
    pub fn ddra(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.ddra) }
    }
    /// Direction for port B
    pub fn ddrb(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.ddrb) }
    }
    /// timer A low byte
    pub fn talo(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.talo) }
    }
    /// timer A high byte
    pub fn tahi(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.tahi) }
    }
    /// timer B low byte
    pub fn tblo(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.tblo) }
    }
    /// timer B high byte
    pub fn tbhi(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.tbhi) }
    }
    /// event counter bits 7-0
    pub fn todlow(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.todlow) }
    }
    /// event counter bits 15-8
    pub fn todmid(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.todmid) }
    }
    /// event counter bits 23-16
    pub fn todhi(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.todhi) }
    }
    /// serial data
    pub fn sdr(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.sdr) }
    }
    /// interrupt control register
    pub fn icr(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.icr) }
    }
    /// Control register A
    pub fn cra(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.cra) }
    }
    /// Control register B
    pub fn crb(&mut self) -> u8 {
        unsafe { read_volatile(&mut self.crb) }
    }

    // TODO: setters?
}

// Port definitions -- what each bit in a cia peripheral register is tied to

#[repr(u8)]
pub enum CiaAPortABit {
    Overlay = 0,
    Led = 1,
    DskChange = 2,
    DskProt = 3,
    DskTrack0 = 4,
    DskReady = 5,
    GamePort0 = 6,
    GamePort1 = 7,
}
impl CiaAPortABit {
    /// Compute the flag for the bit
    pub const fn flag(self) -> u8 {
        1 << (self as u8)
    }
}

// ciaa port B (0xbfe101) -- parallel port
// (No bit defs)

#[repr(u8)]
/// serial and printer control
pub enum CiaBPortABit {
    /// printer busy
    PrtrBusy = 0,
    /// printer paper out
    PrtrPOut = 1,
    /// printer SELECT
    PrtrSel = 2,
    /// serial Data Set Ready*
    ComDsr = 3,
    /// serial Clear to Send
    ComCts = 4,
    /// serial Carrier Detect
    ComCd = 5,
    /// serial Request to Send
    ComRts = 6,
    /// serial Data Terminal Ready
    ComDtr = 7,
}
impl CiaBPortABit {
    /// Compute the flag for the bit
    pub const fn flag(self) -> u8 {
        1 << (self as u8)
    }
}

#[repr(u8)]
/// disk control
pub enum CiaBPortBBit {
    /// disk step heads
    DskStep = 0,
    /// disk direction of seek
    DskDirec = 1,
    /// disk side select
    DskSide = 2,
    /// disk select unit 0
    DskSel0 = 3,
    /// disk select unit 1
    DskSel1 = 4,
    /// disk select unit 2
    DskSel2 = 5,
    /// disk select unit 3
    DskSel3 = 6,
    /// disk motorr*
    DskMotor = 7,
}
impl CiaBPortBBit {
    /// Compute the flag for the bit
    pub const fn flag(self) -> u8 {
        1 << (self as u8)
    }
}

// TODO:

// * interrupt control register bit numbers
// CIAICRB_TA = 0
// CIAICRB_TB = 1
// CIAICRB_ALRM = 2
// CIAICRB_SP = 3
// CIAICRB_FLG = 4
// CIAICRB_IR = 7
// CIAICRB_SETCLR = 7
//
// * control register A bit numbers
// CIACRAB_START = 0
// CIACRAB_PBON = 1
// CIACRAB_OUTMODE = 2
// CIACRAB_RUNMODE = 3
// CIACRAB_LOAD = 4
// CIACRAB_INMODE = 5
// CIACRAB_SPMODE = 6
// CIACRAB_TODIN = 7
//
// * control register B bit numbers
// CIACRBB_START = 0
// CIACRBB_PBON = 1
// CIACRBB_OUTMODE = 2
// CIACRBB_RUNMODE = 3
// CIACRBB_LOAD = 4
// CIACRBB_INMODE0 = 5
// CIACRBB_INMODE1 = 6
// CIACRBB_ALARM = 7
