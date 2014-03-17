/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: examples/ex_2.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Basic input and attribute example.
*/

#[feature(globs)];
#[feature(managed_boxes)];

extern mod ncurses;

use std::char;
use ncurses::*;

#[nolink]
#[cfg(target_os = "linux")]
#[link(name = "GL")]
extern { }

fn main()
{
  /* Setup ncurses. */
  initscr();
  raw();

  /* Allow for extended keyboard (like F1). */
  keypad(stdscr, true);
  noecho();

  /* Prompt for a character. */
  printw("Enter a character: ");

  /* Wait for input. */
  let ch = getch();
  if ch == KEY_F1
  {
    /* Enable attributes and output message. */
    attron(A_BOLD() | A_BLINK());
    printw("\nF1");
    attroff(A_BOLD() | A_BLINK());
    printw(" pressed");
  }
  else
  {
    /* Enable attributes and output message. */
    printw("\nKey pressed: ");
    attron(A_BOLD() | A_BLINK());
    printw(format!("{}\n", char::from_u32(ch as u32).expect("Invalid char")));
    attroff(A_BOLD() | A_BLINK());
  }

  /* Refresh, showing the previous message. */
  refresh();

  /* Wait for one more character before exiting. */
  getch();
  endwin();
}

