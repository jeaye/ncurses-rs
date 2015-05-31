#![allow(dead_code)]
#![allow(unused_imports)]

use libc::{ c_int, c_void };
use ll::WINDOW;

pub type PANEL = *mut PANEL_impl;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PANEL_impl;

#[cfg(feature="panel")] #[link(name="panel")]
extern {
      pub fn panel_window(_:PANEL) -> WINDOW;
      pub fn update_panels() -> c_void;
      pub fn hide_panel(_:PANEL) -> c_int;
      pub fn show_panel(_:PANEL) -> c_int;
      pub fn del_panel(_:PANEL) -> c_int;
      pub fn top_panel(_:PANEL) -> c_int;
      pub fn bottom_panel(_:PANEL) -> c_int;
      pub fn new_panel(_:WINDOW) -> PANEL;
      pub fn panel_above(_:PANEL) -> PANEL;
      pub fn panel_below(_:PANEL) -> PANEL;
      pub fn move_panel(_:PANEL,_:c_int,_:c_int) -> c_int;
      pub fn replace_panel(_:PANEL,_:WINDOW) -> c_int;
      pub fn panel_hidden(_:PANEL) -> c_int;
}
