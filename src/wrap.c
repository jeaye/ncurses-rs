#include <curses.h>

#ifndef NCURSES_VERSION
/*
 * NetBSD Curses only defines this as a macro
 */

int (COLOR_PAIR)(int n) {
	return COLOR_PAIR(n);
}

#endif

/*
 * ACS_ values aren't set until after `init_scr` 
 */

int impl_ACS_ULCORNER() { return ACS_ULCORNER; }
int impl_ACS_LLCORNER() { return ACS_LLCORNER; }
int impl_ACS_URCORNER() { return ACS_URCORNER; }
int impl_ACS_LRCORNER() { return ACS_LRCORNER; }
int impl_ACS_LTEE() { return ACS_LTEE; }
int impl_ACS_RTEE() { return ACS_RTEE; }
int impl_ACS_BTEE() { return ACS_BTEE; }
int impl_ACS_TTEE() { return ACS_TTEE; }
int impl_ACS_HLINE() { return ACS_HLINE; }
int impl_ACS_VLINE() { return ACS_VLINE; }
int impl_ACS_PLUS() { return ACS_PLUS; }
int impl_ACS_S1() { return ACS_S1; }
int impl_ACS_S9() { return ACS_S9; }
int impl_ACS_DIAMOND() { return ACS_DIAMOND; }
int impl_ACS_CKBOARD() { return ACS_CKBOARD; }
int impl_ACS_DEGREE() { return ACS_DEGREE; }
int impl_ACS_PLMINUS() { return ACS_PLMINUS; }
int impl_ACS_BULLET() { return ACS_BULLET; }
int impl_ACS_LARROW() { return ACS_LARROW; }
int impl_ACS_RARROW() { return ACS_RARROW; }
int impl_ACS_DARROW() { return ACS_DARROW; }
int impl_ACS_UARROW() { return ACS_UARROW; }
int impl_ACS_BOARD() { return ACS_BOARD; }
int impl_ACS_LANTERN() { return ACS_LANTERN; }
int impl_ACS_BLOCK() { return ACS_BLOCK; }
int impl_ACS_S3() { return ACS_S3; }
int impl_ACS_S7() { return ACS_S7; }
int impl_ACS_LEQUAL() { return ACS_LEQUAL; }
int impl_ACS_GEQUAL() { return ACS_GEQUAL; }
int impl_ACS_PI() { return ACS_PI; }
int impl_ACS_NEQUAL() { return ACS_NEQUAL; }
int impl_ACS_STERLING() { return ACS_STERLING; }
int impl_ACS_BSSB() { return ACS_BSSB; }
int impl_ACS_SSBB() { return ACS_SSBB; }
int impl_ACS_BBSS() { return ACS_BBSS; }
int impl_ACS_SBBS() { return ACS_SBBS; }
int impl_ACS_SBSS() { return ACS_SBSS; }
int impl_ACS_SSSB() { return ACS_SSSB; }
int impl_ACS_SSBS() { return ACS_SSBS; }
int impl_ACS_BSSS() { return ACS_BSSS; }
int impl_ACS_BSBS() { return ACS_BSBS; }
int impl_ACS_SBSB() { return ACS_SBSB; }
int impl_ACS_SSSS() { return ACS_SSSS; }
