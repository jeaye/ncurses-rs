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

    printw("Upper left corner           "); addch(ACS_ULCORNER()); printw("\n");
    printw("Lower left corner           "); addch(ACS_LLCORNER()); printw("\n");
    printw("Lower right corner          "); addch(ACS_LRCORNER()); printw("\n");
    printw("Tee pointing right          "); addch(ACS_LTEE()); printw("\n");
    printw("Tee pointing left           "); addch(ACS_RTEE()); printw("\n");
    printw("Tee pointing up             "); addch(ACS_BTEE()); printw("\n");
    printw("Tee pointing down           "); addch(ACS_TTEE()); printw("\n");
    printw("Horizontal line             "); addch(ACS_HLINE()); printw("\n");
    printw("Vertical line               "); addch(ACS_VLINE()); printw("\n");
    printw("Large Plus or cross over    "); addch(ACS_PLUS()); printw("\n");
    printw("Scan Line 1                 "); addch(ACS_S1()); printw("\n");
    printw("Scan Line 3                 "); addch(ACS_S3()); printw("\n");
    printw("Scan Line 7                 "); addch(ACS_S7()); printw("\n");
    printw("Scan Line 9                 "); addch(ACS_S9()); printw("\n");
    printw("Diamond                     "); addch(ACS_DIAMOND()); printw("\n");
    printw("Checker board (stipple)     "); addch(ACS_CKBOARD()); printw("\n");
    printw("Degree Symbol               "); addch(ACS_DEGREE()); printw("\n");
    printw("Plus/Minus Symbol           "); addch(ACS_PLMINUS()); printw("\n");
    printw("Bullet                      "); addch(ACS_BULLET()); printw("\n");
    printw("Arrow Pointing Left         "); addch(ACS_LARROW()); printw("\n");
    printw("Arrow Pointing Right        "); addch(ACS_RARROW()); printw("\n");
    printw("Arrow Pointing Down         "); addch(ACS_DARROW()); printw("\n");
    printw("Arrow Pointing Up           "); addch(ACS_UARROW()); printw("\n");
    printw("Board of squares            "); addch(ACS_BOARD()); printw("\n");
    printw("Lantern Symbol              "); addch(ACS_LANTERN()); printw("\n");
    printw("Solid Square Block          "); addch(ACS_BLOCK()); printw("\n");
    printw("Less/Equal sign             "); addch(ACS_LEQUAL()); printw("\n");
    printw("Greater/Equal sign          "); addch(ACS_GEQUAL()); printw("\n");
    printw("Pi                          "); addch(ACS_PI()); printw("\n");
    printw("Not equal                   "); addch(ACS_NEQUAL()); printw("\n");
    printw("UK pound sign               "); addch(ACS_STERLING()); printw("\n");

    refresh();
    getch();
    endwin();
}
