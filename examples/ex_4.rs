/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: examples/ex_4.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Window creation and input example.
      Use the cursor keys to move the window
      around the screen.
*/

#![feature(globs)]

extern crate ncurses;

use ncurses::*;

static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;

fn main()
{
  /* Setup ncurses. */
  initscr();
  raw();

  /* Allow for extended keyboard (like F1). */
  keypad(stdscr, true);
  noecho();

  /* Invisible cursor. */
  curs_set(CURSOR_INVISIBLE);

  /* Status/help info. */
  printw("Use the arrow keys to move");
  mvprintw(LINES - 1, 0, "Press F1 to exit");
  refresh();

  /* Get the screen bounds. */
  let mut max_x = 0;
  let mut max_y = 0;
  getmaxyx(stdscr, &mut max_y, &mut max_x);

  /* Start in the center. */
  let mut start_y = (max_y - WINDOW_HEIGHT) / 2;
  let mut start_x = (max_x - WINDOW_WIDTH) / 2;
  let mut win = create_win(start_y, start_x);

  let mut ch = getch();
  while ch != KEY_F(1)
  {
    match ch
    {
      KEY_LEFT =>
      {
        start_x -= 1;
        destroy_win(win);
        win = create_win(start_y, start_x);
      },
      KEY_RIGHT =>
      {
        start_x += 1;
        destroy_win(win);
        win = create_win(start_y, start_x);
      },
      KEY_UP =>
      {
        start_y -= 1;
        destroy_win(win);
        win = create_win(start_y, start_x);
      },
      KEY_DOWN =>
      {
        start_y += 1;
        destroy_win(win);
        win = create_win(start_y, start_x);
      },
      _ => { }
    }
    ch = getch();
  }

  endwin();
}

fn create_win(start_y: i32, start_x: i32) -> WINDOW
{
  let win = newwin(WINDOW_HEIGHT, WINDOW_WIDTH, start_y, start_x);
  box_(win, 0, 0);
  wrefresh(win);
  win
}

fn destroy_win(win: WINDOW)
{
  let ch = ' ' as u32;
        wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
        wrefresh(win);
        delwin(win);
}
