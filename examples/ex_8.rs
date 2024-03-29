// Derived from the ncurses Programming Howto, used under the
// following license:

// Copyright © 2001 by Pradeep Padala.

// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, distribute with modifications,
// sublicense, and/or sell copies of the Software, and to permit
// persons to whom the Software is furnished to do so, subject to the
// following conditions:

// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.

extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    addstr("Upper left corner           ").unwrap(); addch(ACS_ULCORNER()); addstr("\n").unwrap();
    addstr("Lower left corner           ").unwrap(); addch(ACS_LLCORNER()); addstr("\n").unwrap();
    addstr("Lower right corner          ").unwrap(); addch(ACS_LRCORNER()); addstr("\n").unwrap();
    addstr("Tee pointing right          ").unwrap(); addch(ACS_LTEE()); addstr("\n").unwrap();
    addstr("Tee pointing left           ").unwrap(); addch(ACS_RTEE()); addstr("\n").unwrap();
    addstr("Tee pointing up             ").unwrap(); addch(ACS_BTEE()); addstr("\n").unwrap();
    addstr("Tee pointing down           ").unwrap(); addch(ACS_TTEE()); addstr("\n").unwrap();
    addstr("Horizontal line             ").unwrap(); addch(ACS_HLINE()); addstr("\n").unwrap();
    addstr("Vertical line               ").unwrap(); addch(ACS_VLINE()); addstr("\n").unwrap();
    addstr("Large Plus or cross over    ").unwrap(); addch(ACS_PLUS()); addstr("\n").unwrap();
    addstr("Scan Line 1                 ").unwrap(); addch(ACS_S1()); addstr("\n").unwrap();
    addstr("Scan Line 3                 ").unwrap(); addch(ACS_S3()); addstr("\n").unwrap();
    addstr("Scan Line 7                 ").unwrap(); addch(ACS_S7()); addstr("\n").unwrap();
    addstr("Scan Line 9                 ").unwrap(); addch(ACS_S9()); addstr("\n").unwrap();
    addstr("Diamond                     ").unwrap(); addch(ACS_DIAMOND()); addstr("\n").unwrap();
    addstr("Checker board (stipple)     ").unwrap(); addch(ACS_CKBOARD()); addstr("\n").unwrap();
    addstr("Degree Symbol               ").unwrap(); addch(ACS_DEGREE()); addstr("\n").unwrap();
    addstr("Plus/Minus Symbol           ").unwrap(); addch(ACS_PLMINUS()); addstr("\n").unwrap();
    addstr("Bullet                      ").unwrap(); addch(ACS_BULLET()); addstr("\n").unwrap();
    addstr("Arrow Pointing Left         ").unwrap(); addch(ACS_LARROW()); addstr("\n").unwrap();
    addstr("Arrow Pointing Right        ").unwrap(); addch(ACS_RARROW()); addstr("\n").unwrap();
    addstr("Arrow Pointing Down         ").unwrap(); addch(ACS_DARROW()); addstr("\n").unwrap();
    addstr("Arrow Pointing Up           ").unwrap(); addch(ACS_UARROW()); addstr("\n").unwrap();
    addstr("Board of squares            ").unwrap(); addch(ACS_BOARD()); addstr("\n").unwrap();
    addstr("Lantern Symbol              ").unwrap(); addch(ACS_LANTERN()); addstr("\n").unwrap();
    addstr("Solid Square Block          ").unwrap(); addch(ACS_BLOCK()); addstr("\n").unwrap();
    addstr("Less/Equal sign             ").unwrap(); addch(ACS_LEQUAL()); addstr("\n").unwrap();
    addstr("Greater/Equal sign          ").unwrap(); addch(ACS_GEQUAL()); addstr("\n").unwrap();
    addstr("Pi                          ").unwrap(); addch(ACS_PI()); addstr("\n").unwrap();
    addstr("Not equal                   ").unwrap(); addch(ACS_NEQUAL()); addstr("\n").unwrap();
    addstr("UK pound sign               ").unwrap(); addch(ACS_STERLING()); addstr("\n").unwrap();

    refresh();
    getch();
    endwin();
}
