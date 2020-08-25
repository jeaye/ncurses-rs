/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: constants.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Constants that follow the
      ncurses library for use with
      the safe wrappers.
*/

use libc::{ c_char, c_int };
pub use libc::{LC_ALL, LC_COLLATE, LC_CTYPE, LC_MONETARY, LC_NUMERIC, LC_TIME, LC_MESSAGES};

use super::ll::*;

mod wrapped {
    use libc::{ c_char, c_int };
    use ll::chtype;
    use ll::WINDOW;

    extern "C"
    {
        pub static curscr: WINDOW;
        pub static newscr: WINDOW;
        pub static stdscr: WINDOW;
        pub static ttytype: *mut c_char;
        pub static COLORS: c_int;
        pub static COLOR_PAIRS: c_int;
        pub static COLS: c_int;
        pub static ESCDELAY: c_int;
        pub static LINES: c_int;
        pub static TABSIZE: c_int;

        /* Line graphics */
        pub static mut acs_map: [chtype; 0];
    }
}

macro_rules! wrap_extern {
    ($name:ident: $t:ty) => {
        pub fn $name() -> $t {
            unsafe { wrapped::$name }
        }
    }
}

wrap_extern!(curscr: WINDOW);
wrap_extern!(newscr: WINDOW);
wrap_extern!(stdscr: WINDOW);
wrap_extern!(ttytype: *mut c_char);
wrap_extern!(COLORS: c_int);
wrap_extern!(COLOR_PAIRS: c_int);
wrap_extern!(COLS: c_int);
wrap_extern!(ESCDELAY: c_int);
wrap_extern!(LINES: c_int);
wrap_extern!(TABSIZE: c_int);
pub fn acs_map() -> *const chtype {
    unsafe {
        &wrapped::acs_map as *const chtype
    }
}


/* Success/Failure. */
pub const ERR: i32 = -1;
pub const OK: i32 = 0;
pub const TRUE: c_bool = 1;
pub const FALSE: c_bool = 0;

/* Attributes. */
pub const NCURSES_ATTR_SHIFT: u32 = 8u32;

/* Colors */
pub const COLOR_BLACK: i16 = 0;
pub const COLOR_RED: i16 = 1;
pub const COLOR_GREEN: i16 = 2;
pub const COLOR_YELLOW: i16 = 3;
pub const COLOR_BLUE: i16 = 4;
pub const COLOR_MAGENTA: i16 = 5;
pub const COLOR_CYAN: i16 = 6;
pub const COLOR_WHITE: i16 = 7;

/* Values for the _flags member */
pub const _SUBWIN: i32 = 0x01;	/* is this a sub-window? */
pub const _ENDLINE: i32 = 0x02;	/* is the window flush right? */
pub const _FULLWIN: i32 = 0x04;	/* is the window full-screen? */
pub const _SCROLLWIN: i32 = 0x08;	/* bottom edge is at screen bottom? */
pub const _ISPAD: i32 = 0x10;	/* is this window a pad? */
pub const _HASMOVED: i32 = 0x20;	/* has cursor moved since last refresh? */
pub const _WRAPPED: i32 = 0x40;	/* cursor was just wrappped */

/*
 * This value is used in the firstchar and lastchar fields to mark
 * unchanged lines
 */
pub const _NOCHANGE: i32 = -1;

/*
 * This value is used in the oldindex field to mark lines created by insertions
 * and scrolls.
 */
pub const _NEWINDEX: i32 = -1;

/* Keys */
pub const KEY_CODE_YES: i32 =	0x100;		/* A wchar_t contains a key code */
pub const KEY_MIN: i32 = 0x101;		/* Minimum curses key */
pub const KEY_BREAK: i32 = 0x101;		/* Break key(unreliable) */
pub const KEY_SRESET: i32 =	0x158;		/* Soft(partial) reset(unreliable) */
pub const KEY_RESET: i32 = 0x159;		/* Reset or hard reset(unreliable) */
pub const KEY_DOWN: i32 =	0x102;		/* down-arrow key */
pub const KEY_UP: i32=		0x103;		/* up-arrow key */
pub const KEY_LEFT: i32=	0x104;		/* left-arrow key */
pub const KEY_RIGHT: i32=	0x105;		/* right-arrow key */
pub const KEY_HOME: i32=	0x106;		/* home key */
pub const KEY_BACKSPACE: i32=	0x107;		/* backspace key */
pub const KEY_F0: i32=		0x108;		/* Function keys. Space for 64 */
pub const KEY_F1: i32=		0x109;
pub const KEY_F2: i32=		0x10a;
pub const KEY_F3: i32=		0x10b;
pub const KEY_F4: i32=		0x10c;
pub const KEY_F5: i32=		0x10d;
pub const KEY_F6: i32=		0x10e;
pub const KEY_F7: i32=		0x10f;
pub const KEY_F8: i32=		0x110;
pub const KEY_F9: i32=		0x111;
pub const KEY_F10: i32=		0x112;
pub const KEY_F11: i32=		0x113;
pub const KEY_F12: i32=		0x114;
pub const KEY_F13: i32=		0x115;
pub const KEY_F14: i32=		0x116;
pub const KEY_F15: i32=		0x117;
pub const KEY_DL: i32=		0x148;		/* delete-line key */
pub const KEY_IL: i32=		0x149;		/* insert-line key */
pub const KEY_DC: i32=		0x14a;		/* delete-character key */
pub const KEY_IC: i32=		0x14b;		/* insert-character key */
pub const KEY_EIC: i32=		0x14c;		/* sent by rmir or smir in insert mode */
pub const KEY_CLEAR: i32=	0x14d;		/* clear-screen or erase key */
pub const KEY_EOS: i32=		0x14e;		/* clear-to-end-of-screen key */
pub const KEY_EOL: i32=		0x14f;		/* clear-to-end-of-line key */
pub const KEY_SF: i32=		0x150;		/* scroll-forward key */
pub const KEY_SR: i32=		0x151;		/* scroll-backward key */
pub const KEY_NPAGE: i32=	0x152;		/* next-page key */
pub const KEY_PPAGE: i32=	0x153;		/* previous-page key */
pub const KEY_STAB: i32=	0x154;		/* set-tab key */
pub const KEY_CTAB: i32=	0x155;		/* clear-tab key */
pub const KEY_CATAB: i32=	0x156;		/* clear-all-tabs key */
pub const KEY_ENTER: i32=	0x157;		/* enter/send key */
pub const KEY_PRINT: i32=	0x15a;		/* print key */
pub const KEY_LL: i32=		0x15b;		/* lower-left key(home down) */
pub const KEY_A1: i32=		0x15c;		/* upper left of keypad */
pub const KEY_A3: i32=		0x15d;		/* upper right of keypad */
pub const KEY_B2: i32=		0x15e;		/* center of keypad */
pub const KEY_C1: i32=		0x15f;		/* lower left of keypad */
pub const KEY_C3: i32=		0x160;		/* lower right of keypad */
pub const KEY_BTAB: i32=	0x161;		/* back-tab key */
pub const KEY_BEG: i32=		0x162;		/* begin key */
pub const KEY_CANCEL: i32=	0x163;		/* cancel key */
pub const KEY_CLOSE: i32=	0x164;		/* close key */
pub const KEY_COMMAND: i32=	0x165;		/* command key */
pub const KEY_COPY: i32=	0x166;		/* copy key */
pub const KEY_CREATE: i32=	0x167;		/* create key */
pub const KEY_END: i32=		0x168;		/* end key */
pub const KEY_EXIT: i32=	0x169;		/* exit key */
pub const KEY_FIND: i32=	0x16a;		/* find key */
pub const KEY_HELP: i32=	0x16b;		/* help key */
pub const KEY_MARK: i32=	0x16c;		/* mark key */
pub const KEY_MESSAGE: i32=	0x16d;		/* message key */
pub const KEY_MOVE: i32=	0x16e;		/* move key */
pub const KEY_NEXT: i32=	0x16f;		/* next key */
pub const KEY_OPEN: i32=	0x170;		/* open key */
pub const KEY_OPTIONS: i32=	0x171;		/* options key */
pub const KEY_PREVIOUS: i32=	0x172;		/* previous key */
pub const KEY_REDO: i32=	0x173;		/* redo key */
pub const KEY_REFERENCE: i32=	0x174;		/* reference key */
pub const KEY_REFRESH: i32=	0x175;		/* refresh key */
pub const KEY_REPLACE: i32=	0x176;		/* replace key */
pub const KEY_RESTART: i32=	0x177;		/* restart key */
pub const KEY_RESUME: i32=	0x178;		/* resume key */
pub const KEY_SAVE: i32=	0x179;		/* save key */
pub const KEY_SBEG: i32=	0x17a;		/* shifted begin key */
pub const KEY_SCANCEL: i32=	0x17b;		/* shifted cancel key */
pub const KEY_SCOMMAND: i32=	0x17c;		/* shifted command key */
pub const KEY_SCOPY: i32=	0x17d;		/* shifted copy key */
pub const KEY_SCREATE: i32=	0x17e;		/* shifted create key */
pub const KEY_SDC	: i32=	0x17f;		/* shifted delete-character key */
pub const KEY_SDL	: i32=	0x180;		/* shifted delete-line key */
pub const KEY_SELECT: i32=	0x181;		/* select key */
pub const KEY_SEND: i32=	0x182;		/* shifted end key */
pub const KEY_SEOL: i32=	0x183;		/* shifted clear-to-end-of-line key */
pub const KEY_SEXIT: i32=	0x184;		/* shifted exit key */
pub const KEY_SFIND: i32=	0x185;		/* shifted find key */
pub const KEY_SHELP: i32=	0x186;		/* shifted help key */
pub const KEY_SHOME: i32=	0x187;		/* shifted home key */
pub const KEY_SIC	: i32=	0x188;		/* shifted insert-character key */
pub const KEY_SLEFT: i32=	0x189;		/* shifted left-arrow key */
pub const KEY_SMESSAGE: i32=	0x18a;		/* shifted message key */
pub const KEY_SMOVE: i32=	0x18b;		/* shifted move key */
pub const KEY_SNEXT: i32=	0x18c;		/* shifted next key */
pub const KEY_SOPTIONS: i32=	0x18d;		/* shifted options key */
pub const KEY_SPREVIOUS: i32=	0x18e;		/* shifted previous key */
pub const KEY_SPRINT: i32=	0x18f;		/* shifted print key */
pub const KEY_SREDO: i32=	0x190;		/* shifted redo key */
pub const KEY_SREPLACE: i32=	0x191;		/* shifted replace key */
pub const KEY_SRIGHT: i32=	0x192;		/* shifted right-arrow key */
pub const KEY_SRSUME: i32=	0x193;		/* shifted resume key */
pub const KEY_SSAVE: i32=	0x194;		/* shifted save key */
pub const KEY_SSUSPEND: i32=	0x195;		/* shifted suspend key */
pub const KEY_SUNDO: i32=	0x196;		/* shifted undo key */
pub const KEY_SUSPEND: i32=	0x197;		/* suspend key */
pub const KEY_UNDO: i32=	0x198;		/* undo key */
pub const KEY_MOUSE: i32=	0x199;		/* Mouse event has occurred */
pub const KEY_RESIZE: i32=	0x19a;		/* Terminal resize event */
pub const KEY_EVENT: i32=	0x19b;		/* We were interrupted by an event */
pub const KEY_MAX: i32=	0x1ff;		/* Maximum key value is 0633 */

#[cfg(feature="mouse_v1")]
pub const NCURSES_MOUSE_VERSION: i32= 1;

#[cfg(not(feature="mouse_v1"))]
pub const NCURSES_MOUSE_VERSION: i32= 2;

#[cfg(feature = "mouse_v1")]
const MASK_SHIFT: i32 = 6;
#[cfg(not(feature = "mouse_v1"))]
const MASK_SHIFT: i32 = 5;

#[cfg(feature = "mouse_v1")]
const MODIFIER_SHIFT: i32 = 5;
#[cfg(not(feature = "mouse_v1"))]
const MODIFIER_SHIFT: i32 = 6;

/* Mouse Support */
macro_rules! ncurses_mouse_mask( ($b:expr, $m:expr) => ($m << (($b - 1) * MASK_SHIFT)); );

pub const NCURSES_BUTTON_RELEASED: i32= 0x001;
pub const NCURSES_BUTTON_PRESSED: i32=  0x002;
pub const NCURSES_BUTTON_CLICKED: i32=  0x004;
pub const NCURSES_DOUBLE_CLICKED: i32=  0x008;
pub const NCURSES_TRIPLE_CLICKED: i32=  0x010;
pub const NCURSES_RESERVED_EVENT: i32=  0x020;

/* event masks */
pub const BUTTON1_RELEASED: i32=       ncurses_mouse_mask!(1, NCURSES_BUTTON_RELEASED);
pub const BUTTON1_PRESSED: i32=        ncurses_mouse_mask!(1, NCURSES_BUTTON_PRESSED);
pub const BUTTON1_CLICKED: i32=        ncurses_mouse_mask!(1, NCURSES_BUTTON_CLICKED);
pub const BUTTON1_DOUBLE_CLICKED: i32= ncurses_mouse_mask!(1, NCURSES_DOUBLE_CLICKED);
pub const BUTTON1_TRIPLE_CLICKED: i32= ncurses_mouse_mask!(1, NCURSES_TRIPLE_CLICKED);

pub const BUTTON2_RELEASED: i32=       ncurses_mouse_mask!(2, NCURSES_BUTTON_RELEASED);
pub const BUTTON2_PRESSED: i32=        ncurses_mouse_mask!(2, NCURSES_BUTTON_PRESSED);
pub const BUTTON2_CLICKED: i32=        ncurses_mouse_mask!(2, NCURSES_BUTTON_CLICKED);
pub const BUTTON2_DOUBLE_CLICKED: i32= ncurses_mouse_mask!(2, NCURSES_DOUBLE_CLICKED);
pub const BUTTON2_TRIPLE_CLICKED: i32= ncurses_mouse_mask!(2, NCURSES_TRIPLE_CLICKED);

pub const BUTTON3_RELEASED: i32=       ncurses_mouse_mask!(3, NCURSES_BUTTON_RELEASED);
pub const BUTTON3_PRESSED: i32=        ncurses_mouse_mask!(3, NCURSES_BUTTON_PRESSED);
pub const BUTTON3_CLICKED: i32=        ncurses_mouse_mask!(3, NCURSES_BUTTON_CLICKED);
pub const BUTTON3_DOUBLE_CLICKED: i32= ncurses_mouse_mask!(3, NCURSES_DOUBLE_CLICKED);
pub const BUTTON3_TRIPLE_CLICKED: i32= ncurses_mouse_mask!(3, NCURSES_TRIPLE_CLICKED);

pub const BUTTON4_RELEASED: i32=       ncurses_mouse_mask!(4, NCURSES_BUTTON_RELEASED);
pub const BUTTON4_PRESSED: i32=        ncurses_mouse_mask!(4, NCURSES_BUTTON_PRESSED);
pub const BUTTON4_CLICKED: i32=        ncurses_mouse_mask!(4, NCURSES_BUTTON_CLICKED);
pub const BUTTON4_DOUBLE_CLICKED: i32= ncurses_mouse_mask!(4, NCURSES_DOUBLE_CLICKED);
pub const BUTTON4_TRIPLE_CLICKED: i32= ncurses_mouse_mask!(4, NCURSES_TRIPLE_CLICKED);

pub const BUTTON5_RELEASED: i32=       ncurses_mouse_mask!(5, NCURSES_BUTTON_RELEASED);
pub const BUTTON5_PRESSED: i32=        ncurses_mouse_mask!(5, NCURSES_BUTTON_PRESSED);
pub const BUTTON5_CLICKED: i32=        ncurses_mouse_mask!(5, NCURSES_BUTTON_CLICKED);
pub const BUTTON5_DOUBLE_CLICKED: i32= ncurses_mouse_mask!(5, NCURSES_DOUBLE_CLICKED);
pub const BUTTON5_TRIPLE_CLICKED: i32= ncurses_mouse_mask!(5, NCURSES_TRIPLE_CLICKED);

pub const BUTTON_CTRL: i32 =           ncurses_mouse_mask!(MODIFIER_SHIFT, 0x001);
pub const BUTTON_SHIFT: i32 =          ncurses_mouse_mask!(MODIFIER_SHIFT, 0x002);
pub const BUTTON_ALT: i32 =            ncurses_mouse_mask!(MODIFIER_SHIFT, 0x004);
pub const REPORT_MOUSE_POSITION: i32 = ncurses_mouse_mask!(MODIFIER_SHIFT, 0x008);

pub const ALL_MOUSE_EVENTS: i32=       REPORT_MOUSE_POSITION - 1;

#[derive(Debug)]
#[repr(i32)]
pub enum LcCategory {
    all = LC_ALL,
    collate = LC_COLLATE,
    ctype = LC_CTYPE,
    monetary = LC_MONETARY,
    numeric = LC_NUMERIC,
    time = LC_TIME,
    messages = LC_MESSAGES,
}
