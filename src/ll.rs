/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: ll.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Low-level interface to foreign
      ncurses functions.
*/

use std::libc::{ c_char, c_int, c_short, c_uint, c_void, FILE };

/* Intrinsic types. */
pub type chtype = c_uint;
pub type attr_t = c_int;
pub type NCURSES_ATTR_T = attr_t;

/* Pointer types. */
pub type attr_t_p = *attr_t;
pub type short_p = *c_short;
pub type void_p = *c_void;
pub type char_p = *c_char;
pub type chtype_p = *chtype;
pub type WINDOW_p = *WINDOW;
pub type SCREEN_p = *SCREEN;
pub type FILE_p = *FILE;
pub type va_list = *u8;

/* Custom Types. */
pub struct WINDOW;
pub struct SCREEN;

#[link_args = "-lncurses"]
extern
{
  pub fn addch(_:chtype) -> c_int;
  pub fn addchnstr(_:*chtype, _:c_int) -> c_int;
  pub fn addchstr(_:*chtype) -> c_int;
  pub fn addnstr(_:*c_char, _:c_int) -> c_int;
  pub fn addstr(_:*c_char) -> c_int;
  pub fn attroff(_:NCURSES_ATTR_T) -> c_int;
  pub fn attron(_:NCURSES_ATTR_T) -> c_int;
  pub fn attrset(_:NCURSES_ATTR_T) -> c_int;
  pub fn attr_get(_:attr_t_p, _:short_p, _:void_p) -> c_int;
  pub fn attr_off(_:attr_t, _:void_p) -> c_int;
  pub fn attr_on(_:attr_t, _:void_p) -> c_int;
  pub fn attr_set(_:attr_t, _:c_short, _:void_p) -> c_int;
  pub fn baudrate() -> c_int;
  pub fn beep() -> c_int;
  pub fn bkgd(_:chtype) -> c_int;
  pub fn bkgdset(_:chtype);
  pub fn border(_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype) -> c_int;
  pub fn box(_:WINDOW_p, _:chtype, _:chtype) -> c_int;
  pub fn can_change_color() -> c_int;
  pub fn cbreak() -> c_int;
  pub fn chgat(_:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
  pub fn clear() -> c_int;
  pub fn clearok(_:WINDOW_p,_:c_int) -> c_int;
  pub fn clrtobot() -> c_int;
  pub fn clrtoeol() -> c_int;
  pub fn color_content(_:c_short,_:short_p,_:short_p,_:short_p) -> c_int;
  pub fn color_set(_:c_short,_:void_p) -> c_int;
  pub fn COLOR_PAIR(_:c_int) -> c_int;
  pub fn copywin(_:WINDOW_p,_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
  pub fn curs_set(_:c_int) -> c_int;
  pub fn def_prog_mode() -> c_int;
  pub fn def_shell_mode() -> c_int;
  pub fn delay_output(_:c_int) -> c_int;
  pub fn delch() -> c_int;
  pub fn delscreen(_:SCREEN_p);
  pub fn delwin(_:WINDOW_p) -> c_int;
  pub fn deleteln() -> c_int;
  pub fn derwin(_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int) -> *WINDOW;
  pub fn doupdate() -> c_int;
  pub fn dupwin(_:WINDOW_p) -> *WINDOW;
  pub fn echo() -> c_int;
  pub fn echochar(_:chtype) -> c_int;
  pub fn erase() -> c_int;
  pub fn endwin() -> c_int;
  pub fn erasechar() -> c_char;
  pub fn filter();
  pub fn flash() -> c_int;
  pub fn flushinp() -> c_int;
  pub fn getbkgd(_:WINDOW_p) -> chtype;
  pub fn getch() -> c_int;
  pub fn getnstr(_:*mut c_char, _:c_int) -> c_int;
  pub fn getstr(_:char_p) -> c_int;
  pub fn getwin(_:FILE_p) -> *WINDOW;
  pub fn halfdelay(_:c_int) -> c_int;
  pub fn has_colors() -> c_int;
  pub fn has_ic() -> c_int;
  pub fn has_il() -> c_int;
  pub fn hline(_:chtype, _:c_int) -> c_int;
  pub fn idcok(_:WINDOW_p, _:c_int);
  pub fn idlok(_:WINDOW_p, _:c_int) -> c_int;
  pub fn immedok(_:WINDOW_p, _:c_int);
  pub fn inch() -> chtype;
  pub fn inchnstr(_:chtype_p, _:c_int) -> c_int;
  pub fn inchstr(_:chtype_p) -> c_int;
  pub fn initscr() -> *WINDOW;
  pub fn init_color(_:c_short,_:c_short,_:c_short,_:c_short) -> c_int;
  pub fn init_pair(_:c_short,_:c_short,_:c_short) -> c_int;
  pub fn innstr(_:char_p, _:c_int) -> c_int;
  pub fn insch(_:chtype) -> c_int;
  pub fn insdelln(_:c_int) -> c_int;
  pub fn insertln() -> c_int;
  pub fn insnstr(_:char_p, _:c_int) -> c_int;
  pub fn insstr(_:char_p) -> c_int;
  pub fn instr(_:char_p) -> c_int;
  pub fn intrflush(_:WINDOW_p,_:c_int) -> c_int;
  pub fn isendwin() -> c_int;
  pub fn is_linetouched(_:WINDOW_p,_:c_int) -> c_int;
  pub fn is_wintouched(_:WINDOW_p) -> c_int;
  pub fn keyname(_:c_int) -> *c_char;
  pub fn keypad(_:WINDOW_p, _:c_int) -> c_int;
  pub fn killchar() -> c_char;
  pub fn leaveok(_:WINDOW_p,_:c_int) -> c_int;
  pub fn longname() -> *c_char;
  pub fn meta(_:WINDOW_p,_:c_int) -> c_int;
  pub fn move(_:c_int, _:c_int) -> c_int;
  pub fn mvaddch(_:c_int, _:c_int, _:chtype) -> c_int;
  pub fn mvaddchnstr(_:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
  pub fn mvaddchstr(_:c_int, _:c_int, _:chtype_p) -> c_int;
  pub fn mvaddnstr(_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
  pub fn mvaddstr(_:c_int, _:c_int, _:char_p) -> c_int;
  pub fn mvchgat(_:c_int, _:c_int, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
  pub fn mvcur(_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
  pub fn mvdelch(_:c_int, _:c_int) -> c_int;
  pub fn mvderwin(_:WINDOW_p, _:c_int, _:c_int) -> c_int;
  pub fn mvgetch(_:c_int, _:c_int) -> c_int;
  pub fn mvgetnstr(_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
  pub fn mvgetstr(_:c_int, _:c_int, _:char_p) -> c_int;
  pub fn mvhline(_:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
  pub fn mvinch(_:c_int, _:c_int) -> chtype;
  pub fn mvinchnstr(_:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
  pub fn mvinchstr(_:c_int, _:c_int, _:chtype_p) -> c_int;
  pub fn mvinnstr(_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
  pub fn mvinsch(_:c_int, _:c_int, _:chtype) -> c_int;
  pub fn mvinsnstr(_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
  pub fn mvinsstr(_:c_int, _:c_int, _:char_p) -> c_int;
  pub fn mvinstr(_:c_int, _:c_int, _:char_p) -> c_int;
  pub fn mvprintw(_:c_int, _:c_int, _:char_p) -> c_int;
  //  fn mvscanw(_:c_int,_:c_int, _:char_p) -> c_int;
  pub fn mvvline(_:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
  pub fn mvwaddch(_:WINDOW_p, _:c_int, _:c_int, _:chtype) -> c_int;
  pub fn mvwaddchnstr(_:WINDOW_p, _:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
  pub fn mvwaddchstr(_:WINDOW_p, _:c_int, _:c_int, _:chtype_p) -> c_int;
  pub fn mvwaddnstr(_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
  pub fn mvwaddstr(_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
  pub fn mvwchgat(_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
  pub fn mvwdelch(_:WINDOW_p, _:c_int, _:c_int) -> c_int;
  pub fn mvwgetch(_:WINDOW_p, _:c_int, _:c_int) -> c_int;
  pub fn mvwgetnstr(_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
  pub fn mvwgetstr(_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
  pub fn mvwhline(_:WINDOW_p, _:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
  pub fn mvwin(_:WINDOW_p,_:c_int,_:c_int) -> c_int;
  pub fn mvwinch(_:WINDOW_p, _:c_int, _:c_int) -> chtype;
  pub fn mvwinchnstr(_:WINDOW_p, _:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
  pub fn mvwinchstr(_:WINDOW_p, _:c_int, _:c_int, _:chtype_p) -> c_int;
  pub fn mvwinnstr(_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
  pub fn mvwinsch(_:WINDOW_p, _:c_int, _:c_int, _:chtype) -> c_int;
  pub fn mvwinsnstr(_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
  pub fn mvwinsstr(_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
  pub fn mvwinstr(_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
  pub fn mvwprintw(_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;

  //  fn mvwscanw(_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
  pub fn mvwvline(_:WINDOW_p, _:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
  pub fn napms(_:c_int) -> c_int;
  pub fn newpad(_:c_int,_:c_int) -> *WINDOW;
  pub fn newterm(_:char_p,_:FILE_p,_:FILE_p) -> *SCREEN;
  pub fn newwin(_:c_int,_:c_int,_:c_int,_:c_int) -> *WINDOW;
  pub fn nl() -> c_int;
  pub fn nocbreak() -> c_int;
  pub fn nodelay(_:WINDOW_p,_:c_int) -> c_int;
  pub fn noecho() -> c_int;
  pub fn nonl() -> c_int;
  pub fn noqiflush();
  pub fn noraw() -> c_int;
  pub fn notimeout(_:WINDOW_p,_:c_int) -> c_int;
  pub fn overlay(_:WINDOW_p,_:WINDOW_p) -> c_int;
  pub fn overwrite(_:WINDOW_p,_:WINDOW_p) -> c_int;
  pub fn pair_content(_:c_short,_:short_p,_:short_p) -> c_int;
  pub fn PAIR_NUMBER(_:c_int) -> c_int;
  pub fn pechochar(_:WINDOW_p, _:chtype) -> c_int;
  pub fn pnoutrefresh(_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
  pub fn prefresh(_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;

  pub fn printw(_:char_p) -> c_int;
  pub fn putwin(_:WINDOW_p, _:FILE_p) -> c_int;
  pub fn qiflush();
  pub fn raw() -> c_int;
  pub fn redrawwin(_:WINDOW_p) -> c_int;
  pub fn refresh() -> c_int;
  pub fn resetty() -> c_int;
  pub fn reset_prog_mode() -> c_int;
  pub fn reset_shell_mode() -> c_int;
  // fn ripoffline(_:c_int, extern  fn f(WINDOW_p, c_int) -> c_int) -> c_int;
  pub fn savetty() -> c_int;
  // fn scanw(_:NCURSES_CONST char_p,...) -> c_int;
  pub fn scr_dump(_:char_p) -> c_int;
  pub fn scr_init(_:char_p) -> c_int;
  pub fn scrl(_:c_int) -> c_int;
  pub fn scroll(_:WINDOW_p) -> c_int;
  pub fn scrollok(_:WINDOW_p,_:c_int) -> c_int;
  pub fn scr_restore(_:char_p) -> c_int;
  pub fn scr_set(_:char_p) -> c_int;
  pub fn setscrreg(_:c_int,_:c_int) -> c_int;
  pub fn set_term(_:SCREEN_p) -> SCREEN_p;
  pub fn slk_attroff(_:chtype) -> c_int;
  pub fn slk_attr_off(_:attr_t, _:void_p) -> c_int;
  pub fn slk_attron(_:chtype) -> c_int;
  pub fn slk_attr_on(_:attr_t,_:void_p) -> c_int;
  pub fn slk_attrset(_:chtype) -> c_int;
  pub fn slk_attr() -> attr_t;
  pub fn slk_attr_set(_:attr_t,_:c_short,_:void_p) -> c_int;
  pub fn slk_clear() -> c_int;
  pub fn slk_color(_:c_short) -> c_int;
  pub fn slk_init(_:c_int) -> c_int;
  pub fn slk_label(_:c_int) -> char_p; 
  pub fn slk_noutrefresh() -> c_int;
  pub fn slk_refresh() -> c_int;
  pub fn slk_restore() -> c_int;
  pub fn slk_set(_:c_int,_:char_p,_:c_int) -> c_int;
  pub fn slk_touch() -> c_int;
  pub fn standout() -> c_int;
  pub fn standend() -> c_int;
  pub fn start_color() -> c_int;
  pub fn subpad(_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:c_int) -> WINDOW_p;
  pub fn subwin(_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:c_int) -> WINDOW_p;
  pub fn syncok(_:WINDOW_p, _:c_int) -> c_int;
  pub fn termattrs() -> chtype;
  pub fn termname() -> char_p;
  pub fn timeout(_:c_int);
  pub fn touchline(_:WINDOW_p, _:c_int, _:c_int) -> c_int;
  pub fn touchwin(_:WINDOW_p) -> c_int;
  pub fn typeahead(_:c_int) -> c_int;
  pub fn ungetch(_:c_int) -> c_int;
  pub fn untouchwin(_:WINDOW_p) -> c_int;
  pub fn use_env(_:c_int);
  pub fn vidattr(_:chtype) -> c_int;
  //  fn vidputs(_:chtype, extern  fn f(c_int) -> c_int) -> c_int;
  //pub fn vidputs(_:chtype, f:*c_char) -> c_int;
  pub fn vline(_:chtype, _:c_int) -> c_int;
  pub fn vwprintw(_:WINDOW_p, _:char_p, _:va_list) -> c_int;
  pub fn vw_printw(_:WINDOW_p, _:char_p,_:va_list) -> c_int;
  //  fn vwscanw(_:WINDOW_p, _:char_p, _:va_list) -> c_int;
  //  fn vw_scanw(_:WINDOW_p, _:char_p, _:va_list) -> c_int;
  pub fn waddch(_:WINDOW_p, _:chtype) -> c_int;
  pub fn waddchnstr(_:WINDOW_p,_:chtype_p,_:c_int) -> c_int;
  pub fn waddchstr(_:WINDOW_p,_:chtype_p) -> c_int;
  pub fn waddnstr(_:WINDOW_p,_:char_p,_:c_int) -> c_int;
  pub fn waddstr(_:WINDOW_p,_:char_p) -> c_int;
  pub fn wattron(_:WINDOW_p, _:c_int) -> c_int;
  pub fn wattroff(_:WINDOW_p, _:c_int) -> c_int;
  pub fn wattrset(_:WINDOW_p, _:c_int) -> c_int;
  pub fn wattr_get(_:WINDOW_p, _:attr_t_p, _:short_p, _:void_p) -> c_int;
  pub fn wattr_on(_:WINDOW_p, _:attr_t, _:void_p) -> c_int;
  pub fn wattr_off(_:WINDOW_p, _:attr_t, _:void_p) -> c_int;
  pub fn wattr_set(_:WINDOW_p, _:attr_t, _:c_short, _:void_p) -> c_int;
  pub fn wbkgd(_:WINDOW_p, _:chtype) -> c_int;
  pub fn wbkgdset(_:WINDOW_p,_:chtype);
  pub fn wborder(_:WINDOW_p,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype) -> c_int;
  pub fn wchgat(_:WINDOW_p, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
  pub fn wclear(_:WINDOW_p) -> c_int;
  pub fn wclrtobot(_:WINDOW_p) -> c_int;
  pub fn wclrtoeol(_:WINDOW_p) -> c_int;
  pub fn wcolor_set(_:WINDOW_p,_:c_short,_:void_p) -> c_int;
  pub fn wcursyncup(_:WINDOW_p);
  pub fn wdelch(_:WINDOW_p) -> c_int;
  pub fn wdeleteln(_:WINDOW_p) -> c_int;
  pub fn wechochar(_:WINDOW_p, _:chtype) -> c_int;
  pub fn werase(_:WINDOW_p) -> c_int;
  pub fn wgetch(_:WINDOW_p) -> c_int;
  pub fn wgetnstr(_:WINDOW_p,_:char_p,_:c_int) -> c_int;
  pub fn wgetstr(_:WINDOW_p, _:char_p) -> c_int;
  pub fn whline(_:WINDOW_p, _:chtype, _:c_int) -> c_int;
  pub fn winch(_:WINDOW_p) -> chtype;
  pub fn winchnstr(_:WINDOW_p, _:chtype_p, _:c_int) -> c_int;
  pub fn winchstr(_:WINDOW_p, _:chtype_p) -> c_int;
  pub fn winnstr(_:WINDOW_p, _:char_p, _:c_int) -> c_int;
  pub fn winsch(_:WINDOW_p, _:chtype) -> c_int;
  pub fn winsdelln(_:WINDOW_p,_:c_int) -> c_int;
  pub fn winsertln(_:WINDOW_p) -> c_int;
  pub fn winsnstr(_:WINDOW_p, _:char_p,_:c_int) -> c_int;
  pub fn winsstr(_:WINDOW_p, _:char_p) -> c_int;
  pub fn winstr(_:WINDOW_p, _:char_p) -> c_int;
  pub fn wmove(_:WINDOW_p,_:c_int,_:c_int) -> c_int;
  pub fn wnoutrefresh(_:WINDOW_p) -> c_int;
  pub fn wprintw(_:WINDOW_p, _:char_p) -> c_int;
  pub fn wredrawln(_:WINDOW_p,_:c_int,_:c_int) -> c_int;
  pub fn wrefresh(_:WINDOW_p) -> c_int;
  //  fn wscanw(_:WINDOW_p, _:NCURSES_CONST char_p) -> c_int;
  pub fn wscrl(_:WINDOW_p,_:c_int) -> c_int;
  pub fn wsetscrreg(_:WINDOW_p,_:c_int,_:c_int) -> c_int;
  pub fn wstandout(_:WINDOW_p) -> c_int;
  pub fn wstandend(_:WINDOW_p) -> c_int;
  pub fn wsyncdown(_:WINDOW_p);
  pub fn wsyncup(_:WINDOW_p);
  pub fn wtimeout(_:WINDOW_p,_:c_int);
  pub fn wtouchln(_:WINDOW_p,_:c_int,_:c_int,_:c_int) -> c_int;
  pub fn wvline(_:WINDOW_p,_:chtype,_:c_int) -> c_int;

  /*
   * These are also declared in <term.h>:
   */
  pub fn tigetflag(_:char_p) -> c_int;
  pub fn tigetnum(_:char_p) -> c_int;
  pub fn tigetstr(_:char_p) -> *c_char;
  pub fn putp(_:char_p) -> c_int;

  pub fn tparm(_:char_p) -> *c_char;

  /*
   * These functions are not in X/Open, but we use them in macro definitions:
   */
  pub fn getattrs(_:WINDOW_p) -> c_int;
  pub fn getcurx(_:WINDOW_p) -> c_int;
  pub fn getcury(_:WINDOW_p) -> c_int;
  pub fn getbegx(_:WINDOW_p) -> c_int;
  pub fn getbegy(_:WINDOW_p) -> c_int;
  pub fn getmaxx(_:WINDOW_p) -> c_int;
  pub fn getmaxy(_:WINDOW_p) -> c_int;
  pub fn getparx(_:WINDOW_p) -> c_int;
  pub fn getpary(_:WINDOW_p) -> c_int;

  /*
   * These extensions provide access to information stored in the WINDOW even
   * when NCURSES_OPAQUE is set:
   */
  pub fn wgetparent(_:WINDOW_p) -> WINDOW_p;
  pub fn is_cleared(_:WINDOW_p) -> c_int;
  pub fn is_idcok(_:WINDOW_p) -> c_int;
  pub fn is_idlok(_:WINDOW_p) -> c_int;
  pub fn is_immedok(_:WINDOW_p) -> c_int;
  pub fn is_keypad(_:WINDOW_p) -> c_int;
  pub fn is_leaveok(_:WINDOW_p) -> c_int;
  pub fn is_nodelay(_:WINDOW_p) -> c_int;
  pub fn is_notimeout(_:WINDOW_p) -> c_int;
  pub fn is_scrollok(_:WINDOW_p) -> c_int;
  pub fn is_syncok(_:WINDOW_p) -> c_int;
  pub fn wgetscrreg(_:WINDOW_p, _:*c_int, _:*c_int) -> c_int;
}

