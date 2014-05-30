/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: examples/ex_3.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Implementation of a very simple pager.

      Usage:
        ./bin/ex_3 <rust file>
      Example:
        ./bin/ex_3 examples/ex_3.rs
*/

#![feature(globs)]

extern crate ncurses;

use std::os;
use std::io;
use std::io::File;
use ncurses::*;

fn open_file() -> io::File
{
  let args = os::args();
  if args.len() != 2
  {
    println!("Usage:\n\t{} <rust file>", args.get(0));
    println!("Example:\n\t{} examples/ex_3.rs", args.get(0));
    fail!("Exiting");
  }

  let reader = File::open(&Path::new(args.get(1).to_string()));
  reader.ok().expect("Unable to open file")
}

fn prompt()
{
  printw("<-Press Any Key->");
  getch();
}

fn main()
{
  let mut reader = open_file();

  /* Start ncurses. */
  initscr();
  keypad(stdscr, true);
  noecho();

  /* Get the screen bounds. */
  let mut max_x = 0;
  let mut max_y = 0;
  getmaxyx(stdscr, &mut max_y, &mut max_x);

  /* Read the whole file. */
  while !reader.eof()
  {
    /* Read a character at a time. */
    let ch = reader.read_byte();
    if ch.is_err()
    { break; }
    let ch = ch.unwrap();

    /* Get the current position on the screen. */
    let mut cur_x = 0;
    let mut cur_y = 0;
    getyx(stdscr, &mut cur_y, &mut cur_x);

    if cur_y == (max_y - 1)
    {
      /* Status bar at the bottom. */
      prompt();

      /* Once a key is pressed, clear the screen and continue. */
      clear();
      move(0, 0);
    }
    else
    { addch(ch as u32); }
  }

  /* Terminate ncurses. */
  move(max_y -1, 0);
  prompt();
  endwin();
}
