#![allow(dead_code)]
#![allow(unused_imports)]

use libc::{c_int, c_char, c_void};
use crate::ll::{WINDOW, chtype, c_bool};

pub type MENU = *mut i8;
pub type ITEM = *mut i8;
pub type HOOK = Option<extern "C" fn(MENU)>;

#[cfg(feature="menu")]
extern {
    pub fn menu_items(_:MENU) -> *mut ITEM;
    pub fn current_item(_:MENU) -> ITEM;
    pub fn new_item(_:*const c_char, _:*const c_char) -> ITEM;
    pub fn free_item(_:ITEM);
    pub fn new_menu(_:*mut ITEM) -> MENU;
    pub fn item_opts(_:ITEM) -> c_int;
    pub fn menu_opts(_:MENU) -> c_int;

    pub fn item_init(_:MENU) -> HOOK;
    pub fn item_term(_:MENU) -> HOOK;
    pub fn menu_init(_:MENU) -> HOOK;
    pub fn menu_term(_:MENU) -> HOOK;

    pub fn menu_sub(_:MENU) -> WINDOW;
    pub fn menu_win(_:MENU) -> WINDOW;

    pub fn item_description(_:ITEM) -> *const c_char;
    pub fn item_name(_:ITEM) -> *const c_char;
    pub fn menu_mark(_:MENU) -> *const c_char;
    pub fn menu_request_name(_:c_int) -> *const c_char;

    pub fn menu_pattern(_:MENU) -> *mut c_char;

    pub fn menu_back(_:MENU) -> chtype;
    pub fn menu_fore(_:MENU) -> chtype;
    pub fn menu_grey(_:MENU) -> chtype;

    pub fn free_menu(_:MENU) -> c_int;
    pub fn item_count(_:MENU) -> c_int;
    pub fn item_index(_:ITEM) -> c_int;
    pub fn item_opts_off(_:ITEM, _:c_int) -> c_int;
    pub fn item_opts_on(_:ITEM, _:c_int) -> c_int;
    pub fn menu_driver(_:MENU, _:c_int) -> c_int;
    pub fn menu_opts_off(_:MENU, _:c_int) -> c_int;
    pub fn menu_opts_on(_:MENU, _:c_int) -> c_int;
    pub fn menu_pad(_:MENU) -> c_int;
    pub fn pos_menu_cursor(_:MENU) -> c_int;
    pub fn post_menu(_:MENU) -> c_int;
    pub fn scale_menu(_:MENU, _:*mut c_int, _:*mut c_int) -> c_int;
    pub fn set_current_item(_:MENU, _:ITEM) -> c_int;
    pub fn set_item_init(_:MENU, _:HOOK) -> c_int;
    pub fn set_item_opts(_:ITEM, _:c_int) -> c_int;
    pub fn set_item_term(_:MENU, _:HOOK) -> c_int;
    pub fn set_item_value(_:ITEM, _:c_bool) -> c_int;
    pub fn set_menu_back(_:MENU, _:chtype) -> c_int;
    pub fn set_menu_fore(_:MENU, _:chtype) -> c_int;
    pub fn set_menu_format(_:MENU, _:c_int, _:c_int) -> c_int;
    pub fn set_menu_grey(_:MENU, _:chtype) -> c_int;
    pub fn set_menu_init(_:MENU, _:HOOK) -> c_int;
    pub fn set_menu_items(_:MENU, _:*mut ITEM) -> c_int;
    pub fn set_menu_mark(_:MENU, _:*const c_char) -> c_int;
    pub fn set_menu_opts(_:MENU, _:c_int) -> c_int;
    pub fn set_menu_pad(_:MENU, _:c_int) -> c_int;
    pub fn set_menu_pattern(_:MENU, _:*const c_char) -> c_int;
    pub fn set_menu_sub(_:MENU, _:WINDOW) -> c_int;
    pub fn set_menu_term(_:MENU, _:HOOK) -> c_int;
    pub fn set_menu_win(_:MENU, _:WINDOW) -> c_int;
    pub fn set_top_row(_:MENU, _:c_int) -> c_int;
    pub fn top_row(_:MENU) -> c_int;
    pub fn unpost_menu(_:MENU) -> c_int;
    pub fn menu_request_by_name(_:*const c_char) -> c_int;
    pub fn set_menu_spacing(_:MENU, _:c_int, _:c_int, _:c_int) -> c_int;
    pub fn menu_spacing(_:MENU, _:*mut c_int, _:*mut c_int, _:*mut c_int) -> c_int;

    pub fn item_value(_:ITEM) -> c_bool;
    pub fn item_visible(_:ITEM) -> c_bool;

    pub fn menu_format(_:MENU, _:*mut c_int, _:*mut c_int);
}
