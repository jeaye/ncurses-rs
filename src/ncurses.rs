/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: lib.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Safe wrappers for ncurses functions.
*/

#![crate_name = "ncurses"]
#![crate_type = "lib"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::mem;
use std::{ char, ptr };
use std::ffi::{CString, CStr};
use self::ll::{FILE_p};
pub use self::constants::*;
pub use self::panel::wrapper::*;
pub use self::menu::wrapper::*;
pub use self::menu::constants::*;

pub type chtype = self::ll::chtype;
pub type winttype = u32;

pub type mmask_t = chtype;
pub type attr_t = chtype;
pub type NCURSES_ATTR_T = attr_t;

pub mod ll;
pub mod constants;
pub mod panel;
pub mod menu;

trait FromCStr {
    unsafe fn from_c_str(s: *const libc::c_char) -> Self;
}

impl FromCStr for String {
    unsafe fn from_c_str(s: *const libc::c_char) -> String {
        let bytes = CStr::from_ptr(s).to_bytes();
        String::from_utf8_unchecked(bytes.to_vec())
    }
}

impl FromCStr for Option<String> {
    unsafe fn from_c_str(s: *const libc::c_char) -> Option<String> {
        if s.is_null() {
            None
        } else {
            Some(FromCStr::from_c_str(s))
        }
    }
}

trait ToCStr {
    fn to_c_str(&self) -> CString;
}

impl <'a>ToCStr for &'a str {
    fn to_c_str(&self) -> CString {
        CString::new(*self).unwrap()
    }
}

#[derive(Clone, Copy)]
pub enum CURSOR_VISIBILITY
{
  CURSOR_INVISIBLE = 0,
  CURSOR_VISIBLE,
  CURSOR_VERY_VISIBLE
}

pub type WINDOW = self::ll::WINDOW;
pub type SCREEN = self::ll::SCREEN;
pub type mmaskt = self::ll::mmask_t;
pub type MEVENT = self::ll::MEVENT;

pub fn addch(ch: chtype) -> i32
{ unsafe { ll::addch(ch) } }


pub fn addchnstr(s: &[chtype], n: i32) -> i32
{ unsafe { ll::addchnstr(s.as_ptr(), n) } }


pub fn addchstr(s: &[chtype]) -> i32
{ unsafe { ll::addchstr(s.as_ptr()) } }


pub fn addnstr(s: &str, n: i32) -> i32
{ unsafe { ll::addnstr(s.to_c_str().as_ptr(), n) } }


pub fn addstr(s: &str) -> i32
{ unsafe { ll::addstr(s.to_c_str().as_ptr()) } }


pub fn assume_default_colors(fg: i32, bg: i32) -> i32
{ unsafe { ll::assume_default_colors(fg, bg) } }


pub fn attroff(a: NCURSES_ATTR_T) -> i32
{ unsafe { ll::attroff(a) } }


pub fn attron(a: NCURSES_ATTR_T) -> i32
{ unsafe { ll::attron(a) } }


pub fn attrset(a: NCURSES_ATTR_T) -> i32
{ unsafe { ll::attrset(a) } }


pub fn attr_get(attrs: &mut attr_t, pair: &mut i16) -> i32
{
  unsafe
  {
    ll::attr_get(&mut* attrs as *mut attr_t,
                 &mut* pair as *mut i16,
                 ptr::null())
  }
}


pub fn attr_off(a: attr_t) -> i32
{ unsafe { ll::attr_off(a, ptr::null()) } }


pub fn attr_on(a: attr_t) -> i32
{ unsafe { ll::attr_on(a, ptr::null()) } }


pub fn attr_set(attr: attr_t, pair: i16) -> i32
{ unsafe { ll::attr_set(attr, pair, ptr::null()) } }


pub fn baudrate() -> i32
{ unsafe { ll::baudrate() } }


pub fn beep() -> i32
{ unsafe { ll::beep() } }


pub fn bkgd(ch: chtype) -> i32
{ unsafe { ll::bkgd(ch) } }


pub fn bkgdset(ch: chtype)
{ unsafe { ll::bkgdset(ch) } }


pub fn border(ls: chtype, rs: chtype, ts: chtype, bs: chtype, tl: chtype, tr: chtype, bl: chtype, br: chtype) -> i32
{ unsafe { ll::border(ls, rs, ts, bs, tl, tr, bl, br) } }


#[link_name="box"] pub fn box_(w: WINDOW, v: chtype, h: chtype) -> i32
{ wborder(w, v, v, h, h, 0, 0, 0, 0) }


pub fn can_change_color() -> bool
{ unsafe { ll::can_change_color() == TRUE } }


pub fn cbreak() -> i32
{ unsafe { ll::cbreak() } }


pub fn chgat(n: i32, attr: attr_t, color: i16) -> i32
{ unsafe { ll::chgat(n, attr, color, ptr::null()) } }


pub fn clear() -> i32
{ unsafe { ll::clear() } }


pub fn clearok(w: WINDOW, ok: bool) -> i32
{ unsafe { ll::clearok(w, ok as ll::c_bool) } }


pub fn clrtobot() -> i32
{ unsafe { ll::clrtobot() } }


pub fn clrtoeol() -> i32
{ unsafe { ll::clrtoeol() } }


pub fn color_content(color: i16, r: &mut i16, g: &mut i16, b: &mut i16) -> i32
{
  unsafe
  {
    ll::color_content(color,
                      &mut*r as *mut i16,
                      &mut*g as *mut i16,
                      &mut*b as *mut i16)
  }
}


pub fn color_set(pair: i16) -> i32
{ unsafe { ll::color_set(pair, ptr::null()) } }


pub fn copywin(src_win: WINDOW, dest_win: WINDOW, src_min_row: i32,
               src_min_col: i32, dest_min_row: i32, dest_min_col: i32,
               dest_max_row: i32, dest_max_col: i32, overlay: i32) -> i32
{
  unsafe
  {
    ll::copywin(src_win, dest_win, src_min_row, src_min_col,
                dest_min_row, dest_min_col, dest_max_row,
                dest_max_col, overlay)
  }
}


pub fn curs_set(visibility: CURSOR_VISIBILITY) -> Option<CURSOR_VISIBILITY>
{
  unsafe
  {
    match ll::curs_set(visibility as i32)
    {
      ERR => None,
      ret => Some(mem::transmute::<i8, CURSOR_VISIBILITY>(ret as i8)),
    }
  }
}


pub fn def_prog_mode() -> i32
{ unsafe { ll::def_prog_mode() } }


pub fn def_shell_mode() -> i32
{ unsafe { ll::def_shell_mode() } }


pub fn delay_output(ms: i32) -> i32
{ unsafe { ll::delay_output(ms) } }


pub fn delch() -> i32
{ unsafe { ll::delch() } }


pub fn delscreen(s: SCREEN)
{ unsafe { ll::delscreen(s) } }


pub fn delwin(w: WINDOW) -> i32
{ unsafe { ll::delwin(w) } }


pub fn deleteln() -> i32
{ unsafe { ll::deleteln() } }


pub fn derwin(w: WINDOW, lines: i32, cols: i32, x: i32, y: i32) -> WINDOW
{ unsafe { ll::derwin(w, lines, cols, x, y) } }


pub fn doupdate() -> i32
{ unsafe { ll::doupdate() } }


pub fn dupwin(w: WINDOW) -> WINDOW
{ unsafe { ll::dupwin(w) } }


pub fn echo() -> i32
{ unsafe { ll::echo() } }


pub fn echochar(c: chtype) -> i32
{ unsafe { ll::echochar(c) } }


pub fn erase() -> i32
{ unsafe { ll::erase() } }


pub fn endwin() -> i32
{ unsafe { ll::endwin() } }


pub fn erasechar() -> char
{ unsafe { char::from_u32(ll::erasechar() as u32).expect("Invalid char") } }


pub fn filter()
{ unsafe { ll::filter() } }


pub fn flash() -> i32
{ unsafe { ll::flash() } }


pub fn flushinp() -> i32
{ unsafe { ll::flushinp() } }


pub fn getbkgd(w: WINDOW) -> chtype
{ unsafe { ll::getbkgd(w) } }


pub fn getch() -> i32
{ unsafe { ll::getch() } }

pub enum WchResult {
    KeyCode(i32),
    Char(winttype),
}

pub fn get_wch() -> Option<WchResult> {
    unsafe {
        let mut x = 0;
        match ll::get_wch(&mut x) {
            OK => {
                Some(WchResult::Char(x))
            }
            KEY_CODE_YES => {
                Some(WchResult::KeyCode(x as i32))
            }
            _ => {
                None
            }
        }
    }
}

pub fn mvget_wch(y: i32, x: i32) -> Option<WchResult> {
    unsafe {
        let mut result = 0;
        match ll::mvget_wch(y, x, &mut result) {
            OK => {
                Some(WchResult::Char(result))
            }
            KEY_CODE_YES => {
                Some(WchResult::KeyCode(result as i32))
            }
            _ => {
                None
            }
        }
    }
}

pub fn wget_wch(w: WINDOW) -> Option<WchResult> {
    unsafe {
        let mut result = 0;
        match ll::wget_wch(w, &mut result) {
            OK => {
                Some(WchResult::Char(result))
            }
            KEY_CODE_YES => {
                Some(WchResult::KeyCode(result as i32))
            }
            _ => {
                None
            }
        }
    }
}

pub fn mvwget_wch(w: WINDOW, y: i32, x: i32) -> Option<WchResult> {
    unsafe {
        let mut result = 0;
        match ll::mvwget_wch(w, y, x, &mut result) {
            OK => {
                Some(WchResult::Char(result))
            }
            KEY_CODE_YES => {
                Some(WchResult::KeyCode(result as i32))
            }
            _ => {
                None
            }
        }
    }
}

pub fn unget_wch(ch: u32) -> i32 {
    unsafe {
        ll::unget_wch(ch)
    }
}


pub fn getnstr(s: &mut String, n: i32) -> i32
{ wgetnstr(stdscr(), s, n) }


pub fn getstr(s: &mut String) -> i32
{
  /* XXX: This is probably broken. */
  let mut ch = getch();
  while ch != '\n' as i32 && ch != '\r' as i32
  {
    unsafe { s.as_mut_vec().push(ch as u8); }
    ch = getch();
  }
  OK
}


pub fn getwin(reader: *mut libc::FILE) -> WINDOW
{ unsafe { ll::getwin(reader) } } /* TODO: Make this safe. */


pub fn getattrs(w: WINDOW) -> i32
{ unsafe { ll::getattrs(w) } }


pub fn getcurx(w: WINDOW) -> i32
{ unsafe { ll::getcurx(w) } }


pub fn getcury(w: WINDOW) -> i32
{ unsafe { ll::getcury(w) } }


pub fn getbegx(w: WINDOW) -> i32
{ unsafe { ll::getbegx(w) } }


pub fn getbegy(w: WINDOW) -> i32
{ unsafe { ll::getbegy(w) } }


pub fn getmaxx(w: WINDOW) -> i32
{ unsafe { ll::getmaxx(w) } }


pub fn getmaxy(w: WINDOW) -> i32
{ unsafe { ll::getmaxy(w) } }


pub fn getparx(w: WINDOW) -> i32
{ unsafe { ll::getparx(w) } }


pub fn getpary(w: WINDOW) -> i32
{ unsafe { ll::getpary(w) } }


pub fn halfdelay(tenths: i32) -> i32
{ unsafe { ll::halfdelay(tenths) } }


pub fn has_colors() -> bool
{ unsafe { ll::has_colors() == TRUE } }


pub fn has_ic() -> bool
{ unsafe { ll::has_ic() == TRUE } }


pub fn has_il() -> bool
{ unsafe { ll::has_il() == TRUE } }


pub fn hline(ch: chtype, n: i32) -> i32
{ unsafe { ll::hline(ch, n) } }


pub fn idcok(w: WINDOW, bf: bool)
{ unsafe { ll::idcok(w, bf as ll::c_bool) } }


pub fn idlok(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::idlok(w, bf as ll::c_bool) } }


pub fn immedok(w: WINDOW, bf: bool)
{ unsafe { ll::immedok(w, bf as ll::c_bool) } }


pub fn inch() -> chtype
{ unsafe { ll::inch() } }


pub fn inchnstr(s: &mut Vec<chtype>, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  s.clear();
  s.reserve(n as usize);
  unsafe
  {
    let ret = ll::inchnstr(s.as_ptr(), n);

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as usize),
      None => s.set_len(capacity),
    }

    ret
  }
}


pub fn inchstr(s: &mut Vec<chtype>) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let ret = ll::inchstr(s.as_ptr());

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as usize),
      None => s.set_len(capacity),
    }

    ret
  }
}


pub fn initscr() -> WINDOW
{ unsafe { ll::initscr() } }


pub fn init_color(color: i16, r: i16, g: i16, b: i16) -> i32
{ unsafe { ll::init_color(color, r, g, b) } }


pub fn init_pair(pair: i16, f: i16, b: i16) -> i32
{ unsafe { ll::init_pair(pair, f, b) } }


pub fn innstr(s: &mut String, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    s.as_mut_vec().clear();
    s.reserve(n as usize);
    let buf = s.as_bytes().as_ptr();
    let ret = ll::innstr(mem::transmute(buf), n);

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as usize),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn insch(ch: chtype) -> i32
{ unsafe { ll::insch(ch) } }


pub fn insdelln(n: i32) -> i32
{ unsafe { ll::insdelln(n) } }


pub fn insertln() -> i32
{ unsafe { ll::insertln() } }


pub fn insnstr(s: &str, n: i32) -> i32
{
  unsafe
  {
    let buf = s.as_ptr();
    ll::insnstr(mem::transmute(buf), n)
  }
}


pub fn insstr(s: &str) -> i32
{
  unsafe
  {
    let buf = s.as_ptr();
    ll::insstr(mem::transmute(buf))
  }
}


pub fn instr(s: &mut String) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let buf = s.as_bytes().as_ptr();
    let ret = ll::instr(mem::transmute(buf));

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as usize),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn intrflush(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::intrflush(w, bf as ll::c_bool) } }


pub fn isendwin() -> bool
{ unsafe { ll::isendwin() == TRUE } }


pub fn is_linetouched(w: WINDOW, l: i32) -> bool
{ unsafe { ll::is_linetouched(w, l) == TRUE } }


pub fn is_wintouched(w: WINDOW) -> bool
{ unsafe { ll::is_wintouched(w) == TRUE } }


pub fn is_term_resized(lines: i32, cols: i32) -> bool
{ unsafe { ll::is_term_resized(lines, cols) == TRUE } }


pub fn is_cleared(w: WINDOW) -> bool
{ unsafe { ll::is_cleared(w) == TRUE } }


pub fn is_idcok(w: WINDOW) -> bool
{ unsafe { ll::is_idcok(w) == TRUE } }


pub fn is_idlok(w: WINDOW) -> bool
{ unsafe { ll::is_idlok(w) == TRUE } }


pub fn is_immedok(w: WINDOW) -> bool
{ unsafe { ll::is_immedok(w) == TRUE } }


pub fn is_keypad(w: WINDOW) -> bool
{ unsafe { ll::is_keypad(w) == TRUE } }


pub fn is_leaveok(w: WINDOW) -> bool
{ unsafe { ll::is_leaveok(w) == TRUE } }


pub fn is_nodelay(w: WINDOW) -> bool
{ unsafe { ll::is_nodelay(w) == TRUE } }


pub fn is_notimeout(w: WINDOW) -> bool
{ unsafe { ll::is_notimeout(w) == TRUE } }


pub fn is_scrollok(w: WINDOW) -> bool
{ unsafe { ll::is_scrollok(w) == TRUE } }


pub fn is_syncok(w: WINDOW) -> bool
{ unsafe { ll::is_syncok(w) == TRUE }}


pub fn keyname(c: i32) -> Option<String>
{ unsafe { FromCStr::from_c_str(ll::keyname(c)) } }


pub fn keypad(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::keypad(w, bf as ll::c_bool) } }


pub fn killchar() -> char
{ unsafe { char::from_u32(ll::killchar() as u32).expect("Invalid char") } }


pub fn leaveok(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::leaveok(w, bf as ll::c_bool) } }


pub fn longname() -> String
{ unsafe { FromCStr::from_c_str(ll::longname()) } }


pub fn meta(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::meta(w, bf as ll::c_bool) } }


pub fn mv(y: i32, x: i32) -> i32
{ unsafe { ll::mv(y, x) } }


pub fn mvaddch(y: i32, x: i32, c: chtype) -> i32
{ unsafe { ll::mvaddch(y, x, c) } }


pub fn mvaddchnstr(y: i32, x: i32, s: &[chtype], n: i32) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  addchnstr(s, n)
}


pub fn mvaddchstr(y: i32, x: i32, s: &[chtype]) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  addchstr(s)
}


pub fn mvaddnstr(y: i32, x: i32, s: &str, n: i32) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  addnstr(s, n)
}


pub fn mvaddstr(y: i32, x: i32, s: &str) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  addstr(s)
}


pub fn mvchgat(y: i32, x: i32, n: i32, attr: attr_t, color: i16) -> i32
{ unsafe { ll::mvchgat(y, x, n, attr, color, ptr::null()) } }


pub fn mvcur(old_y: i32, old_x: i32, new_y: i32, new_x: i32) -> i32
{ unsafe { ll::mvcur(old_y, old_x, new_y, new_x) } }


pub fn mvdelch(y: i32, x: i32) -> i32
{ unsafe { ll::mvdelch(y, x) } }


pub fn mvderwin(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { ll::mvderwin(w, y, x) } }


pub fn mvgetch(y: i32, x: i32) -> i32
{ unsafe { ll::mvgetch(y, x) } }


pub fn mvgetnstr(y: i32, x: i32, s: &mut String, n: i32) -> i32
{
  match mv(y, x)
  {
    OK => getnstr(s, n),
    _ => ERR,
  }
}


pub fn mvgetstr(y: i32, x: i32, s: &mut String) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  getstr(s)
}


pub fn mvhline(y: i32, x: i32, ch: chtype, n: i32) -> i32
{ unsafe { ll::mvhline(y, x, ch, n) } }


pub fn mvinch(y: i32, x: i32) -> chtype
{ unsafe { ll::mvinch(y, x) } }


pub fn mvinchnstr(y: i32, x: i32, s: &mut Vec<chtype>, n: i32) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  inchnstr(s, n)
}


pub fn mvinchstr(y: i32, x: i32, s: &mut Vec<chtype>) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  inchstr(s)
}


pub fn mvinnstr(y: i32, x: i32, s: &mut String, n: i32) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  innstr(s, n)
}


pub fn mvinsch(y: i32, x: i32, ch: chtype) -> i32
{ unsafe { ll::mvinsch(y, x, ch) } }


pub fn mvinsnstr(y: i32, x: i32, s: &str, n: i32) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  insnstr(s, n)
}


pub fn mvinsstr(y: i32, x: i32, s: &str) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  insstr(s)
}


pub fn mvinstr(y: i32, x: i32, s: &mut String) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  instr(s)
}


pub fn mvprintw(y: i32, x: i32, s: &str) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }
  printw(s)
}


pub fn mvvline(y: i32, x: i32, ch: chtype, n: i32) -> i32
{ unsafe { ll::mvvline(y, x, ch, n) } }


pub fn mvwaddch(w: WINDOW, y: i32, x: i32, ch: chtype) -> i32
{ unsafe { ll::mvwaddch(w, y, x, ch) } }


pub fn mvwaddchnstr(w: WINDOW, y: i32, x: i32, s: &[chtype], n: i32) -> i32
{ unsafe { ll::mvwaddchnstr(w, y, x, s.as_ptr(), n) } }


pub fn mvwaddchstr(w: WINDOW, y: i32, x: i32, s: &[chtype]) -> i32
{ unsafe { ll::mvwaddchstr(w, y, x, s.as_ptr()) } }


pub fn mvwaddnstr(w: WINDOW, y: i32, x: i32, s: &str, n: i32) -> i32
{ unsafe { ll::mvwaddnstr(w, y, x, s.to_c_str().as_ptr(), n) } }


pub fn mvwaddstr(w: WINDOW, y: i32, x: i32, s: &str) -> i32
{ unsafe { ll::mvwaddstr(w, y, x, s.to_c_str().as_ptr()) } }


pub fn mvwchgat(w: WINDOW, y: i32, x: i32, n: i32, attr: attr_t, color: i16) -> i32
{ unsafe { ll::mvwchgat(w, y, x, n, attr, color, ptr::null()) } }


pub fn mvwdelch(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { ll::mvwdelch(w, y, x) } }


pub fn mvwgetch(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { ll::mvwgetch(w, y, x) } }


pub fn mvwgetnstr(w: WINDOW, y: i32, x: i32, s: &mut String, n: i32) -> i32
{
  match wmove(w, y, x)
  { 
    OK => wgetnstr(w, s, n), 
    _ => ERR,
  }
}


pub fn mvwgetstr(w: WINDOW, y: i32, x: i32, s: &mut String) -> i32
{
  if mv(y, x) == ERR
  { return ERR; }

  /* XXX: This is probably broken. */
  let mut ch = wgetch(w);
  while ch != '\n' as i32 && ch != '\r' as i32
  {
    unsafe { s.as_mut_vec().push(ch as u8); }
    ch = wgetch(w);
  }
  OK
}


pub fn mvwhline(w: WINDOW, y: i32, x: i32, ch: chtype, n: i32) -> i32
{ unsafe { ll::mvwhline(w, y, x, ch, n) } }


pub fn mvwin(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { ll::mvwin(w, y, x) } }


pub fn mvwinch(w: WINDOW, y: i32, x: i32) -> chtype
{ unsafe { ll::mvwinch(w, y, x) } }


pub fn mvwinchnstr(w: WINDOW, y: i32, x: i32, s: &mut Vec<chtype>, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  s.clear();
  s.reserve(n as usize);
  unsafe
  {
    let ret = ll::mvwinchnstr(w, y, x, s.as_ptr(), n);

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as usize),
      None => s.set_len(capacity),
    }

    ret
  }
}


pub fn mvwinchstr(w: WINDOW, y: i32, x: i32, s: &mut Vec<chtype>) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let ret = ll::mvwinchstr(w, y, x, s.as_ptr());

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as usize),
      None => s.set_len(capacity),
    }

    ret
  }
}


pub fn mvwinnstr(w: WINDOW, y: i32, x: i32, s: &mut String, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    s.as_mut_vec().clear();
    s.reserve(n as usize);
    let buf = s.as_bytes().as_ptr();
    let ret = ll::mvwinnstr(w, y, x, mem::transmute(buf), n);

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as usize),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn mvwinsch(w: WINDOW, y: i32, x: i32, ch: chtype) -> i32
{ unsafe { ll::mvwinsch(w, y, x, ch) } }


pub fn mvwinsnstr(w: WINDOW, y: i32, x: i32, s: &str, n: i32) -> i32
{ unsafe { ll::mvwinsnstr(w, y, x, s.to_c_str().as_ptr(), n) } }


pub fn mvwinsstr(w: WINDOW, y: i32, x: i32, s: &str) -> i32
{ unsafe { ll::mvwinsstr(w, y, x, s.to_c_str().as_ptr()) } }


pub fn mvwinstr(w: WINDOW, y: i32, x: i32, s: &mut String) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let buf = s.as_bytes().as_ptr();
    let ret = ll::mvwinstr(w, y, x, mem::transmute(buf));

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as usize),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn mvwprintw(w: WINDOW, y: i32, x: i32, s: &str) -> i32
{ unsafe { ll::mvwprintw(w, y, x, s.to_c_str().as_ptr()) } }


pub fn mvwvline(w: WINDOW, y: i32, x: i32, ch: chtype, n: i32) -> i32
{ unsafe { ll::mvwvline(w, y, x, ch, n) } }


pub fn napms(ms: i32) -> i32
{ unsafe { ll::napms(ms) } }


pub fn newpad(lines: i32, cols: i32) -> WINDOW
{ unsafe { ll::newpad(lines, cols) } }


pub fn newterm(ty: Option<&str>, out_fd: FILE_p, in_fd: FILE_p) -> SCREEN
{
  unsafe
  {
    match ty {
      Some(s) => ll::newterm(s.to_c_str().as_ptr(), out_fd, in_fd),
      None    => ll::newterm(std::ptr::null(), out_fd, in_fd),
    }
  }
}


pub fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> WINDOW
{ unsafe { ll::newwin(lines, cols, y, x) } }


pub fn nl() -> i32
{ unsafe { ll::nl() } }


pub fn nocbreak() -> i32
{ unsafe { ll::nocbreak() } }


pub fn nodelay(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::nodelay(w, bf as ll::c_bool) } }


pub fn noecho() -> i32
{ unsafe { ll::noecho() } }


pub fn nonl() -> i32
{ unsafe { ll::nonl() } }


pub fn noqiflush()
{ unsafe { ll::noqiflush() } }


pub fn noraw() -> i32
{ unsafe { ll::noraw() } }


pub fn notimeout(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::notimeout(w, bf as ll::c_bool) } }


pub fn overlay(src: WINDOW, dst: WINDOW) -> i32
{ unsafe { ll::overlay(src, dst) } }


pub fn overwrite(src: WINDOW, dst: WINDOW) -> i32
{ unsafe { ll::overwrite(src, dst) } }


pub fn pair_content(pair: i16, f: &mut i16, b: &mut i16) -> i32
{ unsafe { ll::pair_content(pair, &mut*f as *mut i16, &mut*b as *mut i16) } }


pub fn PAIR_NUMBER(attr: i32) -> i32
{ unsafe { ll::PAIR_NUMBER(attr) } }


pub fn pechochar(pad: WINDOW, ch: chtype) -> i32
{ unsafe { ll::pechochar(pad, ch) } }


pub fn pnoutrefresh(pad: WINDOW, pmin_row: i32, pmin_col: i32, smin_row: i32, smin_col: i32, smax_row: i32, smax_col: i32) -> i32
{ unsafe { ll::pnoutrefresh(pad, pmin_row, pmin_col, smin_row, smin_col, smax_row, smax_col) } }


pub fn prefresh(pad: WINDOW, pmin_row: i32, pmin_col: i32, smin_row: i32, smin_col: i32, smax_row: i32, smax_col: i32) -> i32
{ unsafe { ll::prefresh(pad, pmin_row, pmin_col, smin_row, smin_col, smax_row, smax_col) } }


pub fn printw(s: &str) -> i32
{ unsafe { ll::printw(s.to_c_str().as_ptr()) } }


pub fn putp(s: &str) -> i32
{ unsafe { ll::putp(s.to_c_str().as_ptr()) } }


pub fn putwin(w: WINDOW, f: FILE_p) -> i32
{ unsafe { ll::putwin(w, f) } }


pub fn qiflush()
{ unsafe { ll::qiflush() } }


pub fn raw() -> i32
{ unsafe { ll::raw() } }


pub fn redrawwin(w: WINDOW) -> i32
{ unsafe { ll::redrawwin(w) } }


pub fn refresh() -> i32
{ unsafe { ll::refresh() } }


pub fn resetty() -> i32
{ unsafe { ll::resetty() } }


pub fn reset_prog_mode() -> i32
{ unsafe { ll::reset_prog_mode() } }


pub fn reset_shell_mode() -> i32
{ unsafe { ll::reset_shell_mode() } }


pub fn resizeterm(lines: i32, cols: i32) -> i32
{ unsafe { ll::resizeterm(lines, cols) } }


pub fn resize_term(lines: i32, cols: i32) -> i32
{ unsafe { ll::resize_term(lines, cols) } }


pub fn savetty() -> i32
{ unsafe { ll::savetty() } }


pub fn scr_dump(filename: &str) -> i32
{ unsafe { ll::scr_dump(filename.to_c_str().as_ptr()) } }


pub fn scr_init(filename: &str) -> i32
{ unsafe { ll::scr_init(filename.to_c_str().as_ptr()) } }


pub fn scrl(n: i32) -> i32
{ unsafe { ll::scrl(n) } }


pub fn scroll(w: WINDOW) -> i32
{ unsafe { ll::scroll(w) } }


pub fn scrollok(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::scrollok(w, bf as ll::c_bool) } }


pub fn scr_restore(filename: &str) -> i32
{ unsafe { ll::scr_restore(filename.to_c_str().as_ptr()) } }


pub fn scr_set(filename: &str) -> i32
{ unsafe { ll::scr_set(filename.to_c_str().as_ptr()) } }

pub fn setlocale(lc: LcCategory, locale: &str) -> String
{
  unsafe {
    let buf = locale.to_c_str().as_ptr();
    let ret = ll::setlocale(lc as libc::c_int, buf);
    if ret == ptr::null() {
        String::new()
    } else {
        // The clone is necessary, as the returned pointer
        // can change at any time
        CStr::from_ptr(ret).to_string_lossy().into_owned()
    }
  }
}

pub fn setscrreg(top: i32, bot: i32) -> i32
{ unsafe { ll::setscrreg(top, bot) } }


pub fn set_term(s: SCREEN) -> SCREEN
{ unsafe { ll::set_term(s) } }


pub fn slk_attroff(ch: chtype) -> i32
{ unsafe { ll::slk_attroff(ch) } }

//
//pub fn slk_attr_off(ch: attr_t) -> i32
//{ unsafe { ll::slk_attr_off(ch, ptr::null()) } }


pub fn slk_attron(ch: chtype) -> i32
{ unsafe { ll::slk_attron(ch) } }

//
//pub fn slk_attr_on(ch: attr_t) -> i32
//{ unsafe { ll::slk_attr_on(ch, ptr::null()) } }


pub fn slk_attrset(ch: chtype) -> i32
{ unsafe { ll::slk_attrset(ch) } }


pub fn slk_attr() -> attr_t
{ unsafe { ll::slk_attr() } }


pub fn slk_attr_set(attrs: attr_t, pair: i16) -> i32
{ unsafe { ll::slk_attr_set(attrs, pair, ptr::null()) } }


pub fn slk_clear() -> i32
{ unsafe { ll::slk_clear() } }


pub fn slk_color(pair: i16) -> i32
{ unsafe { ll::slk_color(pair) } }


pub fn slk_init(fmt: i32) -> i32
{ unsafe { ll::slk_init(fmt) } }


pub fn slk_label(n: i32) -> String
{ unsafe { FromCStr::from_c_str(ll::slk_label(n)) } }


pub fn slk_noutrefresh() -> i32
{ unsafe { ll::slk_noutrefresh() } }


pub fn slk_refresh() -> i32
{ unsafe { ll::slk_refresh() } }


pub fn slk_restore() -> i32
{ unsafe { ll::slk_restore() } }


pub fn slk_set(n: i32, s: &str, fmt: i32) -> i32
{ unsafe { ll::slk_set(n, s.to_c_str().as_ptr(), fmt) } }


pub fn slk_touch() -> i32
{ unsafe { ll::slk_touch() }}


pub fn standout() -> i32
{ unsafe { ll::standout() } }


pub fn standend() -> i32
{ unsafe { ll::standend() } }


pub fn start_color() -> i32
{ unsafe { ll::start_color() } }


pub fn subpad(w: WINDOW, lines: i32, cols: i32, y: i32, x: i32) -> WINDOW
{ unsafe { ll::subpad(w, lines, cols, y, x) } }


pub fn subwin(w: WINDOW, lines: i32, cols: i32, y: i32, x: i32) -> WINDOW
{ unsafe { ll::subwin(w, lines, cols, y, x) } }


pub fn syncok(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::syncok(w, bf as ll::c_bool) } }


pub fn termattrs() -> chtype
{ unsafe { ll::termattrs() } }


pub fn termname() -> String
{ unsafe { FromCStr::from_c_str(ll::termname()) } }


pub fn timeout(delay: i32)
{ unsafe { ll::timeout(delay) } }


pub fn touchline(w: WINDOW, start: i32, count: i32) -> i32
{ unsafe { ll::touchline(w, start, count) } }


pub fn touchwin(w: WINDOW) -> i32
{ unsafe { ll::touchwin(w) } }


pub fn typeahead(fd: i32) -> i32
{ unsafe { ll::typeahead(fd) } }


pub fn tigetflag(capname: &str) -> i32
{ unsafe { ll::tigetflag(capname.to_c_str().as_ptr()) } }


pub fn tigetnum(capname: &str) -> i32
{ unsafe { ll::tigetnum(capname.to_c_str().as_ptr()) } }


pub fn tigetstr(capname: &str) -> String
{ unsafe { { FromCStr::from_c_str(ll::tigetstr(capname.to_c_str().as_ptr())) } } }


pub fn tparm(s: &str) -> String
{ unsafe { { FromCStr::from_c_str(ll::tparm(s.to_c_str().as_ptr())) } } }


pub fn ungetch(ch: i32) -> i32
{ unsafe { ll::ungetch(ch) } }


pub fn untouchwin(w: WINDOW) -> i32
{ unsafe { ll::untouchwin(w) } }


pub fn use_env(f: bool)
{ unsafe { ll::use_env(f as ll::c_bool) } }


pub fn use_default_colors() -> i32
{ unsafe { ll::use_default_colors() } }


pub fn vidattr(attrs: chtype) -> i32
{ unsafe { ll::vidattr(attrs) } }


pub fn vline(ch: chtype, n: i32) -> i32
{ unsafe { ll::vline(ch, n) } }


pub fn waddch(w: WINDOW, ch: chtype) -> i32
{ unsafe { ll::waddch(w, ch) } }


pub fn waddchnstr(w: WINDOW, s: &[chtype], n: i32) -> i32
{ unsafe { ll::waddchnstr(w, s.as_ptr(), n) } }


pub fn waddchstr(w: WINDOW, s: &[chtype]) -> i32
{ unsafe { ll::waddchstr(w, s.as_ptr()) } }


pub fn waddnstr(w: WINDOW, s: &str, n: i32) -> i32
{ unsafe { ll::waddnstr(w, s.to_c_str().as_ptr(), n) } }


pub fn waddstr(w: WINDOW, s: &str) -> i32
{ unsafe { ll::waddstr(w, s.to_c_str().as_ptr()) } }


pub fn wattron(w: WINDOW, attr: NCURSES_ATTR_T) -> i32
{ unsafe { ll::wattron(w, attr) } }


pub fn wattroff(w: WINDOW, attr: NCURSES_ATTR_T) -> i32
{ unsafe { ll::wattroff(w, attr) } }


pub fn wattrset(w: WINDOW, attr: NCURSES_ATTR_T) -> i32
{ unsafe { ll::wattrset(w, attr) } }


pub fn wattr_get(w: WINDOW, attrs: &mut attr_t, pair: &mut i16) -> i32
{ unsafe { ll::wattr_get(w, &mut*attrs as *mut attr_t, &mut*pair as *mut i16, ptr::null()) } }


pub fn wattr_on(w: WINDOW, attr: attr_t) -> i32
{ unsafe { ll::wattr_on(w, attr, ptr::null()) } }


pub fn wattr_off(w: WINDOW, attr: attr_t) -> i32
{ unsafe { ll::wattr_off(w, attr, ptr::null()) } }


pub fn wattr_set(w: WINDOW, attrs: attr_t, pair: i16) -> i32
{ unsafe { ll::wattr_set(w, attrs, pair, ptr::null()) } }


pub fn wbkgd(w: WINDOW, ch: chtype) -> i32
{ unsafe { ll::wbkgd(w, ch) } }


pub fn wbkgdset(w: WINDOW, ch: chtype)
{ unsafe { ll::wbkgdset(w, ch) } }


pub fn wborder(w: WINDOW, ls: chtype, rs: chtype, ts: chtype, bs: chtype, tl: chtype, tr: chtype, bl: chtype, br: chtype) -> i32
{ unsafe { ll::wborder(w, ls, rs, ts, bs, tl, tr, bl, br) } }


pub fn wchgat(w: WINDOW, n: i32, attr: attr_t, color: i16) -> i32
{ unsafe { ll::wchgat(w, n, attr, color, ptr::null()) } }


pub fn wclear(w: WINDOW) -> i32
{ unsafe { ll::wclear(w) } }


pub fn wclrtobot(w: WINDOW) -> i32
{ unsafe { ll::wclrtobot(w) } }


pub fn wclrtoeol(w: WINDOW) -> i32
{ unsafe { ll::wclrtoeol(w) } }


pub fn wcolor_set(w: WINDOW, pair: i16) -> i32
{ unsafe { ll::wcolor_set(w, pair, ptr::null()) } }


pub fn wcursyncup(w: WINDOW)
{ unsafe { ll::wcursyncup(w) } }


pub fn wdelch(w: WINDOW) -> i32
{ unsafe { ll::wdelch(w) } }


pub fn wdeleteln(w: WINDOW) -> i32
{ unsafe { ll::wdeleteln(w) } }


pub fn wechochar(w: WINDOW, ch: chtype) -> i32
{ unsafe { ll::wechochar(w, ch) } }


pub fn werase(w: WINDOW) -> i32
{ unsafe { ll::werase(w) } }


pub fn wgetch(w: WINDOW) -> i32
{ unsafe { ll::wgetch(w) } }


pub fn wgetnstr(w: WINDOW, s: &mut String, n: i32) -> i32
{
  let mut buff: Vec<u8> = Vec::with_capacity(n as usize);
  unsafe { buff.set_len(n as usize); }
  
  match unsafe { ll::wgetnstr(w, buff.as_ptr(), n) }
  {
    OK => {
      *s = buff.iter()
        .take_while(|ch| **ch != '\0' as u8 )
        .map(|ch| *ch as char )
        .collect();

      OK
    },

    _ => ERR,
  }
}


pub fn wgetstr(w: WINDOW, s: &mut String) -> i32
{
  /* XXX: This is probably broken. */
  let mut ch = wgetch(w);
  while ch != '\n' as i32 && ch != '\r' as i32
  {
    unsafe { s.as_mut_vec().push(ch as u8); }
    ch = wgetch(w);
  }
  OK
}


pub fn whline(w: WINDOW, ch: chtype, n: i32) -> i32
{ unsafe { ll::whline(w, ch, n) } }


pub fn winch(w: WINDOW) -> chtype
{ unsafe { ll::winch(w) } }


pub fn winchnstr(w: WINDOW, s: &mut Vec<chtype>, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  s.clear();
  s.reserve(n as usize);
  unsafe
  {
    let ret = ll::winchnstr(w, s.as_ptr(), n);

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as usize),
      None => s.set_len(capacity),
    }

    ret
  }
}


pub fn winchstr(w: WINDOW, s: &mut Vec<chtype>) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let ret = ll::winchstr(w, s.as_ptr());

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as usize),
      None => s.set_len(capacity),
    }

    ret
  }
}


pub fn winnstr(w: WINDOW, s: &mut String, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    s.as_mut_vec().clear();
    s.reserve(n as usize);
    let buf = s.as_bytes().as_ptr();
    let ret = ll::winnstr(w, mem::transmute(buf), n);

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as usize),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn winsch(w: WINDOW, ch: chtype) -> i32
{ unsafe { ll::winsch(w, ch) } }


pub fn winsdelln(w: WINDOW, n: i32) -> i32
{ unsafe { ll::winsdelln(w, n) } }


pub fn winsertln(w: WINDOW) -> i32
{ unsafe { ll::winsertln(w) } }


pub fn winsnstr(w: WINDOW, s: &str, n: i32) -> i32
{
  unsafe
  {
    let buf = s.as_ptr();
    ll::winsnstr(w, mem::transmute(buf), n)
  }
}


pub fn winsstr(w: WINDOW, s: &str) -> i32
{
  unsafe
  {
    let buf = s.as_ptr();
    ll::winsstr(w, mem::transmute(buf))
  }
}


pub fn winstr(w: WINDOW, s: &mut String) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let buf = s.as_bytes().as_ptr();
    let ret = ll::winstr(w, mem::transmute(buf));

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as usize),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn wmove(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { ll::wmove(w, y, x) } }


pub fn wnoutrefresh(w: WINDOW) -> i32
{ unsafe { ll::wnoutrefresh(w) } }


pub fn wprintw(w: WINDOW, s: &str) -> i32
{ unsafe { ll::wprintw(w, s.to_c_str().as_ptr()) } }


pub fn wredrawln(w: WINDOW, start: i32, n: i32) -> i32
{ unsafe { ll::wredrawln(w, start, n) } }


pub fn wrefresh(w: WINDOW) -> i32
{ unsafe { ll::wrefresh(w) } }


pub fn wresize(w: WINDOW, lines: i32, cols: i32) -> i32
{ unsafe { ll::wresize(w, lines, cols) } }


pub fn wscrl(w: WINDOW, n: i32) -> i32
{ unsafe { ll::wscrl(w, n) } }


pub fn wsetscrreg(w: WINDOW, top: i32, bot: i32) -> i32
{ unsafe { ll::wsetscrreg(w, top, bot) } }


pub fn wstandout(w: WINDOW) -> i32
{ unsafe { ll::wstandout(w) } }


pub fn wstandend(w: WINDOW) -> i32
{ unsafe { ll::wstandend(w) } }


pub fn wsyncdown(w: WINDOW)
{ unsafe { ll::wsyncdown(w) } }


pub fn wsyncup(w: WINDOW)
{ unsafe { ll::wsyncup(w) } }


pub fn wtimeout(w: WINDOW, delay: i32)
{ unsafe { ll::wtimeout(w, delay) } }


pub fn wtouchln(w: WINDOW, y: i32, n: i32, changed: i32) -> i32
{ unsafe { ll::wtouchln(w, y, n, changed) } }


pub fn wvline(w: WINDOW, ch: chtype, n: i32) -> i32
{ unsafe { ll::wvline(w, ch, n) } }


pub fn wgetparent(w: WINDOW) -> WINDOW
{ unsafe { ll::wgetparent(w) } }


pub fn wgetscrreg(w: WINDOW, top: &mut i32, bot: &mut i32) -> i32
{ unsafe { ll::wgetscrreg(w, &mut*top as *mut i32, &mut*bot as *mut i32) } }

/* Attributes */
pub fn NCURSES_BITS(mask: u32, shift: u32) -> u32
{ mask << (shift + NCURSES_ATTR_SHIFT) as usize }

pub fn A_NORMAL() -> attr_t
{ (1u32 - 1u32) as attr_t }

pub fn A_ATTRIBUTES() -> attr_t
{ NCURSES_BITS(!(1u32 - 1u32), 0u32) as attr_t }

pub fn A_CHARTEXT() -> attr_t
{(NCURSES_BITS(1u32, 0u32) - 1u32) as attr_t }

pub fn A_COLOR() -> attr_t
{ NCURSES_BITS(((1u32) << 8) - 1u32, 0u32) as attr_t }

pub fn A_STANDOUT() -> attr_t
{ NCURSES_BITS(1u32, 8u32) as attr_t }

pub fn A_UNDERLINE() -> attr_t
{ NCURSES_BITS(1u32, 9u32) as attr_t }

pub fn A_REVERSE() -> attr_t
{ NCURSES_BITS(1u32, 10u32) as attr_t }

pub fn A_BLINK() -> attr_t
{ NCURSES_BITS(1u32, 11u32) as attr_t }

pub fn A_DIM() -> attr_t
{ NCURSES_BITS(1u32, 12u32) as attr_t }

pub fn A_BOLD() -> attr_t
{ NCURSES_BITS(1u32, 13u32) as attr_t }

pub fn A_ALTCHARSET() -> attr_t
{ NCURSES_BITS(1u32, 14u32) as attr_t }

pub fn A_INVIS() -> attr_t
{ NCURSES_BITS(1u32, 15u32) as attr_t }

pub fn A_PROTECT() -> attr_t
{ NCURSES_BITS(1u32, 16u32) as attr_t }

pub fn A_HORIZONTAL() -> attr_t
{ NCURSES_BITS(1u32, 17u32) as attr_t }

pub fn A_LEFT() -> attr_t
{ NCURSES_BITS(1u32, 18u32) as attr_t }

pub fn A_LOW() -> attr_t
{ NCURSES_BITS(1u32, 19u32) as attr_t }

pub fn A_RIGHT() -> attr_t
{ NCURSES_BITS(1u32, 20u32) as attr_t }

pub fn A_TOP() -> attr_t
{ NCURSES_BITS(1u32, 21u32) as attr_t }

pub fn A_VERTICAL() -> attr_t
{ NCURSES_BITS(1u32, 22u32) as attr_t }

/* Colors. */
pub fn COLOR_PAIR(n: i16) -> attr_t
{ NCURSES_BITS(n as u32, 0u32) as attr_t }

/*
 * Most of the pseudo functions are macros that either provide compatibility
 * with older versions of curses, or provide inline functionality to improve
 * performance.
 */

pub fn getyx(win: WINDOW, y: &mut i32, x: &mut i32)
{ unsafe { *y = ll::getcury(win); *x = ll::getcurx(win); } }


pub fn getbegyx(win: WINDOW, y: &mut i32, x: &mut i32)
{ unsafe { *y = ll::getbegy(win); *x = ll::getbegx(win) } }


pub fn getmaxyx(win: WINDOW, y: &mut i32, x: &mut i32)
{ unsafe { *y = ll::getmaxy(win); *x = ll::getmaxx(win) } }


pub fn getparyx(win: WINDOW, y: &mut i32, x: &mut i32)
{ unsafe { *y = ll::getpary(win); *x = ll::getparx(win) } }


pub fn getsyx(y: &mut i32, x: &mut i32)
{
  unsafe
  {
    if newscr() != ptr::null_mut()
    {
      if ll::is_leaveok(newscr()) == TRUE
      {
        *x = -1 as i32;
        *y = -1 as i32;
      }
      else
      { getyx(newscr(), (y), (x)); }
    }
  }
}


pub fn setsyx(y: &mut i32, x: &mut i32)
{
  unsafe
  {
    if newscr() !=(0 as WINDOW)
    {
      if *y == -1 && *x == -1
      {
        ll::leaveok(newscr(), 1);
      }
      else
      {
        ll::leaveok(newscr(), 0);
        ll::wmove(newscr(), *y, *x);
      }
    }
  }
}

/* Line graphics */
pub fn NCURSES_ACS(c: char) -> chtype {
    unsafe { *acs_map().offset((c as libc::c_uchar) as isize) as chtype }
}

/* VT100 symbols begin here */
pub fn ACS_ULCORNER() -> chtype
{ NCURSES_ACS('l') } /* upper left corner */

pub fn ACS_LLCORNER() -> chtype
{ NCURSES_ACS('m') } /* lower left corner */

pub fn ACS_URCORNER() -> chtype
{ NCURSES_ACS('k') } /* upper right corner */

pub fn ACS_LRCORNER() -> chtype
{ NCURSES_ACS('j') } /* lower right corner */

pub fn ACS_LTEE() -> chtype
{ NCURSES_ACS('t') } /* tee pointing right */

pub fn ACS_RTEE() -> chtype
{ NCURSES_ACS('u') } /* tee pointing left */

pub fn ACS_BTEE() -> chtype
{ NCURSES_ACS('v') } /* tee pointing up */

pub fn ACS_TTEE() -> chtype
{ NCURSES_ACS('w') } /* tee pointing down */

pub fn ACS_HLINE() -> chtype
{ NCURSES_ACS('q') } /* horizontal line */

pub fn ACS_VLINE() -> chtype
{ NCURSES_ACS('x') } /* vertical line */

pub fn ACS_PLUS() -> chtype
{ NCURSES_ACS('n') } /* large plus or crossover */

pub fn ACS_S1() -> chtype
{ NCURSES_ACS('o') } /* scan line 1 */

pub fn ACS_S9() -> chtype
{ NCURSES_ACS('s') } /* scan line 9 */

pub fn ACS_DIAMOND() -> chtype
{ NCURSES_ACS('`') } /* diamond */

pub fn ACS_CKBOARD() -> chtype
{ NCURSES_ACS('a') } /* checker board(stipple) */

pub fn ACS_DEGREE() -> chtype
{ NCURSES_ACS('f') } /* degree symbol */

pub fn ACS_PLMINUS() -> chtype
{ NCURSES_ACS('g') } /* plus/minus */

pub fn ACS_BULLET() -> chtype
{ NCURSES_ACS('~') } /* bullet */

/* Teletype 5410v1 symbols begin here */
pub fn ACS_LARROW() -> chtype
{ NCURSES_ACS(',') } /* arrow pointing left */

pub fn ACS_RARROW() -> chtype
{ NCURSES_ACS('+') } /* arrow pointing right */

pub fn ACS_DARROW() -> chtype
{ NCURSES_ACS('.') } /* arrow pointing down */

pub fn ACS_UARROW() -> chtype
{ NCURSES_ACS('-') } /* arrow pointing up */

pub fn ACS_BOARD() -> chtype
{ NCURSES_ACS('h') } /* board of squares */

pub fn ACS_LANTERN() -> chtype
{ NCURSES_ACS('i') } /* lantern symbol */

pub fn ACS_BLOCK() -> chtype
{ NCURSES_ACS('0') } /* solid square block */

/*
 * These aren't documented, but a lot of System Vs have them anyway
 * (you can spot pprryyzz{{||}} in a lot of AT&T terminfo strings).
 * The ACS_names may not match AT&T's, our source didn't know them.
 */
pub fn ACS_S3() -> chtype
{ NCURSES_ACS('p') } /* scan line 3 */

pub fn ACS_S7() -> chtype
{ NCURSES_ACS('r') } /* scan line 7 */

pub fn ACS_LEQUAL() -> chtype
{ NCURSES_ACS('y') } /* less/equal */

pub fn ACS_GEQUAL() -> chtype
{ NCURSES_ACS('z') } /* greater/equal */

pub fn ACS_PI() -> chtype
{ NCURSES_ACS('{') } /* Pi */

pub fn ACS_NEQUAL() -> chtype
{ NCURSES_ACS('|') } /* not equal */

pub fn ACS_STERLING() -> chtype
{ NCURSES_ACS('}') } /* UK pound sign */

/*
 * Line drawing ACS names are of the form ACS_trbl, where t is the top, r
 * is the right, b is the bottom, and l is the left. t, r, b, and l might
 * be B(blank), S(single), D(double), or T(thick). The subset defined
 * here only uses B and S.
 */
pub fn ACS_BSSB() -> chtype
{ ACS_ULCORNER() }

pub fn ACS_SSBB() -> chtype
{ ACS_LLCORNER() }

pub fn ACS_BBSS() -> chtype
{ ACS_URCORNER() }

pub fn ACS_SBBS() -> chtype
{ ACS_LRCORNER() }

pub fn ACS_SBSS() -> chtype
{ ACS_RTEE() }

pub fn ACS_SSSB() -> chtype
{ ACS_LTEE() }

pub fn ACS_SSBS() -> chtype
{ ACS_BTEE() }

pub fn ACS_BSSS() -> chtype
{ ACS_TTEE() }

pub fn ACS_BSBS() -> chtype
{ ACS_HLINE() }

pub fn ACS_SBSB() -> chtype
{ ACS_VLINE() }

pub fn ACS_SSSS() -> chtype
{ ACS_PLUS() }

pub fn KEY_F(n: u8) -> i32
{
  assert!(n < 16);
  KEY_F0 + n as i32
}

/*
 * Added mouse support
 */

pub fn has_mouse() -> bool
{ unsafe { ll::has_mouse() == TRUE } }

pub fn getmouse(event: *mut MEVENT) -> i32
{ unsafe { ll::getmouse(event) } }

pub fn ungetmouse(event: *mut MEVENT) -> i32
{ unsafe { ll::ungetmouse(event) } }

pub fn mouseinterval(n: i32) -> i32
{ unsafe { ll::mouseinterval(n) } }

pub fn mousemask(newmask: mmask_t, oldmask: Option<&mut mmask_t>) -> mmask_t
{
    if oldmask.is_none() { unsafe { ll::mousemask(newmask, ptr::null_mut()) } }
    else { unsafe { ll::mousemask(newmask, oldmask.unwrap()) } }
}

pub fn wenclose(w: WINDOW, y: i32, x: i32) -> bool
{ unsafe { ll::wenclose(w, y as libc::c_int, x as libc::c_int) == TRUE } }

pub fn wmouse_trafo(w: *mut WINDOW, y: &mut[i32], x: &mut[i32], to_screen: bool) -> bool
{ unsafe { ll::wmouse_trafo(w, y.as_mut_ptr(), x.as_mut_ptr(), to_screen as ll::c_bool) == TRUE } }

pub fn mouse_trafo(y: &mut[i32], x: &mut[i32], to_screen: bool) -> bool
{ unsafe { ll::mouse_trafo(y.as_mut_ptr(), x.as_mut_ptr(), to_screen as ll::c_bool) == TRUE } }
