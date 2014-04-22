/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: examples/ex_1.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Simple "Hello, world" example.
*/

#![feature(globs)]

extern crate ncurses;

use ncurses::*;

fn main()
{
  /* Start ncurses. */
  initscr();

  /* Print to the back buffer. */
  printw("Hello, world!");

  /* Update the screen. */
  refresh();

  /* Wait for a key press. */
  getch();

  /* Terminate ncurses. */
  endwin();
}
