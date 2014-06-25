#![allow(dead_code)]

/*
Copyright © 2013 Free Software Foundation, Inc
See licensing in LICENSE file

File: ll.rs
Author: Jesse 'Jeaye' Wilkerson
Description:
Low-level interface to foreign
ncurses functions.
 */

use libc::{ c_char, c_int, c_short, c_uint, c_void, FILE };

/* Intrinsic types. */
pub type chtype = c_uint;
pub type mmask_t = c_uint;
pub type attr_t = c_int;
pub type NCURSES_ATTR_T = attr_t;

/* Pointer types. */
pub type attr_t_p = *attr_t;
pub type short_p = *c_short;
pub type void_p = *c_void;
pub type char_p = *c_char;
pub type chtype_p = *chtype;
pub type WINDOW = *WINDOW_impl;
pub type SCREEN = *SCREEN_impl;
pub type FILE_p = *FILE;
pub type va_list = *u8;

/* Custom Types. */
pub struct WINDOW_impl;
pub struct SCREEN_impl;

pub struct MEVENT { id: c_short, x: c_int, y: c_int, z: c_int, bstate: mmask_t}

macro_rules! define_sharedffi(
    ($cfgopt: meta, $link: meta) => {
        #[$cfgopt] #[$link] extern {
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
            pub fn box_(_:WINDOW, _:chtype, _:chtype) -> c_int;
            pub fn can_change_color() -> c_int;
            pub fn cbreak() -> c_int;
            pub fn chgat(_:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
            pub fn clear() -> c_int;
            pub fn clearok(_:WINDOW,_:c_int) -> c_int;
            pub fn clrtobot() -> c_int;
            pub fn clrtoeol() -> c_int;
            pub fn color_content(_:c_short,_:short_p,_:short_p,_:short_p) -> c_int;
            pub fn color_set(_:c_short,_:void_p) -> c_int;
            pub fn COLOR_PAIR(_:c_int) -> c_int;
            pub fn copywin(_:WINDOW,_:WINDOW,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
            pub fn curs_set(_:c_int) -> c_int;
            pub fn def_prog_mode() -> c_int;
            pub fn def_shell_mode() -> c_int;
            pub fn delay_output(_:c_int) -> c_int;
            pub fn delch() -> c_int;
            pub fn delscreen(_:SCREEN);
            pub fn delwin(_:WINDOW) -> c_int;
            pub fn deleteln() -> c_int;
            pub fn derwin(_:WINDOW,_:c_int,_:c_int,_:c_int,_:c_int) -> WINDOW;
            pub fn doupdate() -> c_int;
            pub fn dupwin(_:WINDOW) -> WINDOW;
            pub fn echo() -> c_int;
            pub fn echochar(_:chtype) -> c_int;
            pub fn erase() -> c_int;
            pub fn endwin() -> c_int;
            pub fn erasechar() -> c_char;
            pub fn filter();
            pub fn flash() -> c_int;
            pub fn flushinp() -> c_int;
            pub fn getbkgd(_:WINDOW) -> chtype;
            pub fn getch() -> c_int;
            pub fn getnstr(_:*mut c_char, _:c_int) -> c_int;
            pub fn getstr(_:char_p) -> c_int;
            pub fn getwin(_:FILE_p) -> WINDOW;
            pub fn halfdelay(_:c_int) -> c_int;
            pub fn has_colors() -> c_int;
            pub fn has_ic() -> c_int;
            pub fn has_il() -> c_int;
            pub fn hline(_:chtype, _:c_int) -> c_int;
            pub fn idcok(_:WINDOW, _:c_int);
            pub fn idlok(_:WINDOW, _:c_int) -> c_int;
            pub fn immedok(_:WINDOW, _:c_int);
            pub fn inch() -> chtype;
            pub fn inchnstr(_:chtype_p, _:c_int) -> c_int;
            pub fn inchstr(_:chtype_p) -> c_int;
            pub fn initscr() -> WINDOW;
            pub fn init_color(_:c_short,_:c_short,_:c_short,_:c_short) -> c_int;
            pub fn init_pair(_:c_short,_:c_short,_:c_short) -> c_int;
            pub fn innstr(_:char_p, _:c_int) -> c_int;
            pub fn insch(_:chtype) -> c_int;
            pub fn insdelln(_:c_int) -> c_int;
            pub fn insertln() -> c_int;
            pub fn insnstr(_:char_p, _:c_int) -> c_int;
            pub fn insstr(_:char_p) -> c_int;
            pub fn instr(_:char_p) -> c_int;
            pub fn intrflush(_:WINDOW,_:c_int) -> c_int;
            pub fn isendwin() -> c_int;
            pub fn is_linetouched(_:WINDOW,_:c_int) -> c_int;
            pub fn is_wintouched(_:WINDOW) -> c_int;
            pub fn keyname(_:c_int) -> *c_char;
            pub fn keypad(_:WINDOW, _:c_int) -> c_int;
            pub fn killchar() -> c_char;
            pub fn leaveok(_:WINDOW,_:c_int) -> c_int;
            pub fn longname() -> *c_char;
            pub fn meta(_:WINDOW,_:c_int) -> c_int;
            pub fn move(_:c_int, _:c_int) -> c_int;
            pub fn mvaddch(_:c_int, _:c_int, _:chtype) -> c_int;
            pub fn mvaddchnstr(_:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
            pub fn mvaddchstr(_:c_int, _:c_int, _:chtype_p) -> c_int;
            pub fn mvaddnstr(_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
            pub fn mvaddstr(_:c_int, _:c_int, _:char_p) -> c_int;
            pub fn mvchgat(_:c_int, _:c_int, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
            pub fn mvcur(_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
            pub fn mvdelch(_:c_int, _:c_int) -> c_int;
            pub fn mvderwin(_:WINDOW, _:c_int, _:c_int) -> c_int;
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
            pub fn mvwaddch(_:WINDOW, _:c_int, _:c_int, _:chtype) -> c_int;
            pub fn mvwaddchnstr(_:WINDOW, _:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
            pub fn mvwaddchstr(_:WINDOW, _:c_int, _:c_int, _:chtype_p) -> c_int;
            pub fn mvwaddnstr(_:WINDOW, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
            pub fn mvwaddstr(_:WINDOW, _:c_int, _:c_int, _:char_p) -> c_int;
            pub fn mvwchgat(_:WINDOW, _:c_int, _:c_int, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
            pub fn mvwdelch(_:WINDOW, _:c_int, _:c_int) -> c_int;
            pub fn mvwgetch(_:WINDOW, _:c_int, _:c_int) -> c_int;
            pub fn mvwgetnstr(_:WINDOW, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
            pub fn mvwgetstr(_:WINDOW, _:c_int, _:c_int, _:char_p) -> c_int;
            pub fn mvwhline(_:WINDOW, _:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
            pub fn mvwin(_:WINDOW,_:c_int,_:c_int) -> c_int;
            pub fn mvwinch(_:WINDOW, _:c_int, _:c_int) -> chtype;
            pub fn mvwinchnstr(_:WINDOW, _:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
            pub fn mvwinchstr(_:WINDOW, _:c_int, _:c_int, _:chtype_p) -> c_int;
            pub fn mvwinnstr(_:WINDOW, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
            pub fn mvwinsch(_:WINDOW, _:c_int, _:c_int, _:chtype) -> c_int;
            pub fn mvwinsnstr(_:WINDOW, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
            pub fn mvwinsstr(_:WINDOW, _:c_int, _:c_int, _:char_p) -> c_int;
            pub fn mvwinstr(_:WINDOW, _:c_int, _:c_int, _:char_p) -> c_int;
            pub fn mvwprintw(_:WINDOW, _:c_int, _:c_int, _:char_p) -> c_int;

            //  fn mvwscanw(_:WINDOW, _:c_int, _:c_int, _:char_p) -> c_int;
            pub fn mvwvline(_:WINDOW, _:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
            pub fn napms(_:c_int) -> c_int;
            pub fn newpad(_:c_int,_:c_int) -> WINDOW;
            pub fn newterm(_:char_p,_:FILE_p,_:FILE_p) -> SCREEN;
            pub fn newwin(_:c_int,_:c_int,_:c_int,_:c_int) -> WINDOW;
            pub fn nl() -> c_int;
            pub fn nocbreak() -> c_int;
            pub fn nodelay(_:WINDOW,_:c_int) -> c_int;
            pub fn noecho() -> c_int;
            pub fn nonl() -> c_int;
            pub fn noqiflush();
            pub fn noraw() -> c_int;
            pub fn notimeout(_:WINDOW,_:c_int) -> c_int;
            pub fn overlay(_:WINDOW,_:WINDOW) -> c_int;
            pub fn overwrite(_:WINDOW,_:WINDOW) -> c_int;
            pub fn pair_content(_:c_short,_:short_p,_:short_p) -> c_int;
            pub fn PAIR_NUMBER(_:c_int) -> c_int;
            pub fn pechochar(_:WINDOW, _:chtype) -> c_int;
            pub fn pnoutrefresh(_:WINDOW,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
            pub fn prefresh(_:WINDOW,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;

            pub fn printw(_:char_p) -> c_int;
            pub fn putwin(_:WINDOW, _:FILE_p) -> c_int;
            pub fn qiflush();
            pub fn raw() -> c_int;
            pub fn redrawwin(_:WINDOW) -> c_int;
            pub fn refresh() -> c_int;
            pub fn resetty() -> c_int;
            pub fn reset_prog_mode() -> c_int;
            pub fn reset_shell_mode() -> c_int;
            // fn ripoffline(_:c_int, extern  fn f(WINDOW, c_int) -> c_int) -> c_int;
            pub fn savetty() -> c_int;
            // fn scanw(_:NCURSES_CONST char_p,...) -> c_int;
            pub fn scr_dump(_:char_p) -> c_int;
            pub fn scr_init(_:char_p) -> c_int;
            pub fn scrl(_:c_int) -> c_int;
            pub fn scroll(_:WINDOW) -> c_int;
            pub fn scrollok(_:WINDOW,_:c_int) -> c_int;
            pub fn scr_restore(_:char_p) -> c_int;
            pub fn scr_set(_:char_p) -> c_int;
            pub fn setscrreg(_:c_int,_:c_int) -> c_int;
            pub fn set_term(_:SCREEN) -> SCREEN;
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
            pub fn subpad(_:WINDOW, _:c_int, _:c_int, _:c_int, _:c_int) -> WINDOW;
            pub fn subwin(_:WINDOW, _:c_int, _:c_int, _:c_int, _:c_int) -> WINDOW;
            pub fn syncok(_:WINDOW, _:c_int) -> c_int;
            pub fn termattrs() -> chtype;
            pub fn termname() -> char_p;
            pub fn timeout(_:c_int);
            pub fn touchline(_:WINDOW, _:c_int, _:c_int) -> c_int;
            pub fn touchwin(_:WINDOW) -> c_int;
            pub fn typeahead(_:c_int) -> c_int;
            pub fn ungetch(_:c_int) -> c_int;
            pub fn untouchwin(_:WINDOW) -> c_int;
            pub fn use_env(_:c_int);
            pub fn vidattr(_:chtype) -> c_int;
            //  fn vidputs(_:chtype, extern  fn f(c_int) -> c_int) -> c_int;
            //pub fn vidputs(_:chtype, f:*c_char) -> c_int;
            pub fn vline(_:chtype, _:c_int) -> c_int;
            pub fn vwprintw(_:WINDOW, _:char_p, _:va_list) -> c_int;
            pub fn vw_printw(_:WINDOW, _:char_p,_:va_list) -> c_int;
            //  fn vwscanw(_:WINDOW, _:char_p, _:va_list) -> c_int;
            //  fn vw_scanw(_:WINDOW, _:char_p, _:va_list) -> c_int;
            pub fn waddch(_:WINDOW, _:chtype) -> c_int;
            pub fn waddchnstr(_:WINDOW,_:chtype_p,_:c_int) -> c_int;
            pub fn waddchstr(_:WINDOW,_:chtype_p) -> c_int;
            pub fn waddnstr(_:WINDOW,_:char_p,_:c_int) -> c_int;
            pub fn waddstr(_:WINDOW,_:char_p) -> c_int;
            pub fn wattron(_:WINDOW, _:c_int) -> c_int;
            pub fn wattroff(_:WINDOW, _:c_int) -> c_int;
            pub fn wattrset(_:WINDOW, _:c_int) -> c_int;
            pub fn wattr_get(_:WINDOW, _:attr_t_p, _:short_p, _:void_p) -> c_int;
            pub fn wattr_on(_:WINDOW, _:attr_t, _:void_p) -> c_int;
            pub fn wattr_off(_:WINDOW, _:attr_t, _:void_p) -> c_int;
            pub fn wattr_set(_:WINDOW, _:attr_t, _:c_short, _:void_p) -> c_int;
            pub fn wbkgd(_:WINDOW, _:chtype) -> c_int;
            pub fn wbkgdset(_:WINDOW,_:chtype);
            pub fn wborder(_:WINDOW,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype) -> c_int;
            pub fn wchgat(_:WINDOW, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
            pub fn wclear(_:WINDOW) -> c_int;
            pub fn wclrtobot(_:WINDOW) -> c_int;
            pub fn wclrtoeol(_:WINDOW) -> c_int;
            pub fn wcolor_set(_:WINDOW,_:c_short,_:void_p) -> c_int;
            pub fn wcursyncup(_:WINDOW);
            pub fn wdelch(_:WINDOW) -> c_int;
            pub fn wdeleteln(_:WINDOW) -> c_int;
            pub fn wechochar(_:WINDOW, _:chtype) -> c_int;
            pub fn werase(_:WINDOW) -> c_int;
            pub fn wgetch(_:WINDOW) -> c_int;
            pub fn wgetnstr(_:WINDOW,_:char_p,_:c_int) -> c_int;
            pub fn wgetstr(_:WINDOW, _:char_p) -> c_int;
            pub fn whline(_:WINDOW, _:chtype, _:c_int) -> c_int;
            pub fn winch(_:WINDOW) -> chtype;
            pub fn winchnstr(_:WINDOW, _:chtype_p, _:c_int) -> c_int;
            pub fn winchstr(_:WINDOW, _:chtype_p) -> c_int;
            pub fn winnstr(_:WINDOW, _:char_p, _:c_int) -> c_int;
            pub fn winsch(_:WINDOW, _:chtype) -> c_int;
            pub fn winsdelln(_:WINDOW,_:c_int) -> c_int;
            pub fn winsertln(_:WINDOW) -> c_int;
            pub fn winsnstr(_:WINDOW, _:char_p,_:c_int) -> c_int;
            pub fn winsstr(_:WINDOW, _:char_p) -> c_int;
            pub fn winstr(_:WINDOW, _:char_p) -> c_int;
            pub fn wmove(_:WINDOW,_:c_int,_:c_int) -> c_int;
            pub fn wnoutrefresh(_:WINDOW) -> c_int;
            pub fn wprintw(_:WINDOW, _:char_p) -> c_int;
            pub fn wredrawln(_:WINDOW,_:c_int,_:c_int) -> c_int;
            pub fn wrefresh(_:WINDOW) -> c_int;
            //  fn wscanw(_:WINDOW, _:NCURSES_CONST char_p) -> c_int;
            pub fn wscrl(_:WINDOW,_:c_int) -> c_int;
            pub fn wsetscrreg(_:WINDOW,_:c_int,_:c_int) -> c_int;
            pub fn wstandout(_:WINDOW) -> c_int;
            pub fn wstandend(_:WINDOW) -> c_int;
            pub fn wsyncdown(_:WINDOW);
            pub fn wsyncup(_:WINDOW);
            pub fn wtimeout(_:WINDOW,_:c_int);
            pub fn wtouchln(_:WINDOW,_:c_int,_:c_int,_:c_int) -> c_int;
            pub fn wvline(_:WINDOW,_:chtype,_:c_int) -> c_int;

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
            pub fn getattrs(_:WINDOW) -> c_int;
            pub fn getcurx(_:WINDOW) -> c_int;
            pub fn getcury(_:WINDOW) -> c_int;
            pub fn getbegx(_:WINDOW) -> c_int;
            pub fn getbegy(_:WINDOW) -> c_int;
            pub fn getmaxx(_:WINDOW) -> c_int;
            pub fn getmaxy(_:WINDOW) -> c_int;
            pub fn getparx(_:WINDOW) -> c_int;
            pub fn getpary(_:WINDOW) -> c_int;

            /*
             * These extensions provide access to information stored in the WINDOW even
             * when NCURSES_OPAQUE is set:
             */
            pub fn wgetparent(_:WINDOW) -> WINDOW;
            pub fn is_cleared(_:WINDOW) -> c_int;
            pub fn is_idcok(_:WINDOW) -> c_int;
            pub fn is_idlok(_:WINDOW) -> c_int;
            pub fn is_immedok(_:WINDOW) -> c_int;
            pub fn is_keypad(_:WINDOW) -> c_int;
            pub fn is_leaveok(_:WINDOW) -> c_int;
            pub fn is_nodelay(_:WINDOW) -> c_int;
            pub fn is_notimeout(_:WINDOW) -> c_int;
            pub fn is_scrollok(_:WINDOW) -> c_int;
            pub fn is_syncok(_:WINDOW) -> c_int;
            pub fn wgetscrreg(_:WINDOW, _:*c_int, _:*c_int) -> c_int;
            /*
             * Added mouse support
             */
            pub fn has_mouse() -> c_int;
            pub fn getmouse(_:*MEVENT) -> c_int;
            pub fn ungetmouse(_:*MEVENT) -> c_int;
            pub fn mousemask(_:mmask_t,_:*mmask_t) -> mmask_t;
            pub fn wenclose(_:WINDOW,_:c_int,_:c_int) -> c_int;
            pub fn mouseinterval(_:c_int) -> c_int;
            pub fn wmouse_trafo(_:*WINDOW,_:*c_int,_:*c_int,_:c_int) -> c_int;
            pub fn mouse_trafo(_:*c_int,_:*c_int,_:c_int) -> c_int;
        }
    })

//end macro rules

define_sharedffi!(cfg(ncursesw), link(name="ncursesw"))
define_sharedffi!(cfg(not(ncursesw)), link(name="ncurses"))
