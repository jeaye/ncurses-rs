ncurses-rs [![Build Status](https://travis-ci.org/jeaye/ncurses-rs.png)](https://travis-ci.org/jeaye/ncurses-rs)
==========

This is a *very thin* wrapper around the ncurses TUI lib.

**NOTE:** The ncurses lib is terribly unsafe and ncurses-rs is only the lightest
wrapper it can be. If you want a safe and idiomatic TUI library for Rust, look
elsewhere. If you want a 1:1 port of C to Rust or you want to crank a TUI out
C-style in Rust, this will probably do the trick.

## Building

The compiled library will go to the `target` directory.

```
cargo build
```

Note that you must to have the ncurses library installed and linkable for ncurses-rs to work. On Linux, this should be trivial. On OS X, consider installing ncurses using Homebrew. (Note that you have to force Homebrew to link the library to `/usr/local/lib`: `brew link --force ncurses` and set that path to
`LIBRARY_PATH` environmental variable.)

## Examples

Examples are built by `cargo build`. To run them, use `cargo run --example ex_<NUMBER>`. Example numbers increase along with the complexity of the example.

Current examples:  
**1.** [Hello World](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_1.rs)  
**2.** [Basic Input & Attributes](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_2.rs)  
**3.** [Simple Pager](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_3.rs)  
**4.** [Window Movement](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_4.rs)  
**5.** [Menu Library](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_5.rs) (requires rust nightly)  
**6.** [Pager & Syntax Highlighting](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_6.rs)  
**7.** [Basic Input & Attributes (Unicode)](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_7.rs)  
**8.** [Special ACS Characters](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_8.rs)  

## Environment variables

Some environment variables are read by `build.rs`:

If set, `NCURSES_RS_RUSTC_LINK_LIB` will be used for `NCURSES_RS_RUSTC_LINK_LIB`.

If set, `NCURSES_RS_RUSTC_FLAGS` will be used for `cargo:rustc-flags`.

If set, `NCURSES_RS_CFLAGS` will be used for the compilation of the test program `chtype_size.c`.
