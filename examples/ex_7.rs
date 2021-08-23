/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: examples/ex_7.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Basic input and attribute example, using the Unicode-aware get_wch functions.
*/

extern crate ncurses;

use std::char;
use ncurses::*;

#[cfg(feature = "wide")]
fn main()
{
  let locale_conf = LcCategory::all;
  setlocale(locale_conf, "en_US.UTF-8");

  /* Setup ncurses. */
  initscr();
  raw();

  /* Require input within 2 seconds. */
  halfdelay(20);
  /* Enable mouse events. */
  mousemask(ALL_MOUSE_EVENTS as mmask_t, None);

  /* Allow for extended keyboard (like F1). */
  keypad(stdscr(), true);
  noecho();

  /* Prompt for a character. */
  addstr("Enter a character within 2 seconds: ");

  /* Wait for input. */
  let ch = wget_wch(stdscr());
  match ch {
    Some(WchResult::KeyCode(KEY_MOUSE)) => {
      /* Enable attributes and output message. */
      attron(A_BOLD | A_BLINK);
      addstr("\nMouse");
      attroff(A_BOLD | A_BLINK);
      addstr(" pressed");
    }

    Some(WchResult::KeyCode(_)) => {
      /* Enable attributes and output message. */
      attron(A_BOLD | A_BLINK);
      addstr("\nKeycode");
      attroff(A_BOLD | A_BLINK);
      addstr(" pressed");
    }

    Some(WchResult::Char(c)) => {
      /* Enable attributes and output message. */
      addstr("\nKey pressed: ");
      attron(A_BOLD | A_BLINK);
      addstr(format!("{}\n", char::from_u32(c as u32).expect("Invalid char")).as_ref());
      attroff(A_BOLD | A_BLINK);
    }

    None => {
      addstr("\nYou didn't enter a character in time!");
    }
  }

  /* Refresh, showing the previous message. */
  refresh();

  /* Wait for one more character before exiting. Disable the input timeout. */
  nocbreak();
  getch();
  endwin();
}

#[cfg(not(feature = "wide"))]
fn main() {
  initscr();
  addstr("This example requires wide character support.");
  getch();
  endwin();
}
