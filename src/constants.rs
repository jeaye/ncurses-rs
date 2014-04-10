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
use super::ll::*;

extern
{ 
  pub static curscr: WINDOW;
  pub static newscr: WINDOW;
  pub static stdscr: WINDOW;
  pub static ttytype: *c_char;
  pub static COLORS: c_int;
  pub static COLOR_PAIRS: c_int;
  pub static COLS: c_int;
  pub static ESCDELAY: c_int;
  pub static LINES: c_int;
  pub static TABSIZE: c_int;

  /* Line graphics */
  pub static acs_map: chtype_p;
}

/* Success/Failure. */
pub static ERR: i32 = -1;
pub static OK: i32 = 0;
pub static TRUE: i32 = 1;
pub static FALSE: i32 = 0;

/* Attributes. */
pub static NCURSES_ATTR_SHIFT: u32 = 8u32;

/* Colors */
pub static COLOR_BLACK: i16 = 0;
pub static COLOR_RED: i16 = 1;
pub static COLOR_GREEN: i16 = 2;
pub static COLOR_YELLOW: i16 = 3;
pub static COLOR_BLUE: i16 = 4;
pub static COLOR_MAGENTA: i16 = 5;
pub static COLOR_CYAN: i16 = 6;
pub static COLOR_WHITE: i16 = 7;

/* Values for the _flags member */
pub static _SUBWIN: i32 = 0x01;	/* is this a sub-window? */
pub static _ENDLINE: i32 = 0x02;	/* is the window flush right? */
pub static _FULLWIN: i32 = 0x04;	/* is the window full-screen? */
pub static _SCROLLWIN: i32 = 0x08;	/* bottom edge is at screen bottom? */
pub static _ISPAD: i32 = 0x10;	/* is this window a pad? */
pub static _HASMOVED: i32 = 0x20;	/* has cursor moved since last refresh? */
pub static _WRAPPED: i32 = 0x40;	/* cursor was just wrappped */

/*
 * This value is used in the firstchar and lastchar fields to mark
 * unchanged lines
 */
pub static _NOCHANGE: i32 = -1;

/*
 * This value is used in the oldindex field to mark lines created by insertions
 * and scrolls.
 */
pub static _NEWINDEX: i32 = -1;

/* Keys */
pub static KEY_CODE_YES: i32 =	0x100;		/* A wchar_t contains a key code */
pub static KEY_MIN: i32 = 0x101;		/* Minimum curses key */
pub static KEY_BREAK: i32 = 0x101;		/* Break key(unreliable) */
pub static KEY_SRESET: i32 =	0x158;		/* Soft(partial) reset(unreliable) */
pub static KEY_RESET: i32 = 0x159;		/* Reset or hard reset(unreliable) */
pub static KEY_DOWN: i32 =	0x102;		/* down-arrow key */
pub static KEY_UP: i32=		0x103;		/* up-arrow key */
pub static KEY_LEFT: i32=	0x104;		/* left-arrow key */
pub static KEY_RIGHT: i32=	0x105;		/* right-arrow key */
pub static KEY_HOME: i32=	0x106;		/* home key */
pub static KEY_BACKSPACE: i32=	0x107;		/* backspace key */
pub static KEY_F0: i32=		0x108;		/* Function keys. Space for 64 */
pub static KEY_F1: i32=		0x109;
pub static KEY_F2: i32=		0x10a;
pub static KEY_F3: i32=		0x10b;
pub static KEY_F4: i32=		0x10c;
pub static KEY_F5: i32=		0x10d;
pub static KEY_F6: i32=		0x10e;
pub static KEY_F7: i32=		0x10f;
pub static KEY_F8: i32=		0x110;
pub static KEY_F9: i32=		0x111;
pub static KEY_F10: i32=		0x112;
pub static KEY_F11: i32=		0x113;
pub static KEY_F12: i32=		0x114;
pub static KEY_F13: i32=		0x115;
pub static KEY_F14: i32=		0x116;
pub static KEY_F15: i32=		0x117;
pub static KEY_DL: i32=		0x148;		/* delete-line key */
pub static KEY_IL: i32=		0x149;		/* insert-line key */
pub static KEY_DC: i32=		0x150;		/* delete-character key */
pub static KEY_IC: i32=		0x151;		/* insert-character key */
pub static KEY_EIC: i32=		0x152;		/* sent by rmir or smir in insert mode */
pub static KEY_CLEAR: i32=	0x14d;		/* clear-screen or erase key */
pub static KEY_EOS: i32=		0x14e;		/* clear-to-end-of-screen key */
pub static KEY_EOL: i32=		0x14f;		/* clear-to-end-of-line key */
pub static KEY_SF: i32=		0x150;		/* scroll-forward key */
pub static KEY_SR: i32=		0x151;		/* scroll-backward key */
pub static KEY_NPAGE: i32=	0x152;		/* next-page key */
pub static KEY_PPAGE: i32=	0x153;		/* previous-page key */
pub static KEY_STAB: i32=	0x154;		/* set-tab key */
pub static KEY_CTAB: i32=	0x155;		/* clear-tab key */
pub static KEY_CATAB: i32=	0x156;		/* clear-all-tabs key */
pub static KEY_ENTER: i32=	0x157;		/* enter/send key */
pub static KEY_PRINT: i32=	0x15a;		/* print key */
pub static KEY_LL: i32=		0x15b;		/* lower-left key(home down) */
pub static KEY_A1: i32=		0x15c;		/* upper left of keypad */
pub static KEY_A3: i32=		0x15d;		/* upper right of keypad */
pub static KEY_B2: i32=		0x15e;		/* center of keypad */
pub static KEY_C1: i32=		0x15f;		/* lower left of keypad */
pub static KEY_C3: i32=		0x160;		/* lower right of keypad */
pub static KEY_BTAB: i32=	0x161;		/* back-tab key */
pub static KEY_BEG: i32=		0x162;		/* begin key */
pub static KEY_CANCEL: i32=	0x163;		/* cancel key */
pub static KEY_CLOSE: i32=	0x164;		/* close key */
pub static KEY_COMMAND: i32=	0x165;		/* command key */
pub static KEY_COPY: i32=	0x166;		/* copy key */
pub static KEY_CREATE: i32=	0x167;		/* create key */
pub static KEY_END: i32=		0x168;		/* end key */
pub static KEY_EXIT: i32=	0x169;		/* exit key */
pub static KEY_FIND: i32=	0x16a;		/* find key */
pub static KEY_HELP: i32=	0x16b;		/* help key */
pub static KEY_MARK: i32=	0x16c;		/* mark key */
pub static KEY_MESSAGE: i32=	0x16d;		/* message key */
pub static KEY_MOVE: i32=	0x16e;		/* move key */
pub static KEY_NEXT: i32=	0x16f;		/* next key */
pub static KEY_OPEN: i32=	0x170;		/* open key */
pub static KEY_OPTIONS: i32=	0x171;		/* options key */
pub static KEY_PREVIOUS: i32=	0x172;		/* previous key */
pub static KEY_REDO: i32=	0x173;		/* redo key */
pub static KEY_REFERENCE: i32=	0x174;		/* reference key */
pub static KEY_REFRESH: i32=	0x175;		/* refresh key */
pub static KEY_REPLACE: i32=	0x176;		/* replace key */
pub static KEY_RESTART: i32=	0x177;		/* restart key */
pub static KEY_RESUME: i32=	0x178;		/* resume key */
pub static KEY_SAVE: i32=	0x179;		/* save key */
pub static KEY_SBEG: i32=	0x17a;		/* shifted begin key */
pub static KEY_SCANCEL: i32=	0x17b;		/* shifted cancel key */
pub static KEY_SCOMMAND: i32=	0x17c;		/* shifted command key */
pub static KEY_SCOPY: i32=	0x17d;		/* shifted copy key */
pub static KEY_SCREATE: i32=	0x17e;		/* shifted create key */
pub static KEY_SDC	: i32=	0x17f;		/* shifted delete-character key */
pub static KEY_SDL	: i32=	0x180;		/* shifted delete-line key */
pub static KEY_SELECT: i32=	0x181;		/* select key */
pub static KEY_SEND: i32=	0x182;		/* shifted end key */
pub static KEY_SEOL: i32=	0x183;		/* shifted clear-to-end-of-line key */
pub static KEY_SEXIT: i32=	0x184;		/* shifted exit key */
pub static KEY_SFIND: i32=	0x185;		/* shifted find key */
pub static KEY_SHELP: i32=	0x186;		/* shifted help key */
pub static KEY_SHOME: i32=	0x187;		/* shifted home key */
pub static KEY_SIC	: i32=	0x188;		/* shifted insert-character key */
pub static KEY_SLEFT: i32=	0x189;		/* shifted left-arrow key */
pub static KEY_SMESSAGE: i32=	0x18a;		/* shifted message key */
pub static KEY_SMOVE: i32=	0x18b;		/* shifted move key */
pub static KEY_SNEXT: i32=	0x18c;		/* shifted next key */
pub static KEY_SOPTIONS: i32=	0x18d;		/* shifted options key */
pub static KEY_SPREVIOUS: i32=	0x18e;		/* shifted previous key */
pub static KEY_SPRINT: i32=	0x18f;		/* shifted print key */
pub static KEY_SREDO: i32=	0x190;		/* shifted redo key */
pub static KEY_SREPLACE: i32=	0x191;		/* shifted replace key */
pub static KEY_SRIGHT: i32=	0x192;		/* shifted right-arrow key */
pub static KEY_SRSUME: i32=	0x193;		/* shifted resume key */
pub static KEY_SSAVE: i32=	0x194;		/* shifted save key */
pub static KEY_SSUSPEND: i32=	0x195;		/* shifted suspend key */
pub static KEY_SUNDO: i32=	0x196;		/* shifted undo key */
pub static KEY_SUSPEND: i32=	0x197;		/* suspend key */
pub static KEY_UNDO: i32=	0x198;		/* undo key */
pub static KEY_MOUSE: i32=	0x199;		/* Mouse event has occurred */
pub static KEY_RESIZE: i32=	0x19a;		/* Terminal resize event */
pub static KEY_EVENT: i32=	0x19b;		/* We were interrupted by an event */
pub static KEY_MAX: i32=	0x1ff;		/* Maximum key value is 0633 */

/* Mouse Support */
macro_rules! ncurses_mouse_mask( ($b: expr $m: expr) => ($m << (($b - 1) * 5)); )
pub static NCURSES_BUTTON_RELEASED: i32=	0x001;
pub static NCURSES_BUTTON_PRESSED: i32=		0x002;
pub static NCURSES_BUTTON_CLICKED: i32=		0x004;
pub static NCURSES_DOUBLE_CLICKED: i32=		0x008;
pub static NCURSES_TRIPLE_CLICKED: i32=		0x010;
pub static NCURSES_RESERVED_EVENT: i32=		0x020;

/* event masks */
pub static BUTTON1_RELEASED: i32=	ncurses_mouse_mask!(1 NCURSES_BUTTON_RELEASED);
pub static BUTTON1_PRESSED: i32=	ncurses_mouse_mask!(1 NCURSES_BUTTON_PRESSED);
pub static BUTTON1_CLICKED: i32=	ncurses_mouse_mask!(1 NCURSES_BUTTON_CLICKED);
pub static BUTTON1_DOUBLE_CLICKED: i32=	ncurses_mouse_mask!(1 NCURSES_DOUBLE_CLICKED);
pub static BUTTON1_TRIPLE_CLICKED: i32=	ncurses_mouse_mask!(1 NCURSES_TRIPLE_CLICKED);

pub static BUTTON2_RELEASED: i32=       ncurses_mouse_mask!(2 NCURSES_BUTTON_RELEASED);
pub static BUTTON2_PRESSED: i32=        ncurses_mouse_mask!(2 NCURSES_BUTTON_PRESSED);
pub static BUTTON2_CLICKED: i32=        ncurses_mouse_mask!(2 NCURSES_BUTTON_CLICKED);
pub static BUTTON2_DOUBLE_CLICKED: i32= ncurses_mouse_mask!(2 NCURSES_DOUBLE_CLICKED);
pub static BUTTON2_TRIPLE_CLICKED: i32= ncurses_mouse_mask!(2 NCURSES_TRIPLE_CLICKED);

pub static BUTTON3_RELEASED: i32=       ncurses_mouse_mask!(3 NCURSES_BUTTON_RELEASED);
pub static BUTTON3_PRESSED: i32=        ncurses_mouse_mask!(3 NCURSES_BUTTON_PRESSED);
pub static BUTTON3_CLICKED: i32=        ncurses_mouse_mask!(3 NCURSES_BUTTON_CLICKED);
pub static BUTTON3_DOUBLE_CLICKED: i32= ncurses_mouse_mask!(3 NCURSES_DOUBLE_CLICKED);
pub static BUTTON3_TRIPLE_CLICKED: i32= ncurses_mouse_mask!(3 NCURSES_TRIPLE_CLICKED);

pub static BUTTON4_RELEASED: i32=       ncurses_mouse_mask!(4 NCURSES_BUTTON_RELEASED);
pub static BUTTON4_PRESSED: i32=        ncurses_mouse_mask!(4 NCURSES_BUTTON_PRESSED);
pub static BUTTON4_CLICKED: i32=        ncurses_mouse_mask!(4 NCURSES_BUTTON_CLICKED);
pub static BUTTON4_DOUBLE_CLICKED: i32= ncurses_mouse_mask!(4 NCURSES_DOUBLE_CLICKED);
pub static BUTTON4_TRIPLE_CLICKED: i32= ncurses_mouse_mask!(4 NCURSES_TRIPLE_CLICKED);

pub static BUTTON5_RELEASED: i32=       ncurses_mouse_mask!(5 NCURSES_BUTTON_RELEASED);
pub static BUTTON5_PRESSED: i32=        ncurses_mouse_mask!(5 NCURSES_BUTTON_PRESSED);
pub static BUTTON5_CLICKED: i32=        ncurses_mouse_mask!(5 NCURSES_BUTTON_CLICKED);
pub static BUTTON5_DOUBLE_CLICKED: i32= ncurses_mouse_mask!(5 NCURSES_DOUBLE_CLICKED);
pub static BUTTON5_TRIPLE_CLICKED: i32= ncurses_mouse_mask!(5 NCURSES_TRIPLE_CLICKED);

pub static BUTTON_CTRL: i32=		ncurses_mouse_mask!(6 0x001);
pub static BUTTON_SHIFT: i32=		ncurses_mouse_mask!(6 0x002);
pub static BUTTON_ALT: i32=		ncurses_mouse_mask!(6 0x004);
pub static REPORT_MOUSE_POSITION: i32=	ncurses_mouse_mask!(6 0x008);

pub static ALL_MOUSE_EVENTS: i32=	REPORT_MOUSE_POSITION - 1;

/* macros to extract single event-bits from masks */
macro_rules! button_release( ($e: expr $x: expr) => (e & ncurses_mouse_mask!(x 0x001)); )
macro_rules! button_press( ($e: expr $x: expr) => (e & ncurses_mouse_mask!(x 0x002)); )
macro_rules! button_click( ($e: expr $x: expr) => (e & ncurses_mouse_mask!(x 0x004)); )
macro_rules! button_double_click( ($e: expr $x: expr) => (e & ncurses_mouse_mask!(x 0x008)); )
macro_rules! button_triple_click( ($e: expr $x: expr) => (e & ncurses_mouse_mask!(x 0x010)); )
macro_rules! button_reserved_event( ($e: expr $x: expr) => (e & ncurses_mouse_mask!(x 0x020)); )

