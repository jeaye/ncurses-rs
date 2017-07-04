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

extern crate ncurses;

use std::env;
use std::io::Read;
use std::fs;
use std::path::Path;
use ncurses::*;

fn open_file() -> fs::File
{
  let args : Vec<_> = env::args().collect();
  if args.len() != 2
  {
    println!("Usage:\n\t{} <rust file>", args[0]);
    println!("Example:\n\t{} examples/ex_3.rs", args[0]);
    panic!("Exiting");
  }

  let reader = fs::File::open(Path::new(&args[1]));
  reader.ok().expect("Unable to open file")
}

fn prompt()
{
  printw("<-Press Any Key->");
  getch();
}

fn main()
{
  let reader = open_file().bytes();

  /* Start ncurses. */
  initscr();
  keypad(stdscr(), true);
  noecho();

  /* Get the screen bounds. */
  let mut max_x = 0;
  let mut max_y = 0;
  getmaxyx(stdscr(), &mut max_y, &mut max_x);

  /* Read the whole file. */
  for ch in reader
  {
    if ch.is_err()
    { break; }
    let ch = ch.unwrap();

    /* Get the current position on the screen. */
    let mut cur_x = 0;
    let mut cur_y = 0;
    getyx(stdscr(), &mut cur_y, &mut cur_x);

    if cur_y == (max_y - 1)
    {
      /* Status bar at the bottom. */
      prompt();

      /* Once a key is pressed, clear the screen and continue. */
      clear();
      mv(0, 0);
    }
    
    addch(ch as chtype);
  }

  /* Terminate ncurses. */
  mv(max_y -1, 0);
  prompt();
  endwin();
}
