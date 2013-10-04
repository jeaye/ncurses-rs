extern mod ncurses;

use std::char;
use ncurses::*;

fn main()
{
  initscr();
  raw();
  keypad(stdscr, true);
  noecho();

  printw("Enter a character: ");

  let ch = getch();
  if ch == KEY_F1
  {
    attron(A_BOLD() | A_BLINK());
    printw("\nF1");
    attroff(A_BOLD() | A_BLINK());
    printw(" pressed");
  }
  else
  {
    printw("\nKey pressed: ");
    attron(A_BOLD() | A_BLINK());
    printw(fmt!("%c\n", char::from_u32(ch as u32).expect("Invalid char")));
    attroff(A_BOLD() | A_BLINK());
  }

  refresh();
  getch();
  endwin();
}

