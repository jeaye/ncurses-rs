extern crate cc;
extern crate pkg_config;

use pkg_config::Library;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn find_library(names: &[&str]) -> Option<Library> {
    for name in names {
        if let Ok(lib) = pkg_config::probe_library(name) {
            return Some(lib);
        }
    }
    None
}

fn main() {
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");

    let wide = cfg!(all(feature = "wide", not(target_os = "macos")));

    let ncurses_lib_names = if wide {
        &["ncursesw5", "ncursesw"]
    } else {
        &["ncurses5", "ncurses"]
    };
    let ncurses_lib = find_library(ncurses_lib_names);

    if cfg!(feature = "menu") {
        if wide {
            find_library(&["menuw5", "menuw"]);
        } else {
            find_library(&["menu5", "menu"]);
        }
    }

    if cfg!(feature = "panel") {
        if wide {
            find_library(&["panelw5", "panelw"]);
        } else {
            find_library(&["panel5", "panel"]);
        }
    }

    let mut already_printed:bool=false;
    const ENV_VAR_NAME_FOR_LIB:&str="NCURSES_RS_RUSTC_LINK_LIB";
    println!("cargo:rerun-if-env-changed={}",ENV_VAR_NAME_FOR_LIB);
    let lib_name:String=match std::env::var(ENV_VAR_NAME_FOR_LIB) {
        Ok(x) => x,
        _ => if let Some(ref lib)=ncurses_lib {
            // you can get something like this ["ncurses", "tinfo"] as the lib.libs vector
            // but we shouldn't assume "ncurses" is the first ie. lib.libs[0]
            // and the exact name of it can be ncurses,ncursesw,ncurses5,ncursesw5 ...
            // so find whichever it is and return that:
            let substring_to_find = "curses";
            if let Some(found) = lib.libs.iter().find(|&s| s.contains(substring_to_find)) {
                //If we're here, the pkg_config::probe_library() calls from above ie. through find_library()
                //have already printed these:
                //   cargo:rustc-link-lib=ncurses
                //   cargo:rustc-link-lib=tinfo
                //so there's no need to re-print the ncurses line
                already_printed=true;
                found.clone()
            } else {
                //if here, we should probably panic, but who knows it might still work even without pkg-config

                // Construct the repeated pkg-config command string
                let repeated_pkg_config_command: String = ncurses_lib_names
                    .iter()
                    .map(|ncurses_lib_name| format!("pkg-config --libs {}", ncurses_lib_name))
                    .collect::<Vec<_>>()
                    .join("` or `");

                // Construct the warning message string with the repeated pkg-config commands
                let warning_message = format!(
                    "pkg_config reported that it found the ncurses libs but the substring '{}' was not among them, ie. in the output of the shell command(s) eg. `{}`",
                    substring_to_find,
                    repeated_pkg_config_command
                    );

                // Print the warning message, but use old style warning with one ":" not two "::",
                // because old cargos(pre 23 Dec 2023) will simply ignore it and show no warning if it's "::"
                println!("cargo:warning={}", warning_message);
                //fallback lib name: 'ncurses' or 'ncursesw'
                //if this fails, there's the warning above to get an idea as to why.
                ncurses_lib_names.last().unwrap().to_string()
            }
        } else {
            //pkg-config didn't find the lib, fallback to 'ncurses' or 'ncursesw'
            ncurses_lib_names.last().unwrap().to_string()
        }
    };
    if !already_printed {
        println!("cargo:rustc-link-lib={}", lib_name);
    }

    const ENV_VAR_NAME_FOR_FLAGS:&str="NCURSES_RS_RUSTC_FLAGS";
    println!("cargo:rerun-if-env-changed={}",ENV_VAR_NAME_FOR_FLAGS);
    if let Ok(x) = std::env::var(ENV_VAR_NAME_FOR_FLAGS) {
        println!("cargo:rustc-flags={}", x);
    }

    check_chtype_size(&ncurses_lib);

    gen_constants(&ncurses_lib, &lib_name);
    gen_menu_constants(&ncurses_lib, &lib_name);
    build_wrap(&ncurses_lib);
}

fn build_wrap(ncurses_lib: &Option<Library>) {
    println!("cargo:rerun-if-changed=src/wrap.c");
    let mut build = cc::Build::new();
    if let Some(lib) = ncurses_lib {
        for path in lib.include_paths.iter() {
            build.include(path);
        }
    }
    build.file("src/wrap.c")
        .compile("wrap");
}

fn gen_constants(ncurses_lib: &Option<Library>, lib_name:&str) {
    println!("cargo:rerun-if-changed=src/genconstants.c");
    let out_dir = env::var("OUT_DIR").expect("cannot get OUT_DIR");
    let bin = format!("{}", Path::new(&out_dir).join(if cfg!(windows) { "genconstants.exe" } else { "genconstants" }).display());
    let src = format!("{}", Path::new(&out_dir).join("raw_constants.rs").display());

    let mut build = cc::Build::new();
    let link_paths_for_linker;
    if let Some(lib) = ncurses_lib {
        for path in lib.include_paths.iter() {
            build.include(path);
        }
        link_paths_for_linker=lib.link_paths.iter()
            .map(|link_path| format!("-L{}", link_path.display()))
            .collect::<Vec<String>>();
    } else {
        link_paths_for_linker=vec!();
    }
    let compiler = build.try_get_compiler().expect("Failed Build::try_get_compiler");
    let mut command = compiler.to_command();

    if let Ok(x) = std::env::var("NCURSES_RS_CFLAGS") {
        command.args(x.split(" "));
    }    

    command.arg("-o").arg(&bin).arg("src/genconstants.c")
        .arg(format!("-l{}",lib_name)).args(link_paths_for_linker);
    assert!(command.status().expect("compilation failed").success());

    let consts = Command::new(&bin).output()
        .expect(&format!("{} failed", bin));

    let mut file = File::create(&src).unwrap();
    
    file.write_all(&consts.stdout).unwrap();
}

//FIXME: dedup code in these 2 functions
fn gen_menu_constants(ncurses_lib: &Option<Library>, lib_name:&str) {
    println!("cargo:rerun-if-changed=src/menu/genconstants.c");
    let out_dir = env::var("OUT_DIR").expect("cannot get OUT_DIR");
    let bin = format!("{}", Path::new(&out_dir).join(if cfg!(windows) { "genmenuconstants.exe" } else { "genmenuconstants" }).display());
    let src = format!("{}", Path::new(&out_dir).join("menu_constants.rs").display());

    let mut build = cc::Build::new();
    let link_paths_for_linker;
    if let Some(lib) = ncurses_lib {
        for path in lib.include_paths.iter() {
            build.include(path);
        }
        link_paths_for_linker=lib.link_paths.iter()
            .map(|link_path| format!("-L{}", link_path.display()))
            .collect::<Vec<String>>();
    } else {
        link_paths_for_linker=vec!();
    }
    let compiler = build.try_get_compiler().expect("Failed Build::try_get_compiler");
    let mut command = compiler.to_command();

    if let Ok(x) = std::env::var("NCURSES_RS_CFLAGS") {
        command.args(x.split(" "));
    }    

    command.arg("-o").arg(&bin).arg("src/menu/genconstants.c")
        .arg(format!("-l{}",lib_name)).args(link_paths_for_linker);
    assert!(command.status().expect("compilation failed").success());

    let consts = Command::new(&bin).output()
        .expect(&format!("{} failed", bin));

    let mut file = File::create(&src).unwrap();
    
    file.write_all(&consts.stdout).unwrap();
}

fn check_chtype_size(ncurses_lib: &Option<Library>) {
    let out_dir = env::var("OUT_DIR").expect("cannot get OUT_DIR");
    let src = format!("{}", Path::new(&out_dir).join("chtype_size.c").display());
    let bin = format!("{}", Path::new(&out_dir).join(if cfg!(windows) { "chtype_size.exe" } else { "chtype_size" }).display());

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

#if defined(NCURSES_MOUSE_VERSION) && NCURSES_MOUSE_VERSION == 1
	puts(\"cargo:rustc-cfg=feature=\\\"mouse_v1\\\"\");
#endif
    return 0;
}
    ").expect(&format!("cannot write into {}", src));

    let mut build = cc::Build::new();
    if let Some(lib) = ncurses_lib {
        for path in lib.include_paths.iter() {
            build.include(path);
        }
    }
    let compiler = build.try_get_compiler().expect("Failed Build::try_get_compiler");
    let mut command = compiler.to_command();

    if let Ok(x) = std::env::var("NCURSES_RS_CFLAGS") {
        command.args(x.split(" "));
    }

    command.arg("-o").arg(&bin).arg(&src);
    assert!(command.status().expect("compilation failed").success());
    let features = Command::new(&bin).output()
        .expect(&format!("{} failed", bin));
    print!("{}", String::from_utf8_lossy(&features.stdout));

    std::fs::remove_file(&src).expect(&format!("cannot delete {}", src));
    std::fs::remove_file(&bin).expect(&format!("cannot delete {}", bin));
}
