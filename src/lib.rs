/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: lib.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Safe wrappers for ncurses functions.
*/

#[link(name="ncurses", vers="5.7")];
#[crate_type = "lib"];

use std::{ str, vec, char, libc, ptr };
use std::libc::c_void;
use self::ll::*;
pub use self::constants::*;

pub mod ll;
pub mod constants;

pub enum Cursor_Visibility
{
  Cursor_Invisible = 0,
  Cursor_Visible,
  Cursor_Very_Visible,
}

#[fixed_stack_segment]
pub fn addch(ch: u32) -> i32
{ unsafe { ll::addch(ch) } }

#[fixed_stack_segment]
pub fn addchnstr(s: &[u32], n: i32) -> i32
{ unsafe { ll::addchnstr(vec::raw::to_ptr(s), n) } }

#[fixed_stack_segment]
pub fn addchstr(s: &[u32]) -> i32
{ unsafe { ll::addchstr(vec::raw::to_ptr(s)) } }

#[fixed_stack_segment]
pub fn addnstr(s: &str, n: i32) -> i32
{
  do s.to_c_str().with_ref() |c_str|
  { unsafe { ll::addnstr(c_str, n) } }
}

#[fixed_stack_segment]
pub fn addstr(s: &str) -> i32
{
  do s.to_c_str().with_ref() |c_str|
  { unsafe { ll::addstr(c_str) } }
}

#[fixed_stack_segment]
pub fn attroff(a: i32) -> i32
{ unsafe { ll::attroff(a) } }

#[fixed_stack_segment]
pub fn attron(a: i32) -> i32
{ unsafe { ll::attron(a) } }

#[fixed_stack_segment]
pub fn attrset(a: i32) -> i32
{ unsafe { ll::attrset(a) } }

#[fixed_stack_segment]
pub fn attr_get(attrs: &mut i32, pair: &mut i16) -> i32
{
  unsafe
  {
    ll::attr_get(ptr::to_unsafe_ptr(attrs),
                 ptr::to_unsafe_ptr(pair),
                 ptr::null())
  }
}

#[fixed_stack_segment]
pub fn attr_off(a: i32) -> i32
{ unsafe { ll::attr_off(a, ptr::null()) } }

#[fixed_stack_segment]
pub fn attr_on(a: i32) -> i32
{ unsafe { ll::attr_on(a, ptr::null()) } }

#[fixed_stack_segment]
pub fn attr_set(attr: i32, pair: i16) -> i32
{ unsafe { ll::attr_set(attr, pair, ptr::null()) } }

#[fixed_stack_segment]
pub fn baudrate() -> i32
{ unsafe { ll::baudrate() } }

#[fixed_stack_segment]
pub fn beep() -> i32
{ unsafe { ll::beep() } }

#[fixed_stack_segment]
pub fn bkgd(ch: u32) -> i32
{ unsafe { ll::bkgd(ch) } }

#[fixed_stack_segment]
pub fn bkgdset(ch: u32)
{ unsafe { ll::bkgdset(ch) } }

#[fixed_stack_segment]
pub fn border(ls: u32, rs: u32, ts: u32, bs: u32, tl: u32, tr: u32, bl: u32, br: u32) -> i32
{ unsafe { ll::border(ls, rs, ts, bs, tl, tr, bl, br) } }

#[fixed_stack_segment]
pub fn box(w: WINDOW_p, v: u32, h: u32) -> i32
{ wborder(w, v, v, h, h, 0, 0, 0, 0) }

#[fixed_stack_segment]
pub fn can_change_color() -> bool
{ unsafe { ll::can_change_color() == OK } }

#[fixed_stack_segment]
pub fn cbreak() -> i32
{ unsafe { ll::cbreak() } }

#[fixed_stack_segment]
pub fn chgat(n: i32, attr: i32, color: i16) -> i32
{ unsafe { ll::chgat(n, attr, color, ptr::null()) } }

#[fixed_stack_segment]
pub fn clear() -> i32
{ unsafe { ll::clear() } }

#[fixed_stack_segment]
pub fn clearok(w: WINDOW_p, ok: bool) -> i32
{ unsafe { ll::clearok(w, ok as libc::c_int) } }

#[fixed_stack_segment]
pub fn clrtobot() -> i32
{ unsafe { ll::clrtobot() } }

#[fixed_stack_segment]
pub fn clrtoeol() -> i32
{ unsafe { ll::clrtoeol() } }

#[fixed_stack_segment]
pub fn color_content(color: i16, r: &mut i16, g: &mut i16, b: &mut i16) -> i32
{
  unsafe
  {
    ll::color_content(color,
                      ptr::to_unsafe_ptr(r),
                      ptr::to_unsafe_ptr(g),
                      ptr::to_unsafe_ptr(b))
  }
}

#[fixed_stack_segment]
pub fn color_set(pair: i16) -> i32
{ unsafe { ll::color_set(pair, ptr::null()) } }

#[fixed_stack_segment]
pub fn copywin(src_win: WINDOW_p, dest_win: WINDOW_p, src_min_row: i32,
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

#[fixed_stack_segment]
pub fn curs_set(visibility: Cursor_Visibility) -> Option<Cursor_Visibility>
{
  use std::cast;
  unsafe
  {
    match ll::curs_set(visibility as i32)
    {
      ERR => None,
      ret => Some(cast::transmute::<i64, Cursor_Visibility>(ret as i64)),
    }
  }
}

#[fixed_stack_segment]
pub fn def_prog_mode() -> i32
{ unsafe { ll::def_prog_mode() } }

#[fixed_stack_segment]
pub fn def_shell_mode() -> i32
{ unsafe { ll::def_shell_mode() } }

#[fixed_stack_segment]
pub fn delay_output(ms: i32) -> i32
{ unsafe { ll::delay_output(ms) } }

#[fixed_stack_segment]
pub fn delch() -> i32
{ unsafe { ll::delch() } }

#[fixed_stack_segment]
pub fn delscreen(s: SCREEN_p)
{ unsafe { ll::delscreen(s) } }

#[fixed_stack_segment]
pub fn delwin(w: WINDOW_p) -> i32
{ unsafe { ll::delwin(w) } }

#[fixed_stack_segment]
pub fn deleteln() -> i32
{ unsafe { ll::deleteln() } }

#[fixed_stack_segment]
pub fn derwin(w: WINDOW_p, lines: i32, cols: i32, x: i32, y: i32) -> WINDOW_p
{ unsafe { ll::derwin(w, lines, cols, x, y) } }

#[fixed_stack_segment]
pub fn doupdate() -> i32
{ unsafe { ll::doupdate() } }

#[fixed_stack_segment]
pub fn dupwin(w: WINDOW_p) -> WINDOW_p
{ unsafe { ll::dupwin(w) } }

#[fixed_stack_segment]
pub fn echo() -> i32
{ unsafe { ll::echo() } }

#[fixed_stack_segment]
pub fn echochar(c: u32) -> i32
{ unsafe { ll::echochar(c) } }

#[fixed_stack_segment]
pub fn erase() -> i32
{ unsafe { ll::erase() } }

#[fixed_stack_segment]
pub fn endwin() -> i32
{ unsafe { ll::endwin() } }

#[fixed_stack_segment]
pub fn erasechar() -> char
{ unsafe { char::from_u32(ll::erasechar() as u32).expect("Invalid char") } }

#[fixed_stack_segment]
pub fn filter()
{ unsafe { ll::filter() } }

#[fixed_stack_segment]
pub fn flash() -> i32
{ unsafe { ll::flash() } }

#[fixed_stack_segment]
pub fn flushinp() -> i32
{ unsafe { ll::flushinp() } }

#[fixed_stack_segment]
pub fn getbkgd(w: WINDOW_p) -> u32
{ unsafe { ll::getbkgd(w) } }

#[fixed_stack_segment]
pub fn getch() -> i32
{ unsafe { ll::getch() } }

#[fixed_stack_segment]
pub fn getnstr(s: &mut ~str, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  use std::cast;

  s.clear();
  s.reserve_at_least(n as uint);
  unsafe
  {
    let ret = do s.as_mut_buf |buf, _len|
    { ll::getnstr(cast::transmute(buf), n) };

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => str::raw::set_len(s, index as uint),
      None => str::raw::set_len(s, capacity),
    }

    ret
  }
}

#[fixed_stack_segment]
pub fn getstr(s: &mut ~str) -> i32
{
  /* XXX: This is probably broken. */
  let mut ch = getch();
  while ch != '\n' as i32 && ch != '\r' as i32
  {
    unsafe { str::raw::push_byte(s, ch as u8); }
    ch = getch();
  }
  OK
}

#[fixed_stack_segment]
pub fn getwin(reader: *libc::FILE) -> WINDOW_p
{ unsafe { ll::getwin(reader) } } /* TODO: Make this safe. */

#[fixed_stack_segment]
pub fn halfdelay(tenths: i32) -> i32
{ unsafe { ll::halfdelay(tenths) } }

#[fixed_stack_segment]
pub fn has_colors() -> bool
{ unsafe { ll::has_colors() == OK } }

#[fixed_stack_segment]
pub fn has_ic() -> bool
{ unsafe { ll::has_ic() == OK } }

#[fixed_stack_segment]
pub fn has_il() -> bool
{ unsafe { ll::has_il() == OK } }

#[fixed_stack_segment]
pub fn hline(ch: u32, n: i32) -> i32
{ unsafe { ll::hline(ch, n) } }

#[fixed_stack_segment]
pub fn idcok(w: WINDOW_p, bf: bool)
{ unsafe { ll::idcok(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn idlok(w: WINDOW_p, bf: bool) -> i32
{ unsafe { ll::idlok(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn immedok(w: WINDOW_p, bf: bool)
{ unsafe { ll::immedok(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn inch() -> u32
{ unsafe { ll::inch() } }

#[fixed_stack_segment]
pub fn inchnstr(s: &mut ~[u32], n: i32) -> i32
{
  /* XXX: This is probably broken. */
  s.clear();
  s.reserve_at_least(n as uint);
  unsafe
  {
    let ret = ll::inchnstr(vec::raw::to_ptr(*s), n);

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => vec::raw::set_len(s, index as uint),
      None => vec::raw::set_len(s, capacity),
    }

    ret
  }
}

#[fixed_stack_segment]
pub fn inchstr(s: &mut ~[u32]) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let ret = ll::inchstr(vec::raw::to_ptr(*s));

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => vec::raw::set_len(s, index as uint),
      None => vec::raw::set_len(s, capacity),
    }

    ret
  }
}

#[fixed_stack_segment]
pub fn initscr() -> WINDOW_p
{ unsafe { ll::initscr() } }

#[fixed_stack_segment]
pub fn init_color(color: i16, r: i16, g: i16, b: i16) -> i32
{ unsafe { ll::init_color(color, r, g, b) } }

#[fixed_stack_segment]
pub fn init_pair(pair: i16, f: i16, b: i16) -> i32
{ unsafe { ll::init_pair(pair, f, b) } }

#[fixed_stack_segment]
pub fn innstr(s: &mut ~str, n: i32) -> i32
{
  use std::cast; 

  /* XXX: This is probably broken. */
  s.clear();
  s.reserve_at_least(n as uint);
  unsafe
  {
    let ret = do s.as_mut_buf |buf, _len|
    { ll::innstr(cast::transmute(buf), n) };

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => str::raw::set_len(s, index as uint),
      None => str::raw::set_len(s, capacity),
    }

    ret
  }
}

#[fixed_stack_segment]
pub fn insch(ch: u32) -> i32
{ unsafe { ll::insch(ch) } }

#[fixed_stack_segment]
pub fn insdelln(n: i32) -> i32
{ unsafe { ll::insdelln(n) } }

#[fixed_stack_segment]
pub fn insertln() -> i32
{ unsafe { ll::insertln() } }

#[fixed_stack_segment]
pub fn insnstr(s: &str, n: i32) -> i32
{
  use std::cast;

  unsafe
  {
    do s.as_imm_buf |buf, _len|
    { ll::insnstr(cast::transmute(buf), n) }
  }
}

#[fixed_stack_segment]
pub fn insstr(s: &str) -> i32
{
  use std::cast;

  unsafe
  {
    do s.as_imm_buf |buf, _len|
    { ll::insstr(cast::transmute(buf)) }
  }
}

#[fixed_stack_segment]
pub fn instr(s: &mut ~str) -> i32
{
  use std::cast; 

  /* XXX: This is probably broken. */
  unsafe
  {
    let ret = do s.as_mut_buf |buf, _len|
    { ll::instr(cast::transmute(buf)) };

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => str::raw::set_len(s, index as uint),
      None => str::raw::set_len(s, capacity),
    }

    ret
  }
}

#[fixed_stack_segment]
pub fn intrflush(w: WINDOW_p, bf: bool) -> i32
{ unsafe { ll::intrflush(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn isendwin() -> bool
{ unsafe { ll::isendwin() == OK } }

#[fixed_stack_segment]
pub fn is_linetouched(w: WINDOW_p, l: i32) -> bool
{ unsafe { ll::is_linetouched(w, l) == OK } }

#[fixed_stack_segment]
pub fn is_wintouched(w: WINDOW_p) -> bool
{ unsafe { ll::is_wintouched(w) == OK } }

#[fixed_stack_segment]
pub fn is_cleared(w: WINDOW_p) -> bool
{ unsafe { ll::is_cleared(w) == OK } }

#[fixed_stack_segment]
pub fn is_idcok(w: WINDOW_p) -> bool
{ unsafe { ll::is_idcok(w) == OK } }

#[fixed_stack_segment]
pub fn is_idlok(w: WINDOW_p) -> bool
{ unsafe { ll::is_idlok(w) == OK } }

#[fixed_stack_segment]
pub fn is_immedok(w: WINDOW_p) -> bool
{ unsafe { ll::is_immedok(w) == OK } }

#[fixed_stack_segment]
pub fn is_keypad(w: WINDOW_p) -> bool
{ unsafe { ll::is_keypad(w) == OK } }

#[fixed_stack_segment]
pub fn is_leaveok(w: WINDOW_p) -> bool
{ unsafe { ll::is_leaveok(w) == OK } }

#[fixed_stack_segment]
pub fn is_nodelay(w: WINDOW_p) -> bool
{ unsafe { ll::is_nodelay(w) == OK } }

#[fixed_stack_segment]
pub fn is_notimeout(w: WINDOW_p) -> bool
{ unsafe { ll::is_notimeout(w) == OK } }

#[fixed_stack_segment]
pub fn is_scrollok(w: WINDOW_p) -> bool
{ unsafe { ll::is_scrollok(w) == OK } }

#[fixed_stack_segment]
pub fn is_syncok(w: WINDOW_p) -> bool
{ unsafe { ll::is_syncok(w) == OK }}

#[fixed_stack_segment]
pub fn keyname(c: i32) -> ~str
{ unsafe { str::raw::from_c_str(ll::keyname(c)) } }

#[fixed_stack_segment]
pub fn keypad(w: WINDOW_p, bf: bool) -> i32
{ unsafe { ll::keypad(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn killchar() -> char
{ unsafe { char::from_u32(ll::killchar() as u32).expect("Invalid char") } }

#[fixed_stack_segment]
pub fn leaveok(w: WINDOW_p, bf: bool) -> i32
{ unsafe { ll::leaveok(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn longname() -> ~str
{ unsafe { str::raw::from_c_str(ll::longname()) } }

#[fixed_stack_segment]
pub fn meta(w: WINDOW_p, bf: bool) -> i32
{ unsafe { ll::meta(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn move(y: i32, x: i32) -> i32
{ unsafe { ll::move(y, x) } }

#[fixed_stack_segment]
pub fn mvaddch(y: i32, x: i32, c: u32) -> i32
{ unsafe { ll::mvaddch(y, x, c) } }

#[fixed_stack_segment]
pub fn mvaddchnstr(y: i32, x: i32, s: &[u32], n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  addchnstr(s, n)
}

#[fixed_stack_segment]
pub fn mvaddchstr(y: i32, x: i32, s: &[u32]) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  addchstr(s)
}

#[fixed_stack_segment]
pub fn mvaddnstr(y: i32, x: i32, s: &str, n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  addnstr(s, n)
}

#[fixed_stack_segment]
pub fn mvaddstr(y: i32, x: i32, s: &str) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  addstr(s)
}

#[fixed_stack_segment]
pub fn mvchgat(y: i32, x: i32, n: i32, attr: i32, color: i16) -> i32
{ unsafe { ll::mvchgat(y, x, n, attr, color, ptr::null()) } }

#[fixed_stack_segment]
pub fn mvcur(old_y: i32, old_x: i32, new_y: i32, new_x: i32) -> i32
{ unsafe { ll::mvcur(old_y, old_x, new_y, new_x) } }

#[fixed_stack_segment]
pub fn mvdelch(y: i32, x: i32) -> i32
{ unsafe { ll::mvdelch(y, x) } }

#[fixed_stack_segment]
pub fn mvderwin(w: WINDOW_p, y: i32, x: i32) -> i32
{ unsafe { ll::mvderwin(w, y, x) } }

#[fixed_stack_segment]
pub fn mvgetch(y: i32, x: i32) -> i32
{ unsafe { ll::mvgetch(y, x) } }

#[fixed_stack_segment]
pub fn mvgetnstr(y: i32, x: i32, s: &mut ~str, n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  getnstr(s, n)
}

#[fixed_stack_segment]
pub fn mvgetstr(y: i32, x: i32, s: &mut ~str) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  getstr(s)
}

#[fixed_stack_segment]
pub fn mvhline(y: i32, x: i32, ch: u32, n: i32) -> i32
{ unsafe { ll::mvhline(y, x, ch, n) } }

#[fixed_stack_segment]
pub fn mvinch(y: i32, x: i32) -> u32
{ unsafe { ll::mvinch(y, x) } }

#[fixed_stack_segment]
pub fn mvinchnstr(y: i32, x: i32, s: &mut ~[u32], n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  inchnstr(s, n)
}

#[fixed_stack_segment]
pub fn mvinchstr(y: i32, x: i32, s: &mut ~[u32]) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  inchstr(s)
}

#[fixed_stack_segment]
pub fn mvinnstr(y: i32, x: i32, s: &mut ~str, n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  innstr(s, n)
}

#[fixed_stack_segment]
pub fn mvinsch(y: i32, x: i32, ch: u32) -> i32
{ unsafe { ll::mvinsch(y, x, ch) } }

#[fixed_stack_segment]
pub fn mvinsnstr(y: i32, x: i32, s: &str, n: i32) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  insnstr(s, n)
}

#[fixed_stack_segment]
pub fn mvinsstr(y: i32, x: i32, s: &str) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  insstr(s)
}

#[fixed_stack_segment]
pub fn mvinstr(y: i32, x: i32, s: &mut ~str) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  instr(s)
}

#[fixed_stack_segment]
pub fn mvprintw(y: i32, x: i32, s: &str) -> i32
{
  if move(y, x) == ERR
  { return ERR; }
  printw(s)
}

#[fixed_stack_segment]
pub fn mvvline(y: i32, x: i32, ch: u32, n: i32) -> i32
{ unsafe { ll::mvvline(y, x, ch, n) } }

#[fixed_stack_segment]
pub fn mvwaddch(w: WINDOW_p, y: i32, x: i32, ch: u32) -> i32
{ unsafe { ll::mvwaddch(w, y, x, ch) } }

#[fixed_stack_segment]
pub fn mvwaddchnstr(w: WINDOW_p, y: i32, x: i32, s: &[u32], n: i32) -> i32
{ unsafe { ll::mvwaddchnstr(w, y, x, vec::raw::to_ptr(s), n) } }

#[fixed_stack_segment]
pub fn mvwaddchstr(w: WINDOW_p, y: i32, x: i32, s: &[u32]) -> i32
{ unsafe { ll::mvwaddchstr(w, y, x, vec::raw::to_ptr(s)) } }

#[fixed_stack_segment]
pub fn mvwaddnstr(w: WINDOW_p, y: i32, x: i32, s: &str, n: i32) -> i32
{
  unsafe
  {
    do s.to_c_str().with_ref() |c_str|
    { ll::mvwaddnstr(w, y, x, c_str, n) }
  }
}

#[fixed_stack_segment]
pub fn mvwaddstr(w: WINDOW_p, y: i32, x: i32, s: &str) -> i32
{
  unsafe
  {
    do s.to_c_str().with_ref() |c_str|
    { ll::mvwaddstr(w, y, x, c_str) }
  }
}

#[fixed_stack_segment]
pub fn mvwchgat(w: WINDOW_p, y: i32, x: i32, n: i32, attr: i32, color: i16) -> i32
{ unsafe { ll::mvwchgat(w, y, x, n, attr, color, ptr::null()) } }

#[fixed_stack_segment]
pub fn mvwdelch(w: WINDOW_p, y: i32, x: i32) -> i32
{ unsafe { ll::mvwdelch(w, y, x) } }

#[fixed_stack_segment]
pub fn mvwgetch(w: WINDOW_p, y: i32, x: i32) -> i32
{ unsafe { ll::mvwgetch(w, y, x) } }

#[fixed_stack_segment]
pub fn mvwgetnstr(w: WINDOW_p, y: i32, x: i32, s: &mut ~str, n: i32) -> i32
{
  /* XXX: This is probably broken. */
  use std::cast;

  s.clear();
  s.reserve_at_least(n as uint);
  unsafe
  {
    let ret = do s.as_mut_buf |buf, _len|
    { ll::mvwgetnstr(w, y, x, cast::transmute(buf), n) };

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => str::raw::set_len(s, index as uint),
      None => str::raw::set_len(s, capacity),
    }

    ret
  }
}

#[fixed_stack_segment]
pub fn mvwgetstr(w: WINDOW_p, y: i32, x: i32, s: &mut ~str) -> i32
{
  if move(y, x) == ERR
  { return ERR; }

  /* XXX: This is probably broken. */
  let mut ch = wgetch(w);
  while ch != '\n' as i32 && ch != '\r' as i32
  {
    unsafe { str::raw::push_byte(s, ch as u8); }
    ch = wgetch(w);
  }
  OK
}

#[fixed_stack_segment]
pub fn mvwhline(w: WINDOW_p, y: i32, x: i32, ch: u32, n: i32) -> i32
{ unsafe { ll::mvwhline(w, y, x, ch, n) } }

#[fixed_stack_segment]
pub fn mvwin(w: WINDOW_p, y: i32, x: i32) -> i32
{ unsafe { ll::mvwin(w, y, x) } }

#[fixed_stack_segment]
pub fn mvwinch(w: WINDOW_p, y: i32, x: i32) -> u32
{ unsafe { ll::mvwinch(w, y, x) } }

#[fixed_stack_segment]
pub fn mvwinchnstr(w: WINDOW_p, y: i32, x: i32, s: &mut ~[u32], n: i32) -> i32
{
  /* XXX: This is probably broken. */
  s.clear();
  s.reserve_at_least(n as uint);
  unsafe
  {
    let ret = ll::mvwinchnstr(w, y, x, vec::raw::to_ptr(*s), n);

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => vec::raw::set_len(s, index as uint),
      None => vec::raw::set_len(s, capacity),
    }

    ret
  }
}

#[fixed_stack_segment]
pub fn mvwinchstr(w: WINDOW_p, y: i32, x: i32, s: &mut ~[u32]) -> i32
{
  /* XXX: This is probably broken. */
  unsafe
  {
    let ret = ll::mvwinchstr(w, y, x, vec::raw::to_ptr(*s));

    let capacity = s.capacity();
    match s.iter().position(|x| *x == 0)
    {
      Some(index) => vec::raw::set_len(s, index as uint),
      None => vec::raw::set_len(s, capacity),
    }

    ret
  }
}

#[fixed_stack_segment]
pub fn mvwinnstr(w: WINDOW_p, y: i32, x: i32, s: &mut ~str, n: i32) -> i32
{
  use std::cast; 

  /* XXX: This is probably broken. */
  s.clear();
  s.reserve_at_least(n as uint);
  unsafe
  {
    let ret = do s.as_mut_buf |buf, _len|
    { ll::mvwinnstr(w, y, x, cast::transmute(buf), n) };

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => str::raw::set_len(s, index as uint),
      None => str::raw::set_len(s, capacity),
    }

    ret
  }
}

#[fixed_stack_segment]
pub fn mvwinsch(w: WINDOW_p, y: i32, x: i32, ch: u32) -> i32
{ unsafe { ll::mvwinsch(w, y, x, ch) } }

#[fixed_stack_segment]
pub fn mvwinsnstr(w: WINDOW_p, y: i32, x: i32, s: &str, n: i32) -> i32
{
  unsafe
  {
    do s.to_c_str().with_ref() |c_str|
    { ll::mvwinsnstr(w, y, x, c_str, n) }
  }
}

#[fixed_stack_segment]
pub fn mvwinsstr(w: WINDOW_p, y: i32, x: i32, s: &str) -> i32
{
  unsafe
  {
    do s.to_c_str().with_ref() |c_str|
    { ll::mvwinsstr(w, y, x, c_str) }
  }
}

#[fixed_stack_segment]
pub fn mvwinstr(w: WINDOW_p, y: i32, x: i32, s: &mut ~str) -> i32
{
  use std::cast; 

  /* XXX: This is probably broken. */
  unsafe
  {
    let ret = do s.as_mut_buf |buf, _len|
    { ll::mvwinstr(w, y, x, cast::transmute(buf)) };

    let capacity = s.capacity();
    match s.find('\0')
    {
      Some(index) => str::raw::set_len(s, index as uint),
      None => str::raw::set_len(s, capacity),
    }

    ret
  }
}

#[fixed_stack_segment]
pub fn mvwprintw(w: WINDOW_p, y: i32, x: i32, s: &str) -> i32
{
  unsafe
  {
    do s.to_c_str().with_ref() |c_str|
    { ll::mvwprintw(w, y, x, c_str) }
  }
}

#[fixed_stack_segment]
pub fn mvwvline(w: WINDOW_p, y: i32, x: i32, ch: u32, n: i32) -> i32
{ unsafe { ll::mvwvline(w, y, x, ch, n) } }

#[fixed_stack_segment]
pub fn napms(ms: i32) -> i32
{ unsafe { ll::napms(ms) } }

#[fixed_stack_segment]
pub fn newpad(lines: i32, cols: i32) -> WINDOW_p
{ unsafe { ll::newpad(lines, cols) } }

#[fixed_stack_segment]
pub fn newterm(ty: &str, out_fd: FILE_p, in_fd: FILE_p) -> SCREEN_p
{
  unsafe
  {
    do ty.to_c_str().with_ref() |c_str|
    { ll::newterm(c_str, out_fd, in_fd) }
  }
}

#[fixed_stack_segment]
pub fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> WINDOW_p
{ unsafe { ll::newwin(lines, cols, y, x) } }

#[fixed_stack_segment]
pub fn nl() -> i32
{ unsafe { ll::nl() } }

#[fixed_stack_segment]
pub fn nocbreak() -> i32
{ unsafe { ll::nocbreak() } }

#[fixed_stack_segment]
pub fn nodelay(w: WINDOW_p, bf: bool) -> i32
{ unsafe { ll::nodelay(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn noecho() -> i32
{ unsafe { ll::noecho() } }

#[fixed_stack_segment]
pub fn nonl() -> i32
{ unsafe { ll::nonl() } }

#[fixed_stack_segment]
pub fn noqiflush()
{ unsafe { ll::noqiflush() } }

#[fixed_stack_segment]
pub fn noraw() -> i32
{ unsafe { ll::noraw() } }

#[fixed_stack_segment]
pub fn notimeout(w: WINDOW_p, bf: bool) -> i32
{ unsafe { ll::notimeout(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn overlay(src: WINDOW_p, dst: WINDOW_p) -> i32
{ unsafe { ll::overlay(src, dst) } }

#[fixed_stack_segment]
pub fn overwrite(src: WINDOW_p, dst: WINDOW_p) -> i32
{ unsafe { ll::overwrite(src, dst) } }

#[fixed_stack_segment]
pub fn pair_content(pair: i16, f: &mut i16, b: &mut i16) -> i32
{ unsafe { ll::pair_content(pair, ptr::to_unsafe_ptr(f), ptr::to_unsafe_ptr(b)) } }

#[fixed_stack_segment]
pub fn PAIR_NUMBER(attr: i32) -> i32
{ unsafe { ll::PAIR_NUMBER(attr) } }

#[fixed_stack_segment]
pub fn pechochar(pad: WINDOW_p, ch: u32) -> i32
{ unsafe { ll::pechochar(pad, ch) } }

#[fixed_stack_segment]
pub fn pnoutrefresh(pad: WINDOW_p, pmin_row: i32, pmin_col: i32, smin_row: i32, smin_col: i32, smax_row: i32, smax_col: i32) -> i32
{ unsafe { ll::pnoutrefresh(pad, pmin_row, pmin_col, smin_row, smin_col, smax_row, smax_col) } }

#[fixed_stack_segment]
pub fn prefresh(pad: WINDOW_p, pmin_row: i32, pmin_col: i32, smin_row: i32, smin_col: i32, smax_row: i32, smax_col: i32) -> i32
{ unsafe { ll::prefresh(pad, pmin_row, pmin_col, smin_row, smin_col, smax_row, smax_col) } }

#[fixed_stack_segment]
pub fn printw(s: &str) -> i32
{
  unsafe
  {
    do s.to_c_str().with_ref() |c_str|
    { ll::printw(c_str) }
  }
}

#[fixed_stack_segment]
pub fn putwin(w: WINDOW_p, f: FILE_p) -> i32
{ unsafe { ll::putwin(w, f) } }

#[fixed_stack_segment]
pub fn qiflush()
{ unsafe { ll::qiflush() } }

#[fixed_stack_segment]
pub fn raw() -> i32
{ unsafe { ll::raw() } }

#[fixed_stack_segment]
pub fn redrawwin(w: WINDOW_p) -> i32
{ unsafe { ll::redrawwin(w) } }

#[fixed_stack_segment]
pub fn refresh() -> i32
{ unsafe { ll::refresh() } }

#[fixed_stack_segment]
pub fn resetty() -> i32
{ unsafe { ll::resetty() } }

#[fixed_stack_segment]
pub fn reset_prog_mode() -> i32
{ unsafe { ll::reset_prog_mode() } }

#[fixed_stack_segment]
pub fn reset_shell_mode() -> i32
{ unsafe { ll::reset_shell_mode() } }

#[fixed_stack_segment]
pub fn savetty() -> i32
{ unsafe { ll::savetty() } }

#[fixed_stack_segment]
pub fn scr_dump(filename: &str) -> i32
{
  unsafe
  {
    do filename.to_c_str().with_ref() |c_str|
    { ll::scr_dump(c_str) }
  }
}

#[fixed_stack_segment]
pub fn scr_init(filename: &str) -> i32
{
  unsafe
  {
    do filename.to_c_str().with_ref() |c_str|
    { ll::scr_init(c_str) }
  }
}

#[fixed_stack_segment]
pub fn scrl(n: i32) -> i32
{ unsafe { ll::scrl(n) } }

#[fixed_stack_segment]
pub fn scroll(w: WINDOW_p) -> i32
{ unsafe { ll::scroll(w) } }

#[fixed_stack_segment]
pub fn scrollok(w: WINDOW_p, bf: bool) -> i32
{ unsafe { ll::scrollok(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn scr_restore(filename: &str) -> i32
{
  unsafe
  {
    do filename.to_c_str().with_ref() |c_str|
    { ll::scr_restore(c_str) }
  }
}

#[fixed_stack_segment]
pub fn scr_set(filename: &str) -> i32
{
  unsafe
  {
    do filename.to_c_str().with_ref() |c_str|
    { ll::scr_set(c_str) }
  }
}

#[fixed_stack_segment]
pub fn setscrreg(top: i32, bot: i32) -> i32
{ unsafe { ll::setscrreg(top, bot) } }

#[fixed_stack_segment]
pub fn set_term(s: SCREEN_p) -> SCREEN_p
{ unsafe { ll::set_term(s) } }

#[fixed_stack_segment]
pub fn slk_attroff(ch: u32) -> i32
{ unsafe { ll::slk_attroff(ch) } }

#[fixed_stack_segment]
pub fn slk_attr_off(ch: i32) -> i32
{ unsafe { ll::slk_attr_off(ch, ptr::null()) } }

#[fixed_stack_segment]
pub fn slk_attron(ch: u32) -> i32
{ unsafe { ll::slk_attron(ch) } }

#[fixed_stack_segment]
pub fn slk_attr_on(ch: i32) -> i32
{ unsafe { ll::slk_attr_on(ch, ptr::null()) } }

#[fixed_stack_segment]
pub fn slk_attrset(ch: u32) -> i32
{ unsafe { ll::slk_attrset(ch) } }

#[fixed_stack_segment]
pub fn slk_attr() -> i32
{ unsafe { ll::slk_attr() } }

#[fixed_stack_segment]
pub fn slk_attr_set(attrs: i32, pair: i16) -> i32
{ unsafe { ll::slk_attr_set(attrs, pair, ptr::null()) } }

#[fixed_stack_segment]
pub fn slk_clear() -> i32
{ unsafe { ll::slk_clear() } }

#[fixed_stack_segment]
pub fn slk_color(pair: i16) -> i32
{ unsafe { ll::slk_color(pair) } }

#[fixed_stack_segment]
pub fn slk_init(fmt: i32) -> i32
{ unsafe { ll::slk_init(fmt) } }

#[fixed_stack_segment]
pub fn slk_label(n: i32) -> ~str
{ unsafe { str::raw::from_c_str(ll::slk_label(n)) } }

#[fixed_stack_segment]
pub fn slk_noutrefresh() -> i32
{ unsafe { ll::slk_noutrefresh() } }

#[fixed_stack_segment]
pub fn slk_refresh() -> i32
{ unsafe { ll::slk_refresh() } }

#[fixed_stack_segment]
pub fn slk_restore() -> i32
{ unsafe { ll::slk_restore() } }

#[fixed_stack_segment]
pub fn slk_set(n: i32, s: &str, fmt: i32) -> i32
{
  unsafe
  {
    do s.to_c_str().with_ref() |c_str|
    { ll::slk_set(n, c_str, fmt) }
  }
}

#[fixed_stack_segment]
pub fn slk_touch() -> i32
{ unsafe { ll::slk_touch() }}

#[fixed_stack_segment]
pub fn standout() -> i32
{ unsafe { ll::standout() } }

#[fixed_stack_segment]
pub fn standend() -> i32
{ unsafe { ll::standend() } }

#[fixed_stack_segment]
pub fn start_color() -> i32
{ unsafe { ll::start_color() } }

#[fixed_stack_segment]
pub fn subpad(w: WINDOW_p, lines: i32, cols: i32, y: i32, x: i32) -> WINDOW_p
{ unsafe { ll::subpad(w, lines, cols, y, x) } }

#[fixed_stack_segment]
pub fn subwin(w: WINDOW_p, lines: i32, cols: i32, y: i32, x: i32) -> WINDOW_p
{ unsafe { ll::subwin(w, lines, cols, y, x) } }

#[fixed_stack_segment]
pub fn syncok(w: WINDOW_p, bf: bool) -> i32
{ unsafe { ll::syncok(w, bf as libc::c_int) } }

#[fixed_stack_segment]
pub fn termattrs() -> u32
{ unsafe { ll::termattrs() } }

#[fixed_stack_segment]
pub fn termname() -> ~str
{ unsafe { str::raw::from_c_str(ll::termname()) } }

#[fixed_stack_segment]
pub fn timeout(delay: i32)
{ unsafe { ll::timeout(delay) } }

#[fixed_stack_segment]
pub fn touchline(w: WINDOW_p, start: i32, count: i32) -> i32
{ unsafe { ll::touchline(w, start, count) } }

#[fixed_stack_segment]
pub fn touchwin(w: WINDOW_p) -> i32
{ unsafe { ll::touchwin(w) } }

#[fixed_stack_segment]
pub fn typeahead(fd: i32) -> i32
{ unsafe { ll::typeahead(fd) } }

#[fixed_stack_segment]
pub fn ungetch(ch: i32) -> i32
{ unsafe { ll::ungetch(ch) } }

#[fixed_stack_segment]
pub fn untouchwin(w: WINDOW_p) -> i32
{ unsafe { ll::untouchwin(w) } }

#[fixed_stack_segment]
pub fn use_env(f: bool)
{ unsafe { ll::use_env(f as libc::c_int) } }

#[fixed_stack_segment]
pub fn vidattr(attrs: u32) -> i32
{ unsafe { ll::vidattr(attrs) } }

#[fixed_stack_segment]
pub fn vline(ch: u32, n: i32) -> i32
{ unsafe { ll::vline(ch, n) } }

#[fixed_stack_segment]
pub fn waddch(w: WINDOW_p, ch: u32) -> i32
{ unsafe { ll::waddch(w, ch) } }

#[fixed_stack_segment]
pub fn waddchnstr(_: WINDOW_p, _: chtype_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn waddchstr(_: WINDOW_p, _: chtype_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn waddnstr(_: WINDOW_p, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn waddstr(_: WINDOW_p, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wattron(_: WINDOW_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wattroff(_: WINDOW_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wattrset(_: WINDOW_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wattr_get(_: WINDOW_p, _: *i32, _: *i16, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wattr_on(_: WINDOW_p, _: i32, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wattr_off(_: WINDOW_p, _: i32, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wattr_set(_: WINDOW_p, _: i32, _: i16, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wbkgd(_: WINDOW_p, _: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wbkgdset(_: WINDOW_p, _: u32)
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wborder(_: WINDOW_p, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wchgat(_: WINDOW_p, _: i32, _: i32, _: i16, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wclear(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wclrtobot(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wclrtoeol(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wcolor_set(_: WINDOW_p, _: i16, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wcursyncup(_: WINDOW_p)
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wdelch(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wdeleteln(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wechochar(_: WINDOW_p, _: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn werase(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wgetch(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wgetnstr(_: WINDOW_p, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wgetstr(_: WINDOW_p, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn whline(_: WINDOW_p, _: u32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn winch(_: WINDOW_p) -> u32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn winchnstr(_: WINDOW_p, _: chtype_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn winchstr(_: WINDOW_p, _: chtype_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn winnstr(_: WINDOW_p, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn winsch(_: WINDOW_p, _: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn winsdelln(_: WINDOW_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn winsertln(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn winsnstr(_: WINDOW_p, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn winsstr(_: WINDOW_p, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn winstr(_: WINDOW_p, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wmove(_: WINDOW_p, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wnoutrefresh(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wprintw(_: WINDOW_p, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wredrawln(_: WINDOW_p, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wrefresh(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wscrl(_: WINDOW_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wsetscrreg(_: WINDOW_p, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wstandout(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wstandend(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wsyncdown(_: WINDOW_p)
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wsyncup(_: WINDOW_p)
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wtimeout(_: WINDOW_p, _: i32)
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wtouchln(_: WINDOW_p, _: i32, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wvline(_: WINDOW_p, _: u32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn tigetflag(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn tigetnum(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn tigetstr(_: char_p) -> ~str
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn putp(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn tparm(_: char_p) -> ~str
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getattrs(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getcurx(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getcury(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getbegx(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getbegy(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getmaxx(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getmaxy(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getparx(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getpary(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wgetparent(_: WINDOW_p) -> WINDOW_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn wgetscrreg(_: WINDOW_p, _: *i32, _: *i32) -> i32
{ fail!("Not implemented"); }

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
pub fn COLOR_PAIR(n: u32) -> i32
{ NCURSES_BITS(n, 0u32) as i32 }

/*
 * Most of the pseudo functions are macros that either provide compatibility
 * with older versions of curses, or provide inline functionality to improve
 * performance.
 */
#[fixed_stack_segment]
pub fn getyx(win: WINDOW_p, y: &mut i32, x: &mut i32)
{ unsafe { *y = ll::getcury(win); *x = ll::getcurx(win); } }

#[fixed_stack_segment]
pub fn getbegyx(win: WINDOW_p, y: &mut i32, x: &mut i32)
{ unsafe { *y = ll::getbegy(win); *x = ll::getbegx(win) } }

#[fixed_stack_segment]
pub fn getmaxyx(win: WINDOW_p, y: &mut i32, x: &mut i32)
{ unsafe { *y = ll::getmaxy(win); *x = ll::getmaxx(win) } }

#[fixed_stack_segment]
pub fn getparyx(win: WINDOW_p, y: &mut i32, x: &mut i32)
{ unsafe { *y = ll::getpary(win); *x = ll::getparx(win) } }

#[fixed_stack_segment]
pub fn getsyx(y: &mut i32, x: &mut i32)
{
  unsafe
  {
    if newscr != ptr::null()
    {
      if ll::is_leaveok(newscr) == OK
      {
        *x = -1 as i32;
        *y = -1 as i32;
      }
      else
      { getyx(newscr, (y), (x)); }
    }
  }
}

#[fixed_stack_segment]
pub fn setsyx(y: &mut i32, x: &mut i32)
{
  unsafe 
  {
    if newscr !=(0 as WINDOW_p)
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

