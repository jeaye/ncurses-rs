/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: examples/ex_2.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Basic input and attribute example.
*/

extern crate ncurses;

use std::char;
use ncurses::*;

fn main()
{
  /* Setup ncurses. */
  initscr();
  raw();

  /* Allow for extended keyboard (like F1). */
  keypad(stdscr(), true);
  noecho();

  /* Prompt for a character. */
  addstr("Enter a character: ");

  /* Wait for input. */
  let ch = getch();
  if ch == KEY_F(1)
  {
    /* Enable attributes and output message. */
    attron(A_BOLD | A_BLINK);
    addstr("\nF1");
    attroff(A_BOLD | A_BLINK);
    addstr(" pressed");
  }
  else
  {
    /* Enable attributes and output message. */
    addstr("\nKey pressed: ");
    attron(A_BOLD | A_BLINK);
    addstr(format!("{}\n", char::from_u32(ch as u32).expect("Invalid char")).as_ref());
    attroff(A_BOLD | A_BLINK);
  }

  /* Refresh, showing the previous message. */
  refresh();

  /* Wait for one more character before exiting. */
  getch();
  endwin();
}
