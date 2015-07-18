#![allow(dead_code)]
#![allow(unused_imports)]

use std::str;
use std::ptr;
use std::slice;
use std::ffi::{CStr, CString};
use libc::*;
use menu::ll;
use ll::{WINDOW, chtype, c_bool};
use constants::TRUE;

pub type MENU = ll::MENU;
pub type ITEM = ll::ITEM;
pub type HOOK = ll::HOOK;

#[cfg(feature="menu")]
pub fn menu_items(menu: MENU) -> Vec<ITEM> { 
  unsafe { 
    slice::from_raw_parts(super::ll::menu_items(menu), item_count(menu) as usize).to_vec()
  }
}

#[cfg(feature="menu")]
pub fn current_item(menu: MENU) -> ITEM {
  unsafe {
    super::ll::current_item(menu)
  }
}

#[cfg(feature="menu")]
pub fn new_item<T: Into<Vec<u8>>>(name: T, description: T) -> ITEM {
  unsafe {
    super::ll::new_item(CString::new(name).unwrap().into_ptr(), CString::new(description).unwrap().into_ptr())
  }
}

#[cfg(feature="menu")]
pub fn new_menu(items: &mut Vec<ITEM>) -> MENU {
  unsafe { 
    items.push(ptr::null_mut());
    let menu = super::ll::new_menu(items.as_mut_ptr());
    items.pop();

    menu
  }
}

#[cfg(feature="menu")]
pub fn item_opts(item: ITEM) -> i32 {
  unsafe {
    super::ll::item_opts(item)
  }
}

#[cfg(feature="menu")]
pub fn menu_opts(menu: MENU) -> i32 {
  unsafe {
    super::ll::menu_opts(menu)
  }
}

#[cfg(feature="menu")]
pub fn menu_sub(menu: MENU) -> WINDOW {
  unsafe {
    super::ll::menu_sub(menu)
  }
}

#[cfg(feature="menu")]
pub fn item_init(menu: MENU) -> HOOK {
  unsafe {
    super::ll::item_init(menu)
  }
}

#[cfg(feature="menu")]
pub fn item_term(menu: MENU) -> HOOK {
  unsafe {
    super::ll::item_term(menu)
  }
}

#[cfg(feature="menu")]
pub fn menu_init(menu: MENU) -> HOOK {
  unsafe {
    super::ll::menu_init(menu)
  }
}

#[cfg(feature="menu")]
pub fn menu_term(menu: MENU) -> HOOK {
  unsafe {
    super::ll::menu_term(menu)
  }
}

#[cfg(feature="menu")]
pub fn menu_win(menu: MENU) -> WINDOW {
  unsafe {
    super::ll::menu_win(menu)
  }
}

#[cfg(feature="menu")]
pub fn item_description(item: ITEM) -> String {
  unsafe {
    ptr_to_string(super::ll::item_description(item))
  }
}

#[cfg(feature="menu")]
pub fn item_name(item: ITEM) -> String {
  unsafe {
    ptr_to_string(super::ll::item_name(item))
  }
}

#[cfg(feature="menu")]
pub fn menu_mark(menu: MENU) -> String {
  unsafe {
    ptr_to_string(super::ll::menu_mark(menu))
  }
}

#[cfg(feature="menu")]
pub fn menu_request_name(request: i32) -> String {
  unsafe {
    ptr_to_string(super::ll::menu_request_name(request))
  }
}

#[cfg(feature="menu")]
pub fn menu_pattern(menu: MENU) -> String {
  unsafe {
    ptr_to_string(super::ll::menu_pattern(menu))
  }
}

#[cfg(feature="menu")]
pub fn menu_back(menu: MENU) -> chtype {
  unsafe {
    super::ll::menu_back(menu)
  }
}

#[cfg(feature="menu")]
pub fn menu_fore(menu: MENU) -> chtype {
  unsafe {
    super::ll::menu_fore(menu)
  }
}

#[cfg(feature="menu")]
pub fn menu_grey(menu: MENU) -> chtype {
  unsafe {
    super::ll::menu_grey(menu)
  }
}

#[cfg(feature="menu")]
pub fn free_item(item: ITEM) -> i32 {
  unsafe {
    CString::from_ptr(item as *const i8);
    super::ll::free_item(item)
  }
}

#[cfg(feature="menu")]
pub fn free_menu(menu: MENU) -> i32 {
  unsafe {
    super::ll::free_menu(menu)
  }
}

#[cfg(feature="menu")]
pub fn item_count(menu: MENU) -> i32 {
  unsafe {
    super::ll::item_count(menu)
  }
}

#[cfg(feature="menu")]
pub fn item_index(item: ITEM) -> i32 {
  unsafe {
    super::ll::item_index(item)
  }
}

#[cfg(feature="menu")]
pub fn item_opts_off(item: ITEM, opts: i32) -> i32 {
  unsafe {
    super::ll::item_opts_off(item, opts)
  }
}

#[cfg(feature="menu")]
pub fn item_opts_on(item: ITEM, opts: i32) -> i32 {
  unsafe {
    super::ll::item_opts_on(item, opts)
  }
}

#[cfg(feature="menu")]
pub fn menu_driver(menu: MENU, c: i32) -> i32 {
  unsafe {
    super::ll::menu_driver(menu, c)
  }
}

#[cfg(feature="menu")]
pub fn menu_opts_off(menu: MENU, opts: i32) -> i32 {
  unsafe {
    super::ll::menu_opts_off(menu, opts)
  }
}

#[cfg(feature="menu")]
pub fn menuopts_on(menu: MENU, opts: i32) -> i32 {
  unsafe {
    super::ll::menu_opts_on(menu, opts)
  }
}

#[cfg(feature="menu")]
pub fn menu_pad(menu: MENU) -> i32 {
  unsafe {
    super::ll::menu_pad(menu)
  }
}

#[cfg(feature="menu")]
pub fn pos_menu_cursor(menu: MENU) -> i32 {
  unsafe {
    super::ll::pos_menu_cursor(menu)
  }
}

#[cfg(feature="menu")]
pub fn post_menu(menu: MENU) -> i32 {
  unsafe {
    super::ll::post_menu(menu)
  }
}

#[cfg(feature="menu")]
pub fn scale_menu(menu: MENU, rows: &mut i32, cols: &mut i32) -> i32 {
  unsafe {
    super::ll::scale_menu(menu, rows as *mut c_int, cols as *mut c_int)
  }
}

#[cfg(feature="menu")]
pub fn set_current_item(menu: MENU, item: ITEM) -> i32 {
  unsafe {
    super::ll::set_current_item(menu, item)
  }
}

#[cfg(feature="menu")]
pub fn set_item_init(menu: MENU, hook: HOOK) -> i32 {
  unsafe {
    super::ll::set_item_init(menu, hook)
  }
}

#[cfg(feature="menu")]
pub fn set_item_opts(item: ITEM, opts: i32) -> i32 {
  unsafe {
    super::ll::set_item_opts(item, opts)
  }
}

#[cfg(feature="menu")]
pub fn set_item_term(menu: MENU, hook: HOOK) -> i32 {
  unsafe {
    super::ll::set_item_term(menu, hook)
  }
}

#[cfg(feature="menu")]
pub fn set_item_value(item: ITEM, value: bool) -> i32 {
  unsafe {
    super::ll::set_item_value(item, value as c_bool)
  }
}

#[cfg(feature="menu")]
pub fn set_menu_back(menu: MENU, attr: chtype) -> i32 {
  unsafe {
    super::ll::set_menu_back(menu, attr)
  }
}

#[cfg(feature="menu")]
pub fn set_menu_fore(menu: MENU, attr: chtype) -> i32 {
  unsafe {
    super::ll::set_menu_fore(menu, attr)
  }
}

#[cfg(feature="menu")]
pub fn set_menu_grey(menu: MENU, attr: chtype) -> i32 {
  unsafe {
    super::ll::set_menu_grey(menu, attr)
  }
}

#[cfg(feature="menu")]
pub fn set_menu_format(menu: MENU, rows: i32, cols: i32) -> i32 {
  unsafe {
    super::ll::set_menu_format(menu, rows, cols)
  }
}

#[cfg(feature="menu")]
pub fn set_menu_init(menu: MENU, hook: HOOK) -> i32 {
  unsafe {
    super::ll::set_menu_init(menu, hook)
  }
}

#[cfg(feature="menu")]
pub fn set_menu_items(menu: MENU, items: &mut Vec<ITEM>) -> i32 {
  unsafe { 
    items.push(ptr::null_mut());
    let ret = super::ll::set_menu_items(menu, items.as_mut_ptr());
    items.pop();

    ret
  }
}

#[cfg(feature="menu")]
pub fn set_menu_mark<T: Into<Vec<u8>>>(menu: MENU, mark: T) -> i32 {
  unsafe {
    super::ll::set_menu_mark(menu, CString::new(mark).unwrap().into_ptr())
  }
}

#[cfg(feature="menu")]
pub fn set_menu_opts(menu: MENU, opts: i32) -> i32 {
  unsafe {
    super::ll::set_menu_opts(menu, opts)
  }
}

#[cfg(feature="menu")]
pub fn set_menu_pad(menu: MENU, opts: i32) -> i32 {
  unsafe {
    super::ll::set_menu_pad(menu, opts)
  }
}

#[cfg(feature="menu")]
pub fn set_menu_pattern<T: Into<Vec<u8>>>(menu: MENU, pattern: T) -> i32 {
  unsafe {
    super::ll::set_menu_pattern(menu, CString::new(pattern).unwrap().into_ptr())
  }
}

#[cfg(feature="menu")]
pub fn set_menu_sub(menu: MENU, win: WINDOW) -> i32 {
  unsafe {
    super::ll::set_menu_sub(menu, win)
  }
}

#[cfg(feature="menu")]
pub fn set_menu_term(menu: MENU, hook: HOOK) -> i32 {
  unsafe {
    super::ll::set_menu_term(menu, hook)
  }
}

#[cfg(feature="menu")]
pub fn set_menu_win(menu: MENU, win: WINDOW) -> i32 {
  unsafe {
    super::ll::set_menu_win(menu, win)
  }
}

#[cfg(feature="menu")]
pub fn set_top_row(menu: MENU, row: i32) -> i32 {
  unsafe {
    super::ll::set_top_row(menu, row)
  }
}

#[cfg(feature="menu")]
pub fn top_row(menu: MENU) -> i32 {
  unsafe {
    super::ll::top_row(menu)
  }
}

#[cfg(feature="menu")]
pub fn unpost_menu(menu: MENU) -> i32 {
  unsafe {
    super::ll::unpost_menu(menu)
  }
}

#[cfg(feature="menu")]
pub fn menu_request_by_name<T: Into<Vec<u8>>>(name: T) -> i32 {
  unsafe {
    super::ll::menu_request_by_name(CString::new(name).unwrap().as_ptr())
  }
}

#[cfg(feature="menu")]
pub fn set_menu_spacing(menu: MENU, spc_description: i32, spc_rows: i32, spc_columns: i32) -> i32 {
  unsafe {
    super::ll::set_menu_spacing(menu, spc_description, spc_rows, spc_columns)
  }
}

#[cfg(feature="menu")]
pub fn menu_spacing(menu: MENU, spc_description: &mut i32, spc_rows: &mut i32, spc_columns: &mut i32) -> i32 {
  unsafe {
    super::ll::menu_spacing(menu, spc_description as *mut i32, spc_rows as *mut i32, spc_columns as *mut i32)
  }
}

#[cfg(feature="menu")]
pub fn item_value(item: ITEM) -> bool {
  unsafe {
    super::ll::item_value(item) == TRUE
  }
}

#[cfg(feature="menu")]
pub fn item_visible(item: ITEM) -> bool {
  unsafe {
    super::ll::item_visible(item) == TRUE
  }
}

#[cfg(feature="menu")]
pub fn menu_format(menu: MENU, rows: &mut i32, cols: &mut i32) {
  unsafe {
    super::ll::menu_format(menu, rows as *mut i32, cols as *mut i32);
  }
}

pub fn ptr_to_string(ptr: *const c_char) -> String {
  unsafe {
    str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()).to_owned()
  }
}
