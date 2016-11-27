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
  printw("Enter a character within 2 seconds: ");

  /* Wait for input. */
  let ch = wget_wch(stdscr());
  match ch {
    Some(WchResult::KeyCode(KEY_MOUSE)) => {
      /* Enable attributes and output message. */
      attron(A_BOLD() | A_BLINK());
      printw("\nMouse");
      attroff(A_BOLD() | A_BLINK());
      printw(" pressed");
    }

    Some(WchResult::KeyCode(_)) => {
      /* Enable attributes and output message. */
      attron(A_BOLD() | A_BLINK());
      printw("\nKeycode");
      attroff(A_BOLD() | A_BLINK());
      printw(" pressed");
    }

    Some(WchResult::Char(c)) => {
      /* Enable attributes and output message. */
      printw("\nKey pressed: ");
      attron(A_BOLD() | A_BLINK());
      printw(format!("{}\n", char::from_u32(c as u32).expect("Invalid char")).as_ref());
      attroff(A_BOLD() | A_BLINK());
    }

    None => {
      printw("\nYou didn't enter a character in time!");
    }
  }

  /* Refresh, showing the previous message. */
  refresh();

  /* Wait for one more character before exiting. Disable the input timeout. */
  nocbreak();
  getch();
  endwin();
}
