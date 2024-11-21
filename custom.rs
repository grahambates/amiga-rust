pub static mut CUSTOM: *mut Custom = 0xdff000 as *mut Custom;

#[repr(C)]
pub struct Custom {
    pub bltddat: u16,
    pub dmaconr: u16,
    pub vposr: u32,
    pub dskdatr: u16,
    pub joy0dat: u16,
    pub joy1dat: u16,
    pub clxdat: u16,
    pub adkconr: u16,
    pub pot0dat: u16,
    pub pot1dat: u16,
    pub potinp: u16,
    pub serdatr: u16,
    pub dskbytr: u16,
    pub intenar: u16,
    pub intreqr: u16,
    pub dskpt: u32,
    pub dsklen: u16,
    pub dskdat: u16,
    pub refptr: u16,
    pub vposw: u16,
    pub vhposw: u16,
    pub copcon: u16,
    pub serdat: u16,
    pub serper: u16,
    pub potgo: u16,
    pub joytest: u16,
    pub strequ: u16,
    pub strvbl: u16,
    pub strhor: u16,
    pub strlong: u16,
    pub bltcon0: u16,
    pub bltcon1: u16,
    pub bltafwm: u16,
    pub bltalwm: u16,
    pub bltcpt: u32,
    pub bltbpt: u32,
    pub bltapt: u32,
    pub bltdpt: u32,
    pub bltsize: u16,
    pub pad2d: u8,
    pub bltcon0l: u8,
    pub bltsizv: u16,
    pub bltsizh: u16,
    pub bltcmod: u16,
    pub bltbmod: u16,
    pub bltamod: u16,
    pub bltdmod: u16,
    pub pad34: [u16; 4],
    pub bltcdat: u16,
    pub bltbdat: u16,
    pub bltadat: u16,
    pub pad3b: [u16; 3],
    pub deniseid: u16,
    pub dsksync: u16,
    pub cop1lc: u32,
    pub cop2lc: u32,
    pub copjmp1: u16,
    pub copjmp2: u16,
    pub copins: u16,
    pub diwstrt: u16,
    pub diwstop: u16,
    pub ddfstrt: u16,
    pub ddfstop: u16,
    pub dmacon: u16,
    pub clxcon: u16,
    pub intena: u16,
    pub intreq: u16,
    pub adkcon: u16,
    pub aud: [AudChannel; 4],
    pub bplpt: [u32; 8],
    pub bplcon0: u16,
    pub bplcon1: u16,
    pub bplcon2: u16,
    pub bplcon3: u16,
    pub bpl1mod: u16,
    pub bpl2mod: u16,
    pub bplcon4: u16,
    pub clxcon2: u16,
    pub bpldat: [u16; 8],
    pub sprpt: [u32; 8],
    pub spr: [SpriteDef; 8],
    pub color: [u16; 32],
    pub htotal: u16,
    pub hsstop: u16,
    pub hbstrt: u16,
    pub hbstop: u16,
    pub vtotal: u16,
    pub vsstop: u16,
    pub vbstrt: u16,
    pub vbstop: u16,
    pub sprhstrt: u16,
    pub sprhstop: u16,
    pub bplhstrt: u16,
    pub bplhstop: u16,
    pub hhposw: u16,
    pub hhposr: u16,
    pub beamcon0: u16,
    pub hsstrt: u16,
    pub vsstrt: u16,
    pub hcenter: u16,
    pub diwhigh: u16,
    pub padf3: [u16; 11],
    pub fmode: u16,
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

// Constants for INTB (Interrupt Bits)
pub const INTB_SETCLR: u16 = 15;
pub const INTB_INTEN: u16 = 14;
pub const INTB_EXTER: u16 = 13;
pub const INTB_DSKSYNC: u16 = 12;
pub const INTB_RBF: u16 = 11;
pub const INTB_AUD3: u16 = 10;
pub const INTB_AUD2: u16 = 9;
pub const INTB_AUD1: u16 = 8;
pub const INTB_AUD0: u16 = 7;
pub const INTB_BLIT: u16 = 6;
pub const INTB_VERTB: u16 = 5;
pub const INTB_COPER: u16 = 4;
pub const INTB_PORTS: u16 = 3;
pub const INTB_SOFTINT: u16 = 2;
pub const INTB_DSKBLK: u16 = 1;
pub const INTB_TBE: u16 = 0;

// Interrupt Flags
pub const INTF_SETCLR: u16 = 1 << INTB_SETCLR;
pub const INTF_INTEN: u16 = 1 << INTB_INTEN;
pub const INTF_EXTER: u16 = 1 << INTB_EXTER;
pub const INTF_DSKSYNC: u16 = 1 << INTB_DSKSYNC;
pub const INTF_RBF: u16 = 1 << INTB_RBF;
pub const INTF_AUD3: u16 = 1 << INTB_AUD3;
pub const INTF_AUD2: u16 = 1 << INTB_AUD2;
pub const INTF_AUD1: u16 = 1 << INTB_AUD1;
pub const INTF_AUD0: u16 = 1 << INTB_AUD0;
pub const INTF_BLIT: u16 = 1 << INTB_BLIT;
pub const INTF_VERTB: u16 = 1 << INTB_VERTB;
pub const INTF_COPER: u16 = 1 << INTB_COPER;
pub const INTF_PORTS: u16 = 1 << INTB_PORTS;
pub const INTF_SOFTINT: u16 = 1 << INTB_SOFTINT;
pub const INTF_DSKBLK: u16 = 1 << INTB_DSKBLK;
pub const INTF_TBE: u16 = 1 << INTB_TBE;
pub const INTF_ALL: u16 = 0x3FFF;

// Constants for DMA (Direct Memory Access)
pub const DMAF_SETCLR: u16 = 1 << 15;
pub const DMAF_AUD0: u16 = 1 << 0;
pub const DMAF_AUD1: u16 = 1 << 1;
pub const DMAF_AUD2: u16 = 1 << 2;
pub const DMAF_AUD3: u16 = 1 << 3;
pub const DMAF_DISK: u16 = 1 << 4;
pub const DMAF_SPRITE: u16 = 1 << 5;
pub const DMAF_BLITTER: u16 = 1 << 6;
pub const DMAF_COPPER: u16 = 1 << 7;
pub const DMAF_RASTER: u16 = 1 << 8;
pub const DMAF_MASTER: u16 = 1 << 9;
pub const DMAF_BLITHOG: u16 = 1 << 10;
pub const DMAF_AUDIO: u16 = 0x000F;
pub const DMAF_ALL: u16 = 0x01FF;

// Custom offsets
pub const BLTDDAT: u16 = 0x000;
pub const DMACONR: u16 = 0x002;
pub const VPOSR: u16 = 0x004;
pub const VHPOSR: u16 = 0x006;
pub const DSKDATR: u16 = 0x008;
pub const JOY0DAT: u16 = 0x00A;
pub const JOY1DAT: u16 = 0x00C;
pub const CLXDAT: u16 = 0x00E;
pub const ADKCONR: u16 = 0x010;
pub const POT0DAT: u16 = 0x012;
pub const POT1DAT: u16 = 0x014;
pub const POTINP: u16 = 0x016;
pub const SERDATR: u16 = 0x018;
pub const DSKBYTR: u16 = 0x01A;
pub const INTENAR: u16 = 0x01C;
pub const INTREQR: u16 = 0x01E;
pub const DSKPT: u16 = 0x020;
pub const DSKLEN: u16 = 0x024;
pub const DSKDAT: u16 = 0x026;
pub const REFPTR: u16 = 0x028;
pub const VPOSW: u16 = 0x02A;
pub const VHPOSW: u16 = 0x02C;
pub const COPCON: u16 = 0x02E;
pub const SERDAT: u16 = 0x030;
pub const SERPER: u16 = 0x032;
pub const POTGO: u16 = 0x034;
pub const JOYTEST: u16 = 0x036;
pub const STR: u16 = 0x038;
pub const STRVBL: u16 = 0x03A;
pub const STRHOR: u16 = 0x03C;
pub const STRLONG: u16 = 0x03E;
pub const BLTCON0: u16 = 0x040;
pub const BLTCON1: u16 = 0x042;
pub const BLTAFWM: u16 = 0x044;
pub const BLTALWM: u16 = 0x046;
pub const BLTCPT: u16 = 0x048;
pub const BLTBPT: u16 = 0x04C;
pub const BLTAPT: u16 = 0x050;
pub const BLTDPT: u16 = 0x054;
pub const BLTSIZE: u16 = 0x058;
pub const BLTCON0L: u16 = 0x05B;
pub const BLTSIZV: u16 = 0x05C;
pub const BLTSIZH: u16 = 0x05E;
pub const BLTCMOD: u16 = 0x060;
pub const BLTBMOD: u16 = 0x062;
pub const BLTAMOD: u16 = 0x064;
pub const BLTDMOD: u16 = 0x066;
pub const BLTCDAT: u16 = 0x070;
pub const BLTBDAT: u16 = 0x072;
pub const BLTADAT: u16 = 0x074;
pub const DENISEID: u16 = 0x07C;
pub const DSKSYNC: u16 = 0x07E;
pub const COP1LC: u16 = 0x080;
pub const COP2LC: u16 = 0x084;
pub const COPJMP1: u16 = 0x088;
pub const COPJMP2: u16 = 0x08A;
pub const COPINS: u16 = 0x08C;
pub const DIWSTRT: u16 = 0x08E;
pub const DIWSTOP: u16 = 0x090;
pub const DDFSTRT: u16 = 0x092;
pub const DDFSTOP: u16 = 0x094;
pub const DMACON: u16 = 0x096;
pub const CLXCON: u16 = 0x098;
pub const INTENA: u16 = 0x09A;
pub const INTREQ: u16 = 0x09C;
pub const ADKCON: u16 = 0x09E;
pub const AUD: u16 = 0x0A0;
pub const AUD0: u16 = 0x0A0;
pub const AUD1: u16 = 0x0B0;
pub const AUD2: u16 = 0x0C0;
pub const AUD3: u16 = 0x0D0;
pub const BPLPT: u16 = 0x0E0;
pub const BPLCON0: u16 = 0x100;
pub const BPLCON1: u16 = 0x102;
pub const BPLCON2: u16 = 0x104;
pub const BPLCON3: u16 = 0x106;
pub const BPL1MOD: u16 = 0x108;
pub const BPL2MOD: u16 = 0x10A;
pub const BPLCON4: u16 = 0x10C;
pub const CLXCON2: u16 = 0x10E;
pub const BPLDAT: u16 = 0x110;
pub const SPRPT: u16 = 0x120;
pub const SPR: u16 = 0x140;
pub const COLOR: u16 = 0x180;
pub const HTOTAL: u16 = 0x1C0;
pub const HSSTOP: u16 = 0x1C2;
pub const HBSTRT: u16 = 0x1C4;
pub const HBSTOP: u16 = 0x1C6;
pub const VTOTAL: u16 = 0x1C8;
pub const VSSTOP: u16 = 0x1CA;
pub const VBSTRT: u16 = 0x1CC;
pub const VBSTOP: u16 = 0x1CE;
pub const SPRHSTRT: u16 = 0x1D0;
pub const SPRHSTOP: u16 = 0x1D2;
pub const BPLHSTRT: u16 = 0x1D4;
pub const BPLHSTOP: u16 = 0x1D6;
pub const HHPOSW: u16 = 0x1D8;
pub const HHPOSR: u16 = 0x1DA;
pub const BEAMCON0: u16 = 0x1DC;
pub const HSSTRT: u16 = 0x1DE;
pub const VSSTRT: u16 = 0x1E0;
pub const HCENTER: u16 = 0x1E2;
pub const DIWHIGH: u16 = 0x1E4;
pub const FMODE: u16 = 0x1FC;
