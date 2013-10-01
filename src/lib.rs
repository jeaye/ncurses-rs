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

use std::{ vec, char, ptr };
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
{ unsafe { ll::can_change_color() } }

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
{ unsafe { ll::clearok(w, ok) } }

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
pub fn getbkgd(_: WINDOW_p) -> u32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getch() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getnstr(_: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getstr(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn getwin(_: FILE_p) -> WINDOW_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn halfdelay(_: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn has_colors() -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn has_ic() -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn has_il() -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn hline(_: u32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn idcok(_: WINDOW_p, _: bool)
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn idlok(_: WINDOW_p, _: bool) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn immedok(_: WINDOW_p, _: bool)
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn inch() -> u32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn inchnstr(_: chtype_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn inchstr(_: chtype_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn initscr() -> WINDOW_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn init_color(_: i16, _: i16, _: i16, _: i16) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn init_pair(_: i16, _: i16, _: i16) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn innstr(_: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn insch(_: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn insdelln(_: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn insertln() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn insnstr(_: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn insstr(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn instr(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn intrflush(_: WINDOW_p, _: bool) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn isendwin() -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_linetouched(_: WINDOW_p, _: i32) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_wintouched(_: WINDOW_p) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn keyname(_: i32) -> ~str
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn keypad(_: WINDOW_p, _: bool) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn killchar() -> char
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn leaveok(_: WINDOW_p, _: bool) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn longname() -> ~str
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn meta(_: WINDOW_p, _: bool) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn move(_: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvaddch(_: i32, _: i32, _: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvaddchnstr(_: i32, _: i32, _: chtype_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvaddchstr(_: i32, _: i32, _: chtype_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvaddnstr(_: i32, _: i32, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvaddstr(_: i32, _: i32, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvchgat(_: i32, _: i32, _: i32, _: i32, _: i16, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvcur(_: i32, _: i32, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvdelch(_: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvderwin(_: WINDOW_p, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvgetch(_: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvgetnstr(_: i32, _: i32, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvgetstr(_: i32, _: i32, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvhline(_: i32, _: i32, _: u32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvinch(_: i32, _: i32) -> u32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvinchnstr(_: i32, _: i32, _: chtype_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvinchstr(_: i32, _: i32, _: chtype_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvinnstr(_: i32, _: i32, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvinsch(_: i32, _: i32, _: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvinsnstr(_: i32, _: i32, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvinsstr(_: i32, _: i32, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvinstr(_: i32, _: i32, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvprintw(_: i32, _: i32, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvvline(_: i32, _: i32, _: u32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwaddch(_: WINDOW_p, _: i32, _: i32, _: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwaddchnstr(_: WINDOW_p, _: i32, _: i32, _: chtype_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwaddchstr(_: WINDOW_p, _: i32, _: i32, _: chtype_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwaddnstr(_: WINDOW_p, _: i32, _: i32, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwaddstr(_: WINDOW_p, _: i32, _: i32, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwchgat(_: WINDOW_p, _: i32, _: i32, _: i32, _: i32, _: i16, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwdelch(_: WINDOW_p, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwgetch(_: WINDOW_p, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwgetnstr(_: WINDOW_p, _: i32, _: i32, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwgetstr(_: WINDOW_p, _: i32, _: i32, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwhline(_: WINDOW_p, _: i32, _: i32, _: u32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwin(_: WINDOW_p, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwinch(_: WINDOW_p, _: i32, _: i32) -> u32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwinchnstr(_: WINDOW_p, _: i32, _: i32, _: chtype_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwinchstr(_: WINDOW_p, _: i32, _: i32, _: chtype_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwinnstr(_: WINDOW_p, _: i32, _: i32, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwinsch(_: WINDOW_p, _: i32, _: i32, _: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwinsnstr(_: WINDOW_p, _: i32, _: i32, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwinsstr(_: WINDOW_p, _: i32, _: i32, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwinstr(_: WINDOW_p, _: i32, _: i32, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwprintw(_: WINDOW_p, _: i32, _: i32, _: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn mvwvline(_: WINDOW_p, _: i32, _: i32, _: u32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn napms(_: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn newpad(_: i32, _: i32) -> WINDOW_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn newterm(_: char_p, _: FILE_p, _: FILE_p) -> SCREEN_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn newwin(_: i32, _: i32, _: i32, _: i32) -> WINDOW_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn nl() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn nocbreak() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn nodelay(_: WINDOW_p, _: bool) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn noecho() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn nonl() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn noqiflush()
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn noraw() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn notimeout(_: WINDOW_p, _: bool) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn overlay(_: WINDOW_p, _: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn overwrite(_: WINDOW_p, _: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn pair_content(_: i16, _: *i16, _: *i16) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn PAIR_NUMBER(_: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn pechochar(_: WINDOW_p, _: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn pnoutrefresh(_: WINDOW_p, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn prefresh(_: WINDOW_p, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn printw(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn putwin(_: WINDOW_p, _: FILE_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn qiflush()
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn raw() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn redrawwin(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn refresh() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn resetty() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn reset_prog_mode() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn reset_shell_mode() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn ripoffline(_: i32, _: *u8) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn savetty() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn scr_dump(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn scr_init(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn scrl(_: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn scroll(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn scrollok(_: WINDOW_p, _: bool) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn scr_restore(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn scr_set(_: char_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn setscrreg(_: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn set_term(_: SCREEN_p) -> SCREEN_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_attroff(_: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_attr_off(_: i32, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_attron(_: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_attr_on(_: i32, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_attrset(_: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_attr() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_attr_set(_: i32, _: i16, _: *c_void) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_clear() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_color(_: i16) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_init(_: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_label(_: i32) -> char_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_noutrefresh() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_refresh() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_restore() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_set(_: i32, _: char_p, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn slk_touch() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn standout() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn standend() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn start_color() -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn subpad(_: WINDOW_p, _: i32, _: i32, _: i32, _: i32) -> WINDOW_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn subwin(_: WINDOW_p, _: i32, _: i32, _: i32, _: i32) -> WINDOW_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn syncok(_: WINDOW_p, _: bool) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn termattrs() -> u32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn termname() -> char_p
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn timeout(_: i32)
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn touchline(_: WINDOW_p, _: i32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn touchwin(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn typeahead(_: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn ungetch(_: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn untouchwin(_: WINDOW_p) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn use_env(_: bool)
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn vidattr(_: u32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn vidputs(_: u32, _: *u8) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn vline(_: u32, _: i32) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn vwprintw(_: WINDOW_p, _: char_p, _: va_list) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn vw_printw(_: WINDOW_p, _: char_p, _: va_list) -> i32
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn waddch(_: WINDOW_p, _: u32) -> i32
{ fail!("Not implemented"); }

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
pub fn is_cleared(_: WINDOW_p) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_idcok(_: WINDOW_p) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_idlok(_: WINDOW_p) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_immedok(_: WINDOW_p) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_keypad(_: WINDOW_p) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_leaveok(_: WINDOW_p) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_nodelay(_: WINDOW_p) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_notimeout(_: WINDOW_p) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_scrollok(_: WINDOW_p) -> bool
{ fail!("Not implemented"); }

#[fixed_stack_segment]
pub fn is_syncok(_: WINDOW_p) -> bool
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
      if ll::is_leaveok(newscr)
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
        ll::leaveok(newscr, true);
      }
      else
      {
        ll::leaveok(newscr, false);
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

