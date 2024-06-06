#include <stdio.h> // for printf/fprintf but curses.h already includes this(everywhere?)
#include <curses.h>

#define PCONST(ty, NAME) printf("pub const " #NAME ": " #ty " = %lld;\n", (long long) (NAME))
#define PCONSTU(ty, NAME) printf("pub const " #NAME ": " #ty " = %llu;\n", (unsigned long long) (NAME))

//XXX: If NAME isn't defined via #define or anything in the ncurses.h file, instead of not defining it at all in our ncurses crate, we define it as being of a specific type(seen below as long snakecase text) which when used by any other crates(like the 'cursive' crate) will compile error in a helpful manner(the type itself is the error message and gives an idea on how to fix it)

// This long snakecase text below, is a type in rust which will be shown when a crate(like cursive) that uses our crate compiles on a system with old ncurses(like macos Mojave 10.14.6 with its native ncurses version instead of the homebrew ncurses version installed), whereby one or more macro definitions like A_ITALIC aren't defined in ncurses.h
#define ERROR_MSG_TYPE "Your_ncurses_installation_is_too_old_and_it_does_not_have_this_underlined_identifier_defined_in_its_header_file_therefore_the_ncurses_crate_did_not_include_it_because_it_tries_to_be_compatible_however_this_crate_in_which_you_see_the_error_it_needs_the_identifier__If_you_are_on_MacOS_then_use_the_brew_version_of_ncurses_and_set_PKG_CONFIG_PATH_to_the_pkgconfig_dir_from_within_it"
//using these two(alias,constructor) in .rs avoids duplication of the long-named type for every .c defined that we wanna handle
#define ERROR_MSG_TYPE_ALIAS "TypeAliasForErrorMsgType"
#define ERROR_MSG_TYPE_CONSTRUCTOR "ERROR_MSG_TYPE_CONSTRUCTOR"

//Same deal as before except this(error) is for defined in ncurses.h file that are expected to be missing in latest ncurses version installed in the system...
#define ERROR_MSG_TYPE_EXPECTED_MISS "The_ncurses_crate_did_not_define_this_underlined_identifier_because_your_ncurses_installation_is_up_to_date_and_expectedly_did_not_have_it__however_the_crate_where_you_see_this_error_requires_this_identifier__so_either_upgrade_or_fix_this_crate__or_downgrade_your_ncurses_installation_and_cargo_clean_before_retrying__or_alternatively_try_using_an_older_ncurses_crate_if_you_know_that_worked_before"
#define ERROR_MSG_EXPECTED_MISS_TYPE_ALIAS "TypeAliasForErrorMsgWhenExpectedMissType"
#define ERROR_MSG_EXPECTED_MISS_TYPE_CONSTRUCTOR "ERROR_MSG_EXPECTED_MISS_TYPE_CONSTRUCTOR"

//Don't forget to call this in main(), one time.
#define DEFINE_RS_TYPES \
  do {\
    printf("/// For MacOS: `brew install ncurses` then `export PKG_CONFIG_PATH=\"/usr/local/opt/ncurses/lib/pkgconfig\"`\n/// that will give you a compatible/newer ncurses installation.\n");\
    printf("#[derive(Debug)] // derive to avoid a warning\n\
#[allow(non_camel_case_types)] \
pub struct "ERROR_MSG_TYPE";\n\n\
type "ERROR_MSG_TYPE_ALIAS" = "ERROR_MSG_TYPE";\n\n\
#[allow(dead_code)]\n\
const "ERROR_MSG_TYPE_CONSTRUCTOR": "ERROR_MSG_TYPE_ALIAS" = "ERROR_MSG_TYPE";\n\n\n");\
printf("///Typically, the crate that errors with the following, should be patched to not use the identifier that's tagged with this type, because the latest ncurses installation doesn't have that identifier definition(anymore).\n");\
printf("#[derive(Debug)] // derive to avoid a warning\n\
#[allow(non_camel_case_types)] \
pub struct "ERROR_MSG_TYPE_EXPECTED_MISS";\n\n\
type "ERROR_MSG_EXPECTED_MISS_TYPE_ALIAS" = "ERROR_MSG_TYPE_EXPECTED_MISS";\n\n\
#[allow(dead_code)]\n\
const "ERROR_MSG_EXPECTED_MISS_TYPE_CONSTRUCTOR": "ERROR_MSG_EXPECTED_MISS_TYPE_ALIAS" = "ERROR_MSG_TYPE_EXPECTED_MISS";\n\n\
\n\n");\
  } while(0)

#define UNEXPECTED_MISS(NAME) \
  do {\
    fprintf(stderr,"Unexpected missing def: " #NAME "\n");\
    printf("pub const " #NAME ": " ERROR_MSG_TYPE_ALIAS " = " ERROR_MSG_TYPE_CONSTRUCTOR ";\n");\
  } while(0)

#define EXPECT_MISS(NAME) \
  do {\
    fprintf(stderr,"Missing def(but it's expected to be missing): " #NAME "\n");\
    printf("pub const " #NAME ": " ERROR_MSG_EXPECTED_MISS_TYPE_ALIAS " = " ERROR_MSG_EXPECTED_MISS_TYPE_CONSTRUCTOR ";\n");\
  } while(0)

//#warning "This warning is unseen during `cargo build` unless compilation fails somewhere at build.rs time"

int main(void) {
	printf("/* Commented out ncurses initialization chars: '");

	//fflush(stdout);fflush(stderr);*((int *)0) = 42; //segfault(on purpose for manual testing purposes from build.rs) here before terminal gets messed up needing a `reset` shell command to restore! and after something's printed to stdout.

	/* some values aren't set until after this is run */
	initscr();
	endwin();
	printf("' */\n");

  DEFINE_RS_TYPES;

	/* Success/Failure. */
	PCONST(i32, ERR);
	PCONST(i32, OK);
	PCONST(c_bool, TRUE);
	PCONST(c_bool, FALSE);

	/* Attributes. */
#ifdef NCURSES_ATTR_SHIFT
	PCONST(u32, NCURSES_ATTR_SHIFT);
#else
  UNEXPECTED_MISS(NCURSES_ATTR_SHIFT);
#endif

	/* Colors */
	PCONST(i16, COLOR_BLACK);
	PCONST(i16, COLOR_RED);
	PCONST(i16, COLOR_GREEN);
	PCONST(i16, COLOR_YELLOW);
	PCONST(i16, COLOR_BLUE);
	PCONST(i16, COLOR_MAGENTA);
	PCONST(i16, COLOR_CYAN);
	PCONST(i16, COLOR_WHITE);

	/* Values for the _flags member */
#ifdef _SUBWIN
	PCONST(i32, _SUBWIN);
#else
  UNEXPECTED_MISS(_SUBWIN);
#endif

#ifdef _ENDLINE
	PCONST(i32, _ENDLINE);
#else
  UNEXPECTED_MISS(_ENDLINE);
#endif

#ifdef _FULLWIN
	PCONST(i32, _FULLWIN);
#else
  UNEXPECTED_MISS(_FULLWIN);
#endif

#ifdef _SCROLLWIN
	PCONST(i32, _SCROLLWIN);
#else
  UNEXPECTED_MISS(_SCROLLWIN);
#endif

#ifdef _ISPAD
	PCONST(i32, _ISPAD);
#else
  UNEXPECTED_MISS(_ISPAD);
#endif

#ifdef _HASMOVED
	PCONST(i32, _HASMOVED);
#else
  UNEXPECTED_MISS(_HASMOVED);
#endif

#ifdef _WRAPPED
	PCONST(i32, _WRAPPED);
#else
  UNEXPECTED_MISS(_WRAPPED);
#endif

	/*
	 * This value is used in the firstchar and lastchar fields to mark
	 * unchanged lines
	 */
#ifdef _NOCHANGE
	PCONST(i32, _NOCHANGE);
#else
  UNEXPECTED_MISS(_NOCHANGE);
#endif

	/*
	 * This value is used in the oldindex field to mark lines created by insertions
	 * and scrolls.
	 */
#ifdef _NEWINDEX
	PCONST(i32, _NEWINDEX);
#else
  UNEXPECTED_MISS(_NEWINDEX);
#endif

	/* Keys */
	PCONST(i32, KEY_CODE_YES);
	PCONST(i32, KEY_MIN);
	PCONST(i32, KEY_BREAK);
	PCONST(i32, KEY_SRESET);
	PCONST(i32, KEY_RESET);
	PCONST(i32, KEY_DOWN);
	PCONST(i32, KEY_UP);
	PCONST(i32, KEY_LEFT);
	PCONST(i32, KEY_RIGHT);
	PCONST(i32, KEY_HOME);
	PCONST(i32, KEY_BACKSPACE);
	PCONST(i32, KEY_F0);
	PCONST(i32, KEY_DL);
	PCONST(i32, KEY_IL);
	PCONST(i32, KEY_DC);
	PCONST(i32, KEY_IC);
	PCONST(i32, KEY_EIC);
	PCONST(i32, KEY_CLEAR);
	PCONST(i32, KEY_EOS);
	PCONST(i32, KEY_EOL);
	PCONST(i32, KEY_SF);
	PCONST(i32, KEY_SR);
	PCONST(i32, KEY_NPAGE);
	PCONST(i32, KEY_PPAGE);
	PCONST(i32, KEY_STAB);
	PCONST(i32, KEY_CTAB);
	PCONST(i32, KEY_CATAB);
	PCONST(i32, KEY_ENTER);
	PCONST(i32, KEY_PRINT);
	PCONST(i32, KEY_LL);
#ifdef KEY_A1
	PCONST(i32, KEY_A1);
#else
  UNEXPECTED_MISS(KEY_A1);
#endif
#ifdef KEY_A3
	PCONST(i32, KEY_A3);
#else
  UNEXPECTED_MISS(KEY_A3);
#endif
#ifdef KEY_B2
	PCONST(i32, KEY_B2);
#else
  UNEXPECTED_MISS(KEY_B2);
#endif
#ifdef KEY_C1
	PCONST(i32, KEY_C1);
#else
  UNEXPECTED_MISS(KEY_C1);
#endif
#ifdef KEY_C3
	PCONST(i32, KEY_C3);
#else
  UNEXPECTED_MISS(KEY_C3);
#endif
	PCONST(i32, KEY_BTAB);
	PCONST(i32, KEY_BEG);
	PCONST(i32, KEY_CANCEL);
	PCONST(i32, KEY_CLOSE);
	PCONST(i32, KEY_COMMAND);
	PCONST(i32, KEY_COPY);
	PCONST(i32, KEY_CREATE);
	PCONST(i32, KEY_END);
	PCONST(i32, KEY_EXIT);
	PCONST(i32, KEY_FIND);
	PCONST(i32, KEY_HELP);
	PCONST(i32, KEY_MARK);
	PCONST(i32, KEY_MESSAGE);
	PCONST(i32, KEY_MOVE);
	PCONST(i32, KEY_NEXT);
	PCONST(i32, KEY_OPEN);
	PCONST(i32, KEY_OPTIONS);
	PCONST(i32, KEY_PREVIOUS);
	PCONST(i32, KEY_REDO);
	PCONST(i32, KEY_REFERENCE);
	PCONST(i32, KEY_REFRESH);
	PCONST(i32, KEY_REPLACE);
	PCONST(i32, KEY_RESTART);
	PCONST(i32, KEY_RESUME);
	PCONST(i32, KEY_SAVE);
	PCONST(i32, KEY_SBEG);
	PCONST(i32, KEY_SCANCEL);
	PCONST(i32, KEY_SCOMMAND);
	PCONST(i32, KEY_SCOPY);
	PCONST(i32, KEY_SCREATE);
	PCONST(i32, KEY_SDC);
	PCONST(i32, KEY_SDL);
	PCONST(i32, KEY_SELECT);
	PCONST(i32, KEY_SEND);
	PCONST(i32, KEY_SEOL);
	PCONST(i32, KEY_SEXIT);
	PCONST(i32, KEY_SFIND);
	PCONST(i32, KEY_SHELP);
	PCONST(i32, KEY_SHOME);
	PCONST(i32, KEY_SIC);
	PCONST(i32, KEY_SLEFT);
	PCONST(i32, KEY_SMESSAGE);
	PCONST(i32, KEY_SMOVE);
	PCONST(i32, KEY_SNEXT);
	PCONST(i32, KEY_SOPTIONS);
	PCONST(i32, KEY_SPREVIOUS);
	PCONST(i32, KEY_SPRINT);
	PCONST(i32, KEY_SREDO);
	PCONST(i32, KEY_SREPLACE);
	PCONST(i32, KEY_SRIGHT);
	PCONST(i32, KEY_SRSUME);
	PCONST(i32, KEY_SSAVE);
	PCONST(i32, KEY_SSUSPEND);
	PCONST(i32, KEY_SUNDO);
	PCONST(i32, KEY_SUSPEND);
	PCONST(i32, KEY_UNDO);
	PCONST(i32, KEY_MOUSE);
	PCONST(i32, KEY_RESIZE);

//TODO: make this 5 line block a 1 line(macro-like) and have rust preprocess it into the 5 lines, so that eg. KEY_EVENT is used only once to avoid duplication and typo or copy/paste mistakes when repeating it.
#ifdef KEY_EVENT
	PCONST(i32, KEY_EVENT);
#else
  EXPECT_MISS(KEY_EVENT);
#endif
	PCONST(i32, KEY_MAX);

#ifdef NCURSES_MOUSE_VERSION
	PCONST(i32, NCURSES_MOUSE_VERSION);
#else
  UNEXPECTED_MISS(NCURSES_MOUSE_VERSION);
#endif

#ifdef MASK_SHIFT
	PCONST(i32, MASK_SHIFT);
#else
  EXPECT_MISS(MASK_SHIFT);
#endif

#ifdef MODIFIER_SHIFT
	PCONST(i32, MODIFIER_SHIFT);
#else
  EXPECT_MISS(MODIFIER_SHIFT);
#endif

	/* Mouse Support */
#ifdef NCURSES_BUTTON_RELEASED
	PCONST(i32, NCURSES_BUTTON_RELEASED);
#else
  UNEXPECTED_MISS(NCURSES_BUTTON_RELEASED);
#endif

#ifdef NCURSES_BUTTON_PRESSED
	PCONST(i32, NCURSES_BUTTON_PRESSED);
#else
  UNEXPECTED_MISS(NCURSES_BUTTON_PRESSED);
#endif

#ifdef NCURSES_BUTTON_CLICKED
	PCONST(i32, NCURSES_BUTTON_CLICKED);
#else
  UNEXPECTED_MISS(NCURSES_BUTTON_CLICKED);
#endif

#ifdef NCURSES_DOUBLE_CLICKED
	PCONST(i32, NCURSES_DOUBLE_CLICKED);
#else
  UNEXPECTED_MISS(NCURSES_DOUBLE_CLICKED);
#endif

#ifdef NCURSES_TRIPLE_CLICKED
	PCONST(i32, NCURSES_TRIPLE_CLICKED);
#else
  UNEXPECTED_MISS(NCURSES_TRIPLE_CLICKED);
#endif

#ifdef NCURSES_RESERVED_EVENT
	PCONST(i32, NCURSES_RESERVED_EVENT);
#else
  UNEXPECTED_MISS(NCURSES_RESERVED_EVENT);
#endif

	/* event masks */
	PCONST(i32, BUTTON1_RELEASED);
	PCONST(i32, BUTTON1_PRESSED);
	PCONST(i32, BUTTON1_CLICKED);
	PCONST(i32, BUTTON1_DOUBLE_CLICKED);
	PCONST(i32, BUTTON1_TRIPLE_CLICKED);

	PCONST(i32, BUTTON2_RELEASED);
	PCONST(i32, BUTTON2_PRESSED);
	PCONST(i32, BUTTON2_CLICKED);
	PCONST(i32, BUTTON2_DOUBLE_CLICKED);
	PCONST(i32, BUTTON2_TRIPLE_CLICKED);

	PCONST(i32, BUTTON3_RELEASED);
	PCONST(i32, BUTTON3_PRESSED);
	PCONST(i32, BUTTON3_CLICKED);
	PCONST(i32, BUTTON3_DOUBLE_CLICKED);
	PCONST(i32, BUTTON3_TRIPLE_CLICKED);

	PCONST(i32, BUTTON4_RELEASED);
	PCONST(i32, BUTTON4_PRESSED);
	PCONST(i32, BUTTON4_CLICKED);
	PCONST(i32, BUTTON4_DOUBLE_CLICKED);
	PCONST(i32, BUTTON4_TRIPLE_CLICKED);

#ifdef BUTTON5_RELEASED
	PCONST(i32, BUTTON5_RELEASED);
#else
  UNEXPECTED_MISS(BUTTON5_RELEASED);
#endif

#ifdef BUTTON5_PRESSED
	PCONST(i32, BUTTON5_PRESSED);
#else
  UNEXPECTED_MISS(BUTTON5_PRESSED);
#endif

#ifdef BUTTON5_CLICKED
	PCONST(i32, BUTTON5_CLICKED);
#else
  UNEXPECTED_MISS(BUTTON5_CLICKED);
#endif

#ifdef BUTTON5_DOUBLE_CLICKED
	PCONST(i32, BUTTON5_DOUBLE_CLICKED);
#else
  UNEXPECTED_MISS(BUTTON5_DOUBLE_CLICKED);
#endif

#ifdef BUTTON5_TRIPLE_CLICKED
	PCONST(i32, BUTTON5_TRIPLE_CLICKED);
#else
  UNEXPECTED_MISS(BUTTON5_TRIPLE_CLICKED);
#endif

	PCONST(i32, BUTTON_CTRL);
	PCONST(i32, BUTTON_SHIFT);
	PCONST(i32, BUTTON_ALT);
	PCONST(i32, REPORT_MOUSE_POSITION);

	PCONST(i32, ALL_MOUSE_EVENTS);

	/* Attributes */
	PCONSTU(crate::ll::chtype, A_NORMAL);
	PCONSTU(crate::ll::chtype, A_STANDOUT);
	PCONSTU(crate::ll::chtype, A_UNDERLINE);
#ifdef A_ITALIC
	PCONSTU(crate::ll::chtype, A_ITALIC);
#else
  UNEXPECTED_MISS(A_ITALIC);
#endif
	PCONSTU(crate::ll::chtype, A_REVERSE);
	PCONSTU(crate::ll::chtype, A_BLINK);
	PCONSTU(crate::ll::chtype, A_DIM);
	PCONSTU(crate::ll::chtype, A_BOLD);

#ifdef A_BLANK
	PCONSTU(crate::ll::chtype, A_BLANK);
#else
  EXPECT_MISS(A_BLANK);
#endif

	PCONSTU(crate::ll::chtype, A_INVIS);
	PCONSTU(crate::ll::chtype, A_PROTECT);
	PCONSTU(crate::ll::chtype, A_ALTCHARSET);
	PCONSTU(crate::ll::chtype, A_ATTRIBUTES);
	PCONSTU(crate::ll::chtype, A_CHARTEXT);
	PCONSTU(crate::ll::chtype, A_COLOR);

	//do last, flush just to be sure!
	fflush(stdout);fflush(stderr);
}
