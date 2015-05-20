/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: examples/ex_1.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Simple "Hello, world" example.
*/

extern crate ncurses;

use ncurses::*;

fn main()
{
  /* If your locale env is unicode, you should use `setlocale`. */
  // let locale_conf = LcCategory::all;
  // setlocale(locale_conf, "zh_CN.UTF-8"); // if your locale is like mine(zh_CN.UTF-8).

  /* Start ncurses. */
  initscr();

  /* Print to the back buffer. */
  printw("Hello, world!");

  /* Print some unicode(Chinese) string. */
  // printw("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。";

  /* Update the screen. */
  refresh();

  /* Wait for a key press. */
  getch();

  /* Terminate ncurses. */
  endwin();
}
