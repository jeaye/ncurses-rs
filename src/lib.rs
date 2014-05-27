/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: lib.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Safe wrappers for ncurses functions.
*/

#![crate_id = "ncurses#5.71"]
#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![allow(non_camel_case_types)]

extern crate core;
extern crate libc;

use core::mem;
use std::{ str, char, ptr };
use self::ll::*;
pub use self::constants::*;

pub mod ll;
pub mod constants;

pub enum CURSOR_VISIBILITY
{
  CURSOR_INVISIBLE = 0,
  CURSOR_VISIBLE,
  CURSOR_VERY_vISIBLE,
}

pub type WINDOW = self::ll::WINDOW;
pub type SCREEN = self::ll::SCREEN;
pub type mmaskt = self::ll::mmask_t;
pub type MEVENT = self::ll::MEVENT;

pub fn addch(ch: u32) -> i32
{ unsafe { ll::addch(ch) } }


pub fn addchnstr(s: &[u32], n: i32) -> i32
{ unsafe { ll::addchnstr(s.as_ptr(), n) } }


pub fn addchstr(s: &[u32]) -> i32
{ unsafe { ll::addchstr(s.as_ptr()) } }


pub fn addnstr(s: &str, n: i32) -> i32
{
  s.to_c_str().with_ref(|c_str|
  { unsafe { ll::addnstr(c_str, n) } })
}


pub fn addstr(s: &str) -> i32
{
  s.to_c_str().with_ref( |c_str|
  { unsafe { ll::addstr(c_str) } })
}


pub fn attroff(a: i32) -> i32
{ unsafe { ll::attroff(a) } }


pub fn attron(a: i32) -> i32
{ unsafe { ll::attron(a) } }


pub fn attrset(a: i32) -> i32
{ unsafe { ll::attrset(a) } }


pub fn attr_get(attrs: &mut i32, pair: &mut i16) -> i32
{
  unsafe
  {
    ll::attr_get(&*attrs as *i32,
                 &*pair as *i16,
                 ptr::null())
  }
}


pub fn attr_off(a: i32) -> i32
{ unsafe { ll::attr_off(a, ptr::null()) } }


pub fn attr_on(a: i32) -> i32
{ unsafe { ll::attr_on(a, ptr::null()) } }


pub fn attr_set(attr: i32, pair: i16) -> i32
{ unsafe { ll::attr_set(attr, pair, ptr::null()) } }


pub fn baudrate() -> i32
{ unsafe { ll::baudrate() } }


pub fn beep() -> i32
{ unsafe { ll::beep() } }


pub fn bkgd(ch: u32) -> i32
{ unsafe { ll::bkgd(ch) } }


pub fn bkgdset(ch: u32)
{ unsafe { ll::bkgdset(ch) } }


pub fn border(ls: u32, rs: u32, ts: u32, bs: u32, tl: u32, tr: u32, bl: u32, br: u32) -> i32
{ unsafe { ll::border(ls, rs, ts, bs, tl, tr, bl, br) } }


#[link_name="box"] pub fn box_(w: WINDOW, v: u32, h: u32) -> i32
{ wborder(w, v, v, h, h, 0, 0, 0, 0) }


pub fn can_change_color() -> bool
{ unsafe { ll::can_change_color() == TRUE } }


pub fn cbreak() -> i32
{ unsafe { ll::cbreak() } }


pub fn chgat(n: i32, attr: i32, color: i16) -> i32
{ unsafe { ll::chgat(n, attr, color, ptr::null()) } }


pub fn clear() -> i32
{ unsafe { ll::clear() } }


pub fn clearok(w: WINDOW, ok: bool) -> i32
{ unsafe { ll::clearok(w, ok as libc::c_int) } }


pub fn clrtobot() -> i32
{ unsafe { ll::clrtobot() } }


pub fn clrtoeol() -> i32
{ unsafe { ll::clrtoeol() } }


pub fn color_content(color: i16, r: &mut i16, g: &mut i16, b: &mut i16) -> i32
{
  unsafe
  {
    ll::color_content(color,
                      &*r as *i16,
                      &*g as *i16,
                      &*b as *i16)
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


pub fn echochar(c: u32) -> i32
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


pub fn getbkgd(w: WINDOW) -> u32
{ unsafe { ll::getbkgd(w) } }


pub fn getch() -> i32
{ unsafe { ll::getch() } }


pub fn getnstr(s: &mut String, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    s.as_mut_vec().clear();
    s.reserve(n as uint);
    let buf = s.as_bytes().as_ptr();
    let ret = ll::getnstr(mem::transmute(buf), n);

    let capacity = s.byte_capacity();
    match s.as_slice().find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as uint),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn getstr(s: &mut String) -> i32
{
  /* XXX: This is probably broken. */
  let mut ch = getch();
  while ch != '\n' as i32 && ch != '\r' as i32
  {
    unsafe { s.push_byte(ch as u8); }
    ch = getch();
  }
  OK
}


pub fn getwin(reader: *libc::FILE) -> WINDOW
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


pub fn hline(ch: u32, n: i32) -> i32
{ unsafe { ll::hline(ch, n) } }


pub fn idcok(w: WINDOW, bf: bool)
{ unsafe { ll::idcok(w, bf as libc::c_int) } }


pub fn idlok(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::idlok(w, bf as libc::c_int) } }


pub fn immedok(w: WINDOW, bf: bool)
{ unsafe { ll::immedok(w, bf as libc::c_int) } }


pub fn inch() -> u32
{ unsafe { ll::inch() } }


pub fn inchnstr(s: &mut Vec<u32>, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  s.clear();
  s.reserve(n as uint);
  unsafe
  {
    let ret = ll::inchnstr(s.as_ptr(), n);

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as uint),
      None => s.set_len(capacity),
    }

    ret
  }
}


pub fn inchstr(s: &mut Vec<u32>) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let ret = ll::inchstr(s.as_ptr());

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as uint),
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
    s.reserve(n as uint);
    let buf = s.as_bytes().as_ptr();
    let ret = ll::innstr(mem::transmute(buf), n);

    let capacity = s.byte_capacity();
    match s.as_slice().find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as uint),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn insch(ch: u32) -> i32
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

    let capacity = s.byte_capacity();
    match s.as_slice().find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as uint),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn intrflush(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::intrflush(w, bf as libc::c_int) } }


pub fn isendwin() -> bool
{ unsafe { ll::isendwin() == TRUE } }


pub fn is_linetouched(w: WINDOW, l: i32) -> bool
{ unsafe { ll::is_linetouched(w, l) == TRUE } }


pub fn is_wintouched(w: WINDOW) -> bool
{ unsafe { ll::is_wintouched(w) == TRUE } }


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


pub fn keyname(c: i32) -> String
{ unsafe { str::raw::from_c_str(ll::keyname(c)) } }


pub fn keypad(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::keypad(w, bf as libc::c_int) } }


pub fn killchar() -> char
{ unsafe { char::from_u32(ll::killchar() as u32).expect("Invalid char") } }


pub fn leaveok(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::leaveok(w, bf as libc::c_int) } }


pub fn longname() -> String
{ unsafe { str::raw::from_c_str(ll::longname()) } }


pub fn meta(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::meta(w, bf as libc::c_int) } }


pub fn move(y: i32, x: i32) -> i32
{ unsafe { ll::move(y, x) } }


pub fn mvaddch(y: i32, x: i32, c: u32) -> i32
{ unsafe { ll::mvaddch(y, x, c) } }


pub fn mvaddchnstr(y: i32, x: i32, s: &[u32], n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  addchnstr(s, n)
}


pub fn mvaddchstr(y: i32, x: i32, s: &[u32]) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  addchstr(s)
}


pub fn mvaddnstr(y: i32, x: i32, s: &str, n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  addnstr(s, n)
}


pub fn mvaddstr(y: i32, x: i32, s: &str) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  addstr(s)
}


pub fn mvchgat(y: i32, x: i32, n: i32, attr: i32, color: i16) -> i32
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
  if move(y, x) == ERR
  { return ERR; }
  getnstr(s, n)
}


pub fn mvgetstr(y: i32, x: i32, s: &mut String) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  getstr(s)
}


pub fn mvhline(y: i32, x: i32, ch: u32, n: i32) -> i32
{ unsafe { ll::mvhline(y, x, ch, n) } }


pub fn mvinch(y: i32, x: i32) -> u32
{ unsafe { ll::mvinch(y, x) } }


pub fn mvinchnstr(y: i32, x: i32, s: &mut Vec<u32>, n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  inchnstr(s, n)
}


pub fn mvinchstr(y: i32, x: i32, s: &mut Vec<u32>) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  inchstr(s)
}


pub fn mvinnstr(y: i32, x: i32, s: &mut String, n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  innstr(s, n)
}


pub fn mvinsch(y: i32, x: i32, ch: u32) -> i32
{ unsafe { ll::mvinsch(y, x, ch) } }


pub fn mvinsnstr(y: i32, x: i32, s: &str, n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  insnstr(s, n)
}


pub fn mvinsstr(y: i32, x: i32, s: &str) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  insstr(s)
}


pub fn mvinstr(y: i32, x: i32, s: &mut String) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  instr(s)
}


pub fn mvprintw(y: i32, x: i32, s: &str) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  printw(s)
}


pub fn mvvline(y: i32, x: i32, ch: u32, n: i32) -> i32
{ unsafe { ll::mvvline(y, x, ch, n) } }


pub fn mvwaddch(w: WINDOW, y: i32, x: i32, ch: u32) -> i32
{ unsafe { ll::mvwaddch(w, y, x, ch) } }


pub fn mvwaddchnstr(w: WINDOW, y: i32, x: i32, s: &[u32], n: i32) -> i32
{ unsafe { ll::mvwaddchnstr(w, y, x, s.as_ptr(), n) } }


pub fn mvwaddchstr(w: WINDOW, y: i32, x: i32, s: &[u32]) -> i32
{ unsafe { ll::mvwaddchstr(w, y, x, s.as_ptr()) } }


pub fn mvwaddnstr(w: WINDOW, y: i32, x: i32, s: &str, n: i32) -> i32
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { ll::mvwaddnstr(w, y, x, c_str, n) })
  }
}


pub fn mvwaddstr(w: WINDOW, y: i32, x: i32, s: &str) -> i32
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { ll::mvwaddstr(w, y, x, c_str) })
  }
}


pub fn mvwchgat(w: WINDOW, y: i32, x: i32, n: i32, attr: i32, color: i16) -> i32
{ unsafe { ll::mvwchgat(w, y, x, n, attr, color, ptr::null()) } }


pub fn mvwdelch(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { ll::mvwdelch(w, y, x) } }


pub fn mvwgetch(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { ll::mvwgetch(w, y, x) } }


pub fn mvwgetnstr(w: WINDOW, y: i32, x: i32, s: &mut String, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    s.as_mut_vec().clear();
    s.reserve(n as uint);
    let buf = s.as_bytes().as_ptr();
    let ret = ll::mvwgetnstr(w, y, x, mem::transmute(buf), n);

    let capacity = s.byte_capacity();
    match s.as_slice().find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as uint),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn mvwgetstr(w: WINDOW, y: i32, x: i32, s: &mut String) -> i32
{
  if move(y, x) == ERR
  { return ERR; }

  /* XXX: This is probably broken. */
  let mut ch = wgetch(w);
  while ch != '\n' as i32 && ch != '\r' as i32
  {
    unsafe { s.push_byte(ch as u8); }
    ch = wgetch(w);
  }
  OK
}


pub fn mvwhline(w: WINDOW, y: i32, x: i32, ch: u32, n: i32) -> i32
{ unsafe { ll::mvwhline(w, y, x, ch, n) } }


pub fn mvwin(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { ll::mvwin(w, y, x) } }


pub fn mvwinch(w: WINDOW, y: i32, x: i32) -> u32
{ unsafe { ll::mvwinch(w, y, x) } }


pub fn mvwinchnstr(w: WINDOW, y: i32, x: i32, s: &mut Vec<u32>, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  s.clear();
  s.reserve(n as uint);
  unsafe
  {
    let ret = ll::mvwinchnstr(w, y, x, s.as_ptr(), n);

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as uint),
      None => s.set_len(capacity),
    }

    ret
  }
}


pub fn mvwinchstr(w: WINDOW, y: i32, x: i32, s: &mut Vec<u32>) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let ret = ll::mvwinchstr(w, y, x, s.as_ptr());

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as uint),
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
    s.reserve(n as uint);
    let buf = s.as_bytes().as_ptr();
    let ret = ll::mvwinnstr(w, y, x, mem::transmute(buf), n);

    let capacity = s.byte_capacity();
    match s.as_slice().find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as uint),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn mvwinsch(w: WINDOW, y: i32, x: i32, ch: u32) -> i32
{ unsafe { ll::mvwinsch(w, y, x, ch) } }


pub fn mvwinsnstr(w: WINDOW, y: i32, x: i32, s: &str, n: i32) -> i32
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { ll::mvwinsnstr(w, y, x, c_str, n) })
  }
}


pub fn mvwinsstr(w: WINDOW, y: i32, x: i32, s: &str) -> i32
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { ll::mvwinsstr(w, y, x, c_str) })
  }
}


pub fn mvwinstr(w: WINDOW, y: i32, x: i32, s: &mut String) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let buf = s.as_bytes().as_ptr();
    let ret = ll::mvwinstr(w, y, x, mem::transmute(buf));

    let capacity = s.byte_capacity();
    match s.as_slice().find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as uint),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn mvwprintw(w: WINDOW, y: i32, x: i32, s: &str) -> i32
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { ll::mvwprintw(w, y, x, c_str) })
  }
}


pub fn mvwvline(w: WINDOW, y: i32, x: i32, ch: u32, n: i32) -> i32
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
      Some(s) => s.to_c_str().with_ref( |c_str|
        { ll::newterm(c_str, out_fd, in_fd) }),
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
{ unsafe { ll::nodelay(w, bf as libc::c_int) } }


pub fn noecho() -> i32
{ unsafe { ll::noecho() } }


pub fn nonl() -> i32
{ unsafe { ll::nonl() } }


pub fn noqiflush()
{ unsafe { ll::noqiflush() } }


pub fn noraw() -> i32
{ unsafe { ll::noraw() } }


pub fn notimeout(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::notimeout(w, bf as libc::c_int) } }


pub fn overlay(src: WINDOW, dst: WINDOW) -> i32
{ unsafe { ll::overlay(src, dst) } }


pub fn overwrite(src: WINDOW, dst: WINDOW) -> i32
{ unsafe { ll::overwrite(src, dst) } }


pub fn pair_content(pair: i16, f: &mut i16, b: &mut i16) -> i32
{ unsafe { ll::pair_content(pair, &*f as *i16, &*b as *i16) } }


pub fn PAIR_NUMBER(attr: i32) -> i32
{ unsafe { ll::PAIR_NUMBER(attr) } }


pub fn pechochar(pad: WINDOW, ch: u32) -> i32
{ unsafe { ll::pechochar(pad, ch) } }


pub fn pnoutrefresh(pad: WINDOW, pmin_row: i32, pmin_col: i32, smin_row: i32, smin_col: i32, smax_row: i32, smax_col: i32) -> i32
{ unsafe { ll::pnoutrefresh(pad, pmin_row, pmin_col, smin_row, smin_col, smax_row, smax_col) } }


pub fn prefresh(pad: WINDOW, pmin_row: i32, pmin_col: i32, smin_row: i32, smin_col: i32, smax_row: i32, smax_col: i32) -> i32
{ unsafe { ll::prefresh(pad, pmin_row, pmin_col, smin_row, smin_col, smax_row, smax_col) } }


pub fn printw(s: &str) -> i32
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { ll::printw(c_str) })
  }
}


pub fn putp(s: &str) -> i32
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { ll::putp(c_str) })
  }
}


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


pub fn savetty() -> i32
{ unsafe { ll::savetty() } }


pub fn scr_dump(filename: &str) -> i32
{
  unsafe
  {
    filename.to_c_str().with_ref( |c_str|
    { ll::scr_dump(c_str) })
  }
}


pub fn scr_init(filename: &str) -> i32
{
  unsafe
  {
    filename.to_c_str().with_ref( |c_str|
    { ll::scr_init(c_str) })
  }
}


pub fn scrl(n: i32) -> i32
{ unsafe { ll::scrl(n) } }


pub fn scroll(w: WINDOW) -> i32
{ unsafe { ll::scroll(w) } }


pub fn scrollok(w: WINDOW, bf: bool) -> i32
{ unsafe { ll::scrollok(w, bf as libc::c_int) } }


pub fn scr_restore(filename: &str) -> i32
{
  unsafe
  {
    filename.to_c_str().with_ref( |c_str|
    { ll::scr_restore(c_str) })
  }
}


pub fn scr_set(filename: &str) -> i32
{
  unsafe
  {
    filename.to_c_str().with_ref( |c_str|
    { ll::scr_set(c_str) })
  }
}


pub fn setscrreg(top: i32, bot: i32) -> i32
{ unsafe { ll::setscrreg(top, bot) } }


pub fn set_term(s: SCREEN) -> SCREEN
{ unsafe { ll::set_term(s) } }


pub fn slk_attroff(ch: u32) -> i32
{ unsafe { ll::slk_attroff(ch) } }

//
//pub fn slk_attr_off(ch: i32) -> i32
//{ unsafe { ll::slk_attr_off(ch, ptr::null()) } }


pub fn slk_attron(ch: u32) -> i32
{ unsafe { ll::slk_attron(ch) } }

//
//pub fn slk_attr_on(ch: i32) -> i32
//{ unsafe { ll::slk_attr_on(ch, ptr::null()) } }


pub fn slk_attrset(ch: u32) -> i32
{ unsafe { ll::slk_attrset(ch) } }


pub fn slk_attr() -> i32
{ unsafe { ll::slk_attr() } }


pub fn slk_attr_set(attrs: i32, pair: i16) -> i32
{ unsafe { ll::slk_attr_set(attrs, pair, ptr::null()) } }


pub fn slk_clear() -> i32
{ unsafe { ll::slk_clear() } }


pub fn slk_color(pair: i16) -> i32
{ unsafe { ll::slk_color(pair) } }


pub fn slk_init(fmt: i32) -> i32
{ unsafe { ll::slk_init(fmt) } }


pub fn slk_label(n: i32) -> String
{ unsafe { str::raw::from_c_str(ll::slk_label(n)) } }


pub fn slk_noutrefresh() -> i32
{ unsafe { ll::slk_noutrefresh() } }


pub fn slk_refresh() -> i32
{ unsafe { ll::slk_refresh() } }


pub fn slk_restore() -> i32
{ unsafe { ll::slk_restore() } }


pub fn slk_set(n: i32, s: &str, fmt: i32) -> i32
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { ll::slk_set(n, c_str, fmt) })
  }
}


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
{ unsafe { ll::syncok(w, bf as libc::c_int) } }


pub fn termattrs() -> u32
{ unsafe { ll::termattrs() } }


pub fn termname() -> String
{ unsafe { str::raw::from_c_str(ll::termname()) } }


pub fn timeout(delay: i32)
{ unsafe { ll::timeout(delay) } }


pub fn touchline(w: WINDOW, start: i32, count: i32) -> i32
{ unsafe { ll::touchline(w, start, count) } }


pub fn touchwin(w: WINDOW) -> i32
{ unsafe { ll::touchwin(w) } }


pub fn typeahead(fd: i32) -> i32
{ unsafe { ll::typeahead(fd) } }


pub fn tigetflag(capname: &str) -> i32
{
  unsafe
  {
    capname.to_c_str().with_ref( |c_str|
    { ll::tigetflag(c_str) })
  }
}


pub fn tigetnum(capname: &str) -> i32
{
  unsafe
  {
    capname.to_c_str().with_ref( |c_str|
    { ll::tigetnum(c_str) })
  }
}


pub fn tigetstr(capname: &str) -> String
{
  unsafe
  {
    capname.to_c_str().with_ref( |c_str|
    { str::raw::from_c_str(ll::tigetstr(c_str)) })
  }
}


pub fn tparm(s: &str) -> String
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { str::raw::from_c_str(ll::tparm(c_str)) })
  }
}


pub fn ungetch(ch: i32) -> i32
{ unsafe { ll::ungetch(ch) } }


pub fn untouchwin(w: WINDOW) -> i32
{ unsafe { ll::untouchwin(w) } }


pub fn use_env(f: bool)
{ unsafe { ll::use_env(f as libc::c_int) } }


pub fn vidattr(attrs: u32) -> i32
{ unsafe { ll::vidattr(attrs) } }


pub fn vline(ch: u32, n: i32) -> i32
{ unsafe { ll::vline(ch, n) } }


pub fn waddch(w: WINDOW, ch: u32) -> i32
{ unsafe { ll::waddch(w, ch) } }


pub fn waddchnstr(w: WINDOW, s: &[u32], n: i32) -> i32
{ unsafe { ll::waddchnstr(w, s.as_ptr(), n) } }


pub fn waddchstr(w: WINDOW, s: &[u32]) -> i32
{ unsafe { ll::waddchstr(w, s.as_ptr()) } }


pub fn waddnstr(w: WINDOW, s: &str, n: i32) -> i32
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { ll::waddnstr(w, c_str, n) })
  }
}


pub fn waddstr(w: WINDOW, s: &str) -> i32
{
  unsafe
  {
    s.to_c_str().with_ref( |c_str|
    { ll::waddstr(w, c_str) })
  }
}


pub fn wattron(w: WINDOW, attr: i32) -> i32
{ unsafe { ll::wattron(w, attr) } }


pub fn wattroff(w: WINDOW, attr: i32) -> i32
{ unsafe { ll::wattroff(w, attr) } }


pub fn wattrset(w: WINDOW, attr: i32) -> i32
{ unsafe { ll::wattrset(w, attr) } }


pub fn wattr_get(w: WINDOW, attrs: &mut i32, pair: &mut i16) -> i32
{ unsafe { ll::wattr_get(w, &*attrs as *i32, &*pair as *i16, ptr::null()) } }


pub fn wattr_on(w: WINDOW, attr: i32) -> i32
{ unsafe { ll::wattr_on(w, attr, ptr::null()) } }


pub fn wattr_off(w: WINDOW, attr: i32) -> i32
{ unsafe { ll::wattr_off(w, attr, ptr::null()) } }


pub fn wattr_set(w: WINDOW, attrs: i32, pair: i16) -> i32
{ unsafe { ll::wattr_set(w, attrs, pair, ptr::null()) } }


pub fn wbkgd(w: WINDOW, ch: u32) -> i32
{ unsafe { ll::wbkgd(w, ch) } }


pub fn wbkgdset(w: WINDOW, ch: u32)
{ unsafe { ll::wbkgdset(w, ch) } }


pub fn wborder(w: WINDOW, ls: u32, rs: u32, ts: u32, bs: u32, tl: u32, tr: u32, bl: u32, br: u32) -> i32
{ unsafe { ll::wborder(w, ls, rs, ts, bs, tl, tr, bl, br) } }


pub fn wchgat(w: WINDOW, n: i32, attr: i32, color: i16) -> i32
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


pub fn wechochar(w: WINDOW, ch: u32) -> i32
{ unsafe { ll::wechochar(w, ch) } }


pub fn werase(w: WINDOW) -> i32
{ unsafe { ll::werase(w) } }


pub fn wgetch(w: WINDOW) -> i32
{ unsafe { ll::wgetch(w) } }


pub fn wgetnstr(w: WINDOW, s: &mut String, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let buf = s.as_bytes().as_ptr();
    let ret = ll::wgetnstr(w, mem::transmute(buf), n);

    let capacity = s.byte_capacity();
    match s.as_slice().find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as uint),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn wgetstr(w: WINDOW, s: &mut String) -> i32
{
  /* XXX: This is probably broken. */
  let mut ch = wgetch(w);
  while ch != '\n' as i32 && ch != '\r' as i32
  {
    unsafe { s.push_byte(ch as u8); }
    ch = wgetch(w);
  }
  OK
}


pub fn whline(w: WINDOW, ch: u32, n: i32) -> i32
{ unsafe { ll::whline(w, ch, n) } }


pub fn winch(w: WINDOW) -> u32
{ unsafe { ll::winch(w) } }


pub fn winchnstr(w: WINDOW, s: &mut Vec<u32>, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  s.clear();
  s.reserve(n as uint);
  unsafe
  {
    let ret = ll::winchnstr(w, s.as_ptr(), n);

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as uint),
      None => s.set_len(capacity),
    }

    ret
  }
}


pub fn winchstr(w: WINDOW, s: &mut Vec<u32>) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let ret = ll::winchstr(w, s.as_ptr());

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => s.set_len(index as uint),
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
    s.reserve(n as uint);
    let buf = s.as_bytes().as_ptr();
    let ret = ll::winnstr(w, mem::transmute(buf), n);

    let capacity = s.byte_capacity();
    match s.as_slice().find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as uint),
      None => s.as_mut_vec().set_len(capacity),
    }

    ret
  }
}


pub fn winsch(w: WINDOW, ch: u32) -> i32
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

    let capacity = s.byte_capacity();
    match s.as_slice().find('\0')
    {
      Some(index) => s.as_mut_vec().set_len(index as uint),
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
{
  unsafe
  {
    s.to_c_str().with_ref(|c_str|
    { ll::wprintw(w, c_str) })
  }
}


pub fn wredrawln(w: WINDOW, start: i32, n: i32) -> i32
{ unsafe { ll::wredrawln(w, start, n) } }


pub fn wrefresh(w: WINDOW) -> i32
{ unsafe { ll::wrefresh(w) } }


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


pub fn wvline(w: WINDOW, ch: u32, n: i32) -> i32
{ unsafe { ll::wvline(w, ch, n) } }


pub fn wgetparent(w: WINDOW) -> WINDOW
{ unsafe { ll::wgetparent(w) } }


pub fn wgetscrreg(w: WINDOW, top: &mut i32, bot: &mut i32) -> i32
{ unsafe { ll::wgetscrreg(w, &*top as *i32, &*bot as *i32) } }

/* Attributes */
pub fn NCURSES_BITS(mask: u32, shift: u32) -> u32
{ ((mask) << ((shift) + NCURSES_ATTR_SHIFT)) }

pub fn A_NORMAL() -> i32
{ (1u32 - 1u32) as i32 }

pub fn A_ATTRIBUTES() -> i32
{ NCURSES_BITS(!(1u32 - 1u32), 0u32) as i32 }

pub fn A_CHARTEXT() -> i32
{(NCURSES_BITS(1u32, 0u32) - 1u32) as i32 }

pub fn A_COLOR() -> i32
{ NCURSES_BITS(((1u32) << 8u32) - 1u32, 0u32) as i32 }

pub fn A_STANDOUT() -> i32
{ NCURSES_BITS(1u32, 8u32) as i32 }

pub fn A_UNDERLINE() -> i32
{ NCURSES_BITS(1u32, 9u32) as i32 }

pub fn A_REVERSE() -> i32
{ NCURSES_BITS(1u32, 10u32) as i32 }

pub fn A_BLINK() -> i32
{ NCURSES_BITS(1u32, 11u32) as i32 }

pub fn A_DIM() -> i32
{ NCURSES_BITS(1u32, 12u32) as i32 }

pub fn A_BOLD() -> i32
{ NCURSES_BITS(1u32, 13u32) as i32 }

pub fn A_ALTCHARSET() -> i32
{ NCURSES_BITS(1u32, 14u32) as i32 }

pub fn A_INVIS() -> i32
{ NCURSES_BITS(1u32, 15u32) as i32 }

pub fn A_PROTECT() -> i32
{ NCURSES_BITS(1u32, 16u32) as i32 }

pub fn A_HORIZONTAL() -> i32
{ NCURSES_BITS(1u32, 17u32) as i32 }

pub fn A_LEFT() -> i32
{ NCURSES_BITS(1u32, 18u32) as i32 }

pub fn A_LOW() -> i32
{ NCURSES_BITS(1u32, 19u32) as i32 }

pub fn A_RIGHT() -> i32
{ NCURSES_BITS(1u32, 20u32) as i32 }

pub fn A_TOP() -> i32
{ NCURSES_BITS(1u32, 21u32) as i32 }

pub fn A_VERTICAL() -> i32
{ NCURSES_BITS(1u32, 22u32) as i32 }

/* Colors. */
pub fn COLOR_PAIR(n: i16) -> i32
{ NCURSES_BITS(n as u32, 0u32) as i32 }

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
    if newscr != ptr::null()
    {
      if ll::is_leaveok(newscr) == TRUE
      {
        *x = -1 as i32;
        *y = -1 as i32;
      }
      else
      { getyx(newscr, (y), (x)); }
    }
  }
}


pub fn setsyx(y: &mut i32, x: &mut i32)
{
  unsafe
  {
    if newscr !=(0 as WINDOW)
    {
      if *y == -1 && *x == -1
      {
        ll::leaveok(newscr, 1);
      }
      else
      {
        ll::leaveok(newscr, 0);
        ll::wmove(newscr, *y, *x);
      }
    }
  }
}

/* Line graphics */
pub fn NCURSES_ACS(c: char) -> char
{ unsafe { char::from_u32(*((acs_map as i32 + 4 * (c as i32)) as *u32)).expect("Invalid char") } }

/* VT100 symbols begin here */
pub fn ACS_ULCORNER() -> char
{ NCURSES_ACS('l') } /* upper left corner */

pub fn ACS_LLCORNER() -> char
{ NCURSES_ACS('m') } /* lower left corner */

pub fn ACS_URCORNER() -> char
{ NCURSES_ACS('k') } /* upper right corner */

pub fn ACS_LRCORNER() -> char
{ NCURSES_ACS('j') } /* lower right corner */

pub fn ACS_LTEE() -> char
{ NCURSES_ACS('t') } /* tee pointing right */

pub fn ACS_RTEE() -> char
{ NCURSES_ACS('u') } /* tee pointing left */

pub fn ACS_BTEE() -> char
{ NCURSES_ACS('v') } /* tee pointing up */

pub fn ACS_TTEE() -> char
{ NCURSES_ACS('w') } /* tee pointing down */

pub fn ACS_HLINE() -> char
{ NCURSES_ACS('q') } /* horizontal line */

pub fn ACS_VLINE() -> char
{ NCURSES_ACS('x') } /* vertical line */

pub fn ACS_PLUS() -> char
{ NCURSES_ACS('n') } /* large plus or crossover */

pub fn ACS_S1() -> char
{ NCURSES_ACS('o') } /* scan line 1 */

pub fn ACS_S9() -> char
{ NCURSES_ACS('s') } /* scan line 9 */

pub fn ACS_DIAMOND() -> char
{ NCURSES_ACS('`') } /* diamond */

pub fn ACS_CKBOARD() -> char
{ NCURSES_ACS('a') } /* checker board(stipple) */

pub fn ACS_DEGREE() -> char
{ NCURSES_ACS('f') } /* degree symbol */

pub fn ACS_PLMINUS() -> char
{ NCURSES_ACS('g') } /* plus/minus */

pub fn ACS_BULLET() -> char
{ NCURSES_ACS('~') } /* bullet */

/* Teletype 5410v1 symbols begin here */
pub fn ACS_LARROW() -> char
{ NCURSES_ACS(',') } /* arrow pointing left */

pub fn ACS_RARROW() -> char
{ NCURSES_ACS('+') } /* arrow pointing right */

pub fn ACS_DARROW() -> char
{ NCURSES_ACS('.') } /* arrow pointing down */

pub fn ACS_UARROW() -> char
{ NCURSES_ACS('-') } /* arrow pointing up */

pub fn ACS_BOARD() -> char
{ NCURSES_ACS('h') } /* board of squares */

pub fn ACS_LANTERN() -> char
{ NCURSES_ACS('i') } /* lantern symbol */

pub fn ACS_BLOCK() -> char
{ NCURSES_ACS('0') } /* solid square block */

/*
 * These aren't documented, but a lot of System Vs have them anyway
 *(you can spot pprryyzz{{||}} in a lot of AT&T terminfo strings).
 * The ACS_names may not match AT&T's, our source didn't know them.
 */
pub fn ACS_S3() -> char
{ NCURSES_ACS('p') } /* scan line 3 */

pub fn ACS_S7() -> char
{ NCURSES_ACS('r') } /* scan line 7 */

pub fn ACS_LEQUAL() -> char
{ NCURSES_ACS('y') } /* less/equal */

pub fn ACS_GEQUAL() -> char
{ NCURSES_ACS('z') } /* greater/equal */

pub fn ACS_PI() -> char
{ NCURSES_ACS('{') } /* Pi */

pub fn ACS_NEQUAL() -> char
{ NCURSES_ACS('|') } /* not equal */

pub fn ACS_STERLING() -> char
{ NCURSES_ACS('}') } /* UK pound sign */

/*
 * Line drawing ACS names are of the form ACS_trbl, where t is the top, r
 * is the right, b is the bottom, and l is the left. t, r, b, and l might
 * be B(blank), S(single), D(double), or T(thick). The subset defined
 * here only uses B and S.
 */
pub fn ACS_BSSB() -> char
{ ACS_ULCORNER() }

pub fn ACS_SSBB() -> char
{ ACS_LLCORNER() }

pub fn ACS_BBSS() -> char
{ ACS_URCORNER() }

pub fn ACS_SBBS() -> char
{ ACS_LRCORNER() }

pub fn ACS_SBSS() -> char
{ ACS_RTEE() }

pub fn ACS_SSSB() -> char
{ ACS_LTEE() }

pub fn ACS_SSBS() -> char
{ ACS_BTEE() }

pub fn ACS_BSSS() -> char
{ ACS_TTEE() }

pub fn ACS_BSBS() -> char
{ ACS_HLINE() }

pub fn ACS_SBSB() -> char
{ ACS_VLINE() }

pub fn ACS_SSSS() -> char
{ ACS_PLUS() }

pub fn KEY_F(n: u8) -> i32
{
  assert!(n < 16);
  KEY_F0 + n as i32
}

/*
 * Added mouse support
 */

pub fn has_mouse() -> i32
{ unsafe { ll::has_mouse() } }

pub fn getmouse(event: *MEVENT) -> i32
{ unsafe { ll::getmouse(event) } }

pub fn ungetmouse(event: *MEVENT) -> i32
{ unsafe { ll::ungetmouse(event) } }

pub fn mouseinterval(n: i32) -> i32
{ unsafe { ll::mouseinterval(n) } }

pub fn mousemask(newmask: mmask_t, oldmask: Option<&mmask_t>) -> mmask_t
{
    if oldmask.is_none() { unsafe { ll::mousemask(newmask, ptr::null()) } }
    else { unsafe { ll::mousemask(newmask, oldmask.unwrap()) } }
}

pub fn wenclose(w: WINDOW, y: i32, x: i32) -> i32
{ unsafe { ll::wenclose(w, y as libc::c_int, x as libc::c_int) } }

pub fn wmouse_trafo(w: *WINDOW, y: &[i32], x: &[i32], to_screen: bool) -> i32
{ unsafe { ll::wmouse_trafo(w, y.as_ptr(), x.as_ptr(), to_screen as libc::c_int) } }

pub fn mouse_trafo( y: &[i32], x: &[i32], to_screen: bool  ) -> i32
{ unsafe { ll::mouse_trafo(y.as_ptr(), x.as_ptr(), to_screen as libc::c_int ) } }
