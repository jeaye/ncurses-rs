#![allow(dead_code)]
#![allow(unused_imports)]

use panel::ll;
use ll::WINDOW;
use constants::TRUE;

pub type PANEL = ll::PANEL;

#[cfg(feature="panel")]
pub fn panel_window(panel: PANEL) -> WINDOW
{ unsafe { ll::panel_window(panel) } }

#[cfg(feature="panel")]
pub fn update_panels()
{ unsafe { ll::update_panels(); } }

#[cfg(feature="panel")]
pub fn hide_panel(panel: PANEL) -> i32
{ unsafe { ll::hide_panel(panel) } }

#[cfg(feature="panel")]
pub fn show_panel(panel: PANEL) -> i32
{ unsafe { ll::show_panel(panel) } }

#[cfg(feature="panel")]
pub fn del_panel(panel: PANEL) -> i32
{ unsafe { ll::del_panel(panel) } }

#[cfg(feature="panel")]
pub fn top_panel(panel: PANEL) -> i32
{ unsafe { ll::top_panel(panel) } }

#[cfg(feature="panel")]
pub fn bottom_panel(panel: PANEL) -> i32
{ unsafe { ll::bottom_panel(panel) } }

#[cfg(feature="panel")]
pub fn new_panel(window: WINDOW) -> PANEL
{ unsafe { ll::new_panel(window) } }

#[cfg(feature="panel")]
pub fn panel_above(panel: PANEL) -> PANEL
{ unsafe { ll::panel_above(panel) } }

#[cfg(feature="panel")]
pub fn panel_below(panel: PANEL) -> PANEL
{ unsafe { ll::panel_below(panel) } }

#[cfg(feature="panel")]
pub fn move_panel(panel: PANEL, y: i32, x: i32) -> i32
{ unsafe { ll::move_panel(panel, y, x) } }

#[cfg(feature="panel")]
pub fn replace_panel(panel: PANEL, window: WINDOW) -> i32
{ unsafe { ll::replace_panel(panel, window) } }

#[cfg(feature="panel")]
pub fn panel_hidden(panel: PANEL) -> bool
{ unsafe { ll::panel_hidden(panel) == TRUE } }
