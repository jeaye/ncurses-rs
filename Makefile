# Copyright © 2013 Free Software Foundation, Inc
# See licensing in LICENSE file
#
# File: Makefile
# Author: Jesse 'Jeaye' Wilkerson
# Description:
# 	Builds ncurses-rs

# Sources
LIB_SRC=$(shell find src -type f -name '*.rs')
EXAMPLES_SRC=$(shell find examples -type f -name '*.rs')

# Colors
COLOR_OFF=$(shell tput sgr0)
COLOR_GREEN=$(shell tput setaf 2)
PREFIX=${COLOR_GREEN}»»»${COLOR_OFF}

.SILENT:

.PHONY: all clean

all: .build_examples
	printf "${PREFIX} Finished \o/\n"
	
.build_lib: .setup_lib ${LIB_SRC}
	printf "${PREFIX} Building ncurses-rs "
	rustc --out-dir lib src/lib.rs
	printf "... success\n"
	touch .build_lib

.setup_lib:
	mkdir -p lib
	touch .setup_lib

.build_examples: .build_lib .setup_examples ${EXAMPLES_SRC}
	printf "${PREFIX} Building examples "
	$(foreach file, ${EXAMPLES_SRC}, rustc --out-dir bin -Llib $(file);)
	printf "... success\n"
	touch .build_examples

.setup_examples:
	mkdir -p bin
	touch .setup_examples

clean:
	find . -type f -name '.build_*' | xargs rm -f
	printf "${PREFIX} Cleaned\n"

