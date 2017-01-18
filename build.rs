extern crate gcc;
extern crate pkg_config;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    check_chtype_size();
}

fn check_chtype_size() {
    let out_dir = env::var("OUT_DIR").expect("cannot get OUT_DIR");
    let src = format!("{}", Path::new(&out_dir).join("chtype_size.c").display());
    let bin = format!("{}", Path::new(&out_dir).join("chtype_size").display());

    let mut fp = File::create(&src).expect(&format!("cannot create {}", src));
    fp.write_all(b"
#include <assert.h>
#include <limits.h>
#include <stdio.h>

#include <ncurses.h>

int main(void)
{
    if (sizeof(chtype)*CHAR_BIT == 64) {
        puts(\"cargo:rustc-cfg=feature=\\\"wide_chtype\\\"\");
    } else {
        /* We only support 32-bit and 64-bit chtype. */
        assert(sizeof(chtype)*CHAR_BIT == 32 && \"unsupported size for chtype\");
    }
    return 0;
}
    ").expect(&format!("cannot write into {}", src));

    let cfg = gcc::Config::new();
    let compiler = cfg.get_compiler();

    Command::new(compiler.path()).arg(&src).arg("-o").arg(&bin)
                                 .status().expect("compilation failed");
    let features = Command::new(&bin).output()
                   .expect(&format!("{} failed", bin));
    print!("{}", String::from_utf8_lossy(&features.stdout));

    let ncurses_names = ["ncurses5", "ncurses"];
    for ncurses_name in &ncurses_names {
        if pkg_config::probe_library(ncurses_name).is_ok() {
            break;
        }
    }

    std::fs::remove_file(&src).expect(&format!("cannot delete {}", src));
    std::fs::remove_file(&bin).expect(&format!("cannot delete {}", bin));
}
