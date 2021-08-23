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

include!(concat!(env!("OUT_DIR"), "/raw_constants.rs"));

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
